// cargo run --bin ingest --features ssr -- content/classical-mechanics
// cargo run --bin ingest --features ssr -- content/classical-mechanics/kinematics
// cargo run --bin ingest --features ssr -- content/classical-mechanics/kinematics --dry-run

use clap::Parser;
use db::create_pool;
use db::probe_repo;
use domain::content_spec::{validate_node, BloomLevel, ParsedNode};
use server::content_fs::{discover_node_dirs, load_probe, parse_node_dir, read_phase_content};
use sqlx::PgPool;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Parser)]
#[command(
    name = "ingest",
    about = "Ingest content directories into the database"
)]
struct Cli {
    /// Content directories to ingest (node dirs or parent dirs).
    /// If a directory contains node.yaml, it is treated as a single node dir.
    /// Otherwise, immediate children are scanned for dirs containing node.yaml.
    paths: Vec<String>,

    /// Validate only — do not write to the database.
    #[arg(long)]
    dry_run: bool,
}

fn bloom_to_str(b: &BloomLevel) -> &'static str {
    match b {
        BloomLevel::Remember => "remember",
        BloomLevel::Understand => "understand",
        BloomLevel::Apply => "apply",
        BloomLevel::Analyze => "analyze",
        BloomLevel::Evaluate => "evaluate",
        BloomLevel::Create => "create",
    }
}

/// Infer the branch name from the directory path.
/// Expects a structure like content/<branch>/<node>/, returns <branch>.
/// Falls back to "unknown" if the structure doesn't match.
fn infer_branch(dir: &Path) -> String {
    dir.parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string()
}

/// Upsert a single node directory into the database in its own transaction.
async fn ingest_node_dir(pool: &PgPool, dir: &Path) -> Result<String, String> {
    let parsed: ParsedNode = parse_node_dir(dir)?;

    let errors = validate_node(&parsed);
    if !errors.is_empty() {
        let messages: Vec<String> = errors.iter().map(|e| format!("    {e}")).collect();
        return Err(messages.join("\n"));
    }

    let slug = parsed.meta.concept_id.clone();
    let branch = infer_branch(dir);
    let meta = &parsed.meta;

    // Begin per-node transaction (D-06)
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("    transaction begin: {e}"))?;

    // Upsert nodes row (D-07, D-10)
    let node_id: uuid::Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO nodes (slug, title, node_type, branch, depth_tier,
                           eqf_level, bloom_minimum, estimated_minutes,
                           derivation_required, misconceptions,
                           domain_of_applicability, esco_tags)
        VALUES ($1, $2, $3::node_type, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        ON CONFLICT (slug) DO UPDATE SET
            title = EXCLUDED.title,
            eqf_level = EXCLUDED.eqf_level,
            bloom_minimum = EXCLUDED.bloom_minimum,
            estimated_minutes = EXCLUDED.estimated_minutes,
            derivation_required = EXCLUDED.derivation_required,
            misconceptions = EXCLUDED.misconceptions,
            domain_of_applicability = EXCLUDED.domain_of_applicability,
            esco_tags = EXCLUDED.esco_tags,
            updated_at = NOW()
        RETURNING id
        "#,
    )
    .bind(&slug)
    .bind(&meta.title)
    .bind(&meta.node_type)
    .bind(&branch)
    .bind(&meta.depth_tier)
    .bind(meta.eqf_level as i16)
    .bind(bloom_to_str(&meta.bloom_minimum))
    .bind(meta.estimated_minutes as i16)
    .bind(meta.derivation_required)
    // Typed misconceptions are flattened to their statement text: the
    // nodes.misconceptions column is TEXT[]. Persisting the graduate type tag
    // needs an additive migration — see the M2 report.
    .bind(meta.misconception_statements())
    .bind(meta.domain_of_applicability.as_slice())
    .bind(meta.esco_tags.as_slice())
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| format!("    nodes upsert: {e}"))?;

    // Upsert node_phases rows (D-08).
    //
    // `estimated_minutes` per phase has been parsed and validated since v1.1
    // (check 14) and dropped on the floor ever since; M13 gives it a column, so
    // the pace dashboard can compare per phase rather than only per node.
    for phase in &meta.phases {
        let content_body = read_phase_content(dir, phase.number).unwrap_or_default();
        let phase_minutes = parsed
            .phase_estimated_minutes
            .get(&phase.number)
            .map(|m| *m as i16);

        sqlx::query(
            r#"
            INSERT INTO node_phases (node_id, phase_number, phase_type, content_body, estimated_minutes)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (node_id, phase_number) DO UPDATE SET
                phase_type = EXCLUDED.phase_type,
                content_body = EXCLUDED.content_body,
                estimated_minutes = EXCLUDED.estimated_minutes,
                updated_at = NOW()
            "#,
        )
        .bind(node_id)
        .bind(phase.number as i16)
        .bind(phase.phase_type.name())
        .bind(&content_body)
        .bind(phase_minutes)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("    node_phases upsert phase {}: {e}", phase.number))?;
    }

    // Upsert the probe sidecar (content-spec v1.4). Removing a probe.yaml from a
    // node directory removes the stored spec too, so a re-ingest cannot leave a
    // routing rule live that the content no longer declares.
    let probe = load_probe(dir)?;
    let mut probe_note = "";
    match probe {
        Some(loaded) => {
            let spec_json = serde_json::to_value(&loaded.spec)
                .map_err(|e| format!("    probe.yaml serialize: {e}"))?;
            probe_repo::upsert_probe(
                &mut tx,
                node_id,
                &loaded.spec,
                &spec_json,
                &loaded.digest,
                meta.effective_relaxation(),
            )
            .await
            .map_err(|e| format!("    node_probes upsert: {e}"))?;
            probe_note = " (+probe)";
        }
        None => {
            probe_repo::delete_probe(&mut tx, node_id)
                .await
                .map_err(|e| format!("    node_probes delete: {e}"))?;
        }
    }

    tx.commit()
        .await
        .map_err(|e| format!("    transaction commit: {e}"))?;

    Ok(format!("  {slug:<36} OK{probe_note}"))
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.paths.is_empty() {
        eprintln!("Usage: ingest <path> [<path>...] [--dry-run]");
        process::exit(1);
    }

    // Discover all node directories from path arguments
    let mut node_dirs: Vec<PathBuf> = Vec::new();
    for path in &cli.paths {
        let found = discover_node_dirs(path);
        if found.is_empty() {
            eprintln!("Warning: no node directories found under '{path}'");
        }
        node_dirs.extend(found);
    }

    if node_dirs.is_empty() {
        eprintln!("Error: no node directories found in any of the provided paths.");
        process::exit(1);
    }

    // Create DB pool (skip if dry-run to avoid requiring DB connection during validate-only runs)
    let pool_opt: Option<PgPool> = if cli.dry_run {
        None
    } else {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            eprintln!("Error: DATABASE_URL must be set (or use --dry-run)");
            process::exit(1);
        });
        match create_pool(&database_url).await {
            Ok(p) => Some(p),
            Err(e) => {
                eprintln!("Error: failed to connect to database: {e}");
                process::exit(1);
            }
        }
    };

    let total = node_dirs.len();
    let mut failed = 0usize;

    for dir in &node_dirs {
        let pool_ref = pool_opt.as_ref();
        let result = if let Some(pool) = pool_ref {
            ingest_node_dir(pool, dir).await
        } else {
            // dry_run: parse+validate only, no DB
            ingest_node_dir_dry(dir)
        };

        match result {
            Ok(msg) => println!("{msg}"),
            Err(msg) => {
                let slug = dir
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                eprintln!("  {slug:<36} FAIL\n{msg}");
                failed += 1;
            }
        }
    }

    let succeeded = total - failed;
    println!();
    if failed > 0 {
        if cli.dry_run {
            println!("Validated: {succeeded}/{total} nodes   ({failed} failed)  (no database changes made)");
        } else {
            println!("Ingested: {succeeded}/{total} nodes   ({failed} failed)");
        }
        process::exit(1);
    } else if cli.dry_run {
        println!("Validated: {succeeded}/{total} nodes   (no database changes made)");
    } else {
        println!("Ingested: {succeeded}/{total} nodes");
    }
}

/// Dry-run path: parse and validate only, no DB writes.
fn ingest_node_dir_dry(dir: &Path) -> Result<String, String> {
    let parsed = parse_node_dir(dir)?;
    let errors = validate_node(&parsed);
    if !errors.is_empty() {
        let messages: Vec<String> = errors.iter().map(|e| format!("    {e}")).collect();
        return Err(messages.join("\n"));
    }
    let slug = &parsed.meta.concept_id;
    let probe_note = if parsed.probe.is_some() {
        " (+probe)"
    } else {
        ""
    };
    Ok(format!("  {slug:<36} OK{probe_note} (dry run)"))
}
