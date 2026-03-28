// cargo run --bin ingest --features ssr -- content/classical-mechanics
// cargo run --bin ingest --features ssr -- content/classical-mechanics/kinematics
// cargo run --bin ingest --features ssr -- content/classical-mechanics/kinematics --dry-run

use clap::Parser;
use db::create_pool;
use domain::content_spec::{
    extract_h2_headings, validate_node, BloomLevel, NodeMeta, ParsedNode,
};
use sqlx::PgPool;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Parser)]
#[command(name = "ingest", about = "Ingest content directories into the database")]
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

/// Discover node directories from a path argument.
/// - If path/node.yaml exists: return [path] as a single-node list.
/// - Otherwise: scan immediate children for dirs containing node.yaml.
fn discover_node_dirs(path: &str) -> Vec<PathBuf> {
    let base = PathBuf::from(path);

    if base.join("node.yaml").exists() {
        return vec![base];
    }

    let mut dirs = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&base) {
        for entry in entries.flatten() {
            let child = entry.path();
            if child.is_dir() && child.join("node.yaml").exists() {
                dirs.push(child);
            }
        }
    }
    dirs.sort(); // deterministic order
    dirs
}

/// Parse a node directory into a ParsedNode — mirrors the validate.rs pattern.
fn parse_node_dir(dir: &Path) -> Result<ParsedNode, String> {
    // Step 1: Read node.yaml
    let yaml_path = dir.join("node.yaml");
    let yaml_str = std::fs::read_to_string(&yaml_path)
        .map_err(|_| format!("node.yaml: file not found at {}", yaml_path.display()))?;

    // Step 2: Parse node.yaml with serde_saphyr
    let meta: NodeMeta = serde_saphyr::from_str(&yaml_str)
        .map_err(|e| format!("node.yaml:parse  {e}"))?;

    // Step 3: Read each phase file
    let mut phase_files_found: Vec<u8> = Vec::new();
    let mut phase_headings: HashMap<u8, Vec<String>> = HashMap::new();

    for n in 0u8..=6 {
        let phase_path = dir.join(format!("phase-{n}.md"));
        if let Ok(content) = std::fs::read_to_string(&phase_path) {
            phase_files_found.push(n);

            let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
            let body = matter
                .parse::<serde_json::Value>(&content)
                .map(|p| p.content)
                .unwrap_or(content);

            let headings = extract_h2_headings(&body);
            phase_headings.insert(n, headings);
        }
    }

    Ok(ParsedNode { meta, phase_files_found, phase_headings })
}

/// Read the raw content (including frontmatter) of a phase file.
fn read_phase_content(dir: &Path, phase_number: u8) -> Option<String> {
    let path = dir.join(format!("phase-{phase_number}.md"));
    std::fs::read_to_string(path).ok()
}

/// Infer the branch name from the directory path.
/// Expects a structure like content/<branch>/<node>/, returns <branch>.
/// Falls back to "unknown" if the structure doesn't match.
fn infer_branch(dir: &Path) -> String {
    // Walk up: dir = .../content/classical-mechanics/kinematics
    // parent = .../content/classical-mechanics
    // parent.file_name() = "classical-mechanics"
    dir.parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string()
}

/// Upsert a single node directory into the database in its own transaction.
async fn ingest_node_dir(
    pool: &PgPool,
    dir: &Path,
    dry_run: bool,
) -> Result<String, String> {
    // Parse and validate
    let parsed = parse_node_dir(dir)?;

    let errors = validate_node(&parsed);
    if !errors.is_empty() {
        let messages: Vec<String> = errors.iter().map(|e| format!("    {e}")).collect();
        return Err(messages.join("\n"));
    }

    let slug = parsed.meta.concept_id.clone();

    if dry_run {
        return Ok(format!("  {slug:<36} OK (dry run)"));
    }

    let branch = infer_branch(dir);
    let meta = &parsed.meta;

    // Begin per-node transaction (D-06)
    let mut tx = pool.begin().await.map_err(|e| format!("    transaction begin: {e}"))?;

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
    .bind(meta.misconceptions.as_slice())
    .bind(meta.domain_of_applicability.as_slice())
    .bind(meta.esco_tags.as_slice())
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| format!("    nodes upsert: {e}"))?;

    // Upsert node_phases rows (D-08)
    for phase in &meta.phases {
        let content_body = read_phase_content(dir, phase.number).unwrap_or_default();

        sqlx::query(
            r#"
            INSERT INTO node_phases (node_id, phase_number, phase_type, content_body)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (node_id, phase_number) DO UPDATE SET
                phase_type = EXCLUDED.phase_type,
                content_body = EXCLUDED.content_body,
                updated_at = NOW()
            "#,
        )
        .bind(node_id)
        .bind(phase.number as i16)
        .bind(phase.phase_type.name())
        .bind(&content_body)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("    node_phases upsert phase {}: {e}", phase.number))?;
    }

    tx.commit().await.map_err(|e| format!("    transaction commit: {e}"))?;

    Ok(format!("  {slug:<36} OK"))
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
            ingest_node_dir(pool, dir, false).await
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
    Ok(format!("  {slug:<36} OK (dry run)"))
}
