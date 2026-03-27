// cargo run --bin validate --features ssr -- content/classical-mechanics/kinematics
// cargo run --bin validate --features ssr -- content/classical-mechanics/kinematics --json

use domain::content_spec::{extract_h2_headings, validate_node, NodeMeta, ParsedNode};
use std::collections::HashMap;
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

    // Step 1: Read node.yaml
    let yaml_path = format!("{dir}/node.yaml");
    let yaml_str = match std::fs::read_to_string(&yaml_path) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("node.yaml: file not found at {yaml_path}");
            process::exit(1);
        }
    };

    // Step 2: Parse node.yaml with serde-saphyr
    let meta: NodeMeta = match serde_saphyr::from_str(&yaml_str) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("node.yaml:parse  {e}");
            process::exit(1);
        }
    };

    // Step 3: Read each phase file
    let mut phase_files_found: Vec<u8> = Vec::new();
    let mut phase_headings: HashMap<u8, Vec<String>> = HashMap::new();

    for n in 0u8..=6 {
        let phase_path = format!("{dir}/phase-{n}.md");
        if let Ok(content) = std::fs::read_to_string(&phase_path) {
            phase_files_found.push(n);

            // Use gray_matter to split frontmatter from body.
            // We use serde_json::Value for the frontmatter type — we only need the Markdown body.
            let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
            // On parse error (malformed frontmatter), fall back to treating full file as body.
            let body = matter
                .parse::<serde_json::Value>(&content)
                .map(|p| p.content)
                .unwrap_or(content);

            // Extract H2 headings from the Markdown body
            let headings = extract_h2_headings(&body);
            phase_headings.insert(n, headings);
        }
    }

    // Step 4: Build ParsedNode and validate
    let parsed_node = ParsedNode { meta, phase_files_found, phase_headings };
    let errors = validate_node(&parsed_node);

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
