// cargo run --bin validate --features ssr -- content/classical-mechanics/kinematics
// cargo run --bin validate --features ssr -- content/classical-mechanics/kinematics --json

use domain::content_spec::{validate_node, validate_node_warnings};
use server::content_fs::parse_node_dir;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Parse CLI args
    let dir = args
        .iter()
        .skip(1)
        .find(|a| !a.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("Usage: validate <node_dir> [--json]");
            process::exit(1);
        });

    let json_output = args.iter().any(|a| a == "--json");

    // Steps 1–3: read node.yaml, every phase-N.md, and the optional probe.yaml
    // sidecar. Shared with the ingest binary so the two cannot drift.
    let parsed_node = match parse_node_dir(Path::new(&dir)) {
        Ok(node) => node,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    // Step 4: validate
    let errors = validate_node(&parsed_node);

    // Warnings never affect the exit code (v1.3). Printed before the errors so
    // they are not lost at the bottom of a long failure list.
    let warnings = validate_node_warnings(&parsed_node);
    if !warnings.is_empty() {
        if json_output {
            // stdout stays the errors array; warnings go to stderr so a caller
            // piping `--json` into a parser is unaffected.
            match serde_json::to_string_pretty(&warnings) {
                Ok(json) => eprintln!("{json}"),
                Err(e) => eprintln!("Failed to serialize warnings: {e}"),
            }
        } else {
            for warning in &warnings {
                eprintln!("warning: {warning}");
            }
        }
    }

    if errors.is_empty() {
        println!("OK: {dir} is valid");
        process::exit(0);
    } else {
        if json_output {
            match serde_json::to_string_pretty(&errors) {
                Ok(json) => println!("{json}"),
                Err(e) => eprintln!("Failed to serialize errors: {e}"),
            }
        } else {
            for error in &errors {
                eprintln!("{error}");
            }
        }
        process::exit(1);
    }
}
