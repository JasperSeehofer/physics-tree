//! Reading a content node directory off disk.
//!
//! `validate` and `ingest` both parse the same tree the same way, and until v1.4
//! each carried its own copy of the loop. The sidecar `probe.yaml` (content-spec
//! v1.4) would have made that two copies of a second parse and two copies of the
//! digest, so the shared half lives here instead.
//!
//! This module is the only place in the workspace that does content I/O; the
//! validation itself stays pure in `domain::content_spec`.

use domain::content_spec::{extract_h2_headings, NodeMeta, ParsedNode};
use domain::probe::ProbeSpec;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A node directory's `probe.yaml`, parsed and fingerprinted.
pub struct LoadedProbe {
    pub spec: ProbeSpec,
    /// sha256 of the file as it sits on disk. Pins a sitting to the exact
    /// revision of the routing rules it was judged under.
    pub digest: String,
}

/// Discover node directories from a path argument.
///
/// - If `path/node.yaml` exists: return `[path]` as a single-node list.
/// - Otherwise: scan immediate children for dirs containing `node.yaml`.
pub fn discover_node_dirs(path: &str) -> Vec<PathBuf> {
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

/// Every `concept_id` reachable from a node directory's content root.
///
/// Used by validation check 21 to resolve an `internal` route target. The root
/// is inferred as the node directory's grandparent (`content/{branch}/{slug}`),
/// and every `content/*/*/node.yaml` under it is read for its `concept_id`.
/// A tree that does not match the layout yields an empty list, which check 21
/// reads as "not supplied" and skips.
pub fn discover_concept_ids(node_dir: &Path) -> Vec<String> {
    let Some(content_root) = node_dir.parent().and_then(|p| p.parent()) else {
        return Vec::new();
    };

    let mut ids: Vec<String> = Vec::new();
    let Ok(branches) = std::fs::read_dir(content_root) else {
        return ids;
    };
    for branch in branches.flatten() {
        let branch_path = branch.path();
        if !branch_path.is_dir() {
            continue;
        }
        let Ok(nodes) = std::fs::read_dir(&branch_path) else {
            continue;
        };
        for node in nodes.flatten() {
            let yaml = node.path().join("node.yaml");
            if !yaml.exists() {
                continue;
            }
            // A `concept_id` read is cheap and a full NodeMeta parse is not, but
            // parsing is what guarantees the id is the one the validator sees.
            if let Ok(text) = std::fs::read_to_string(&yaml) {
                if let Ok(meta) = serde_saphyr::from_str::<NodeMeta>(&text) {
                    if !ids.contains(&meta.concept_id) {
                        ids.push(meta.concept_id);
                    }
                }
            }
        }
    }
    ids.sort();
    ids
}

/// Read and parse `probe.yaml`, if the node has one.
///
/// A missing file is `Ok(None)` — the pre-v1.4 shape, and the shape of every
/// school and undergraduate node forever. A *malformed* file is an error: the
/// sidecar is `deny_unknown_fields` by choice, so a typo in a routing rule fails
/// the node loudly instead of dropping a rule in silence.
pub fn load_probe(dir: &Path) -> Result<Option<LoadedProbe>, String> {
    let path = dir.join("probe.yaml");
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Ok(None);
    };

    let spec: ProbeSpec =
        serde_saphyr::from_str(&text).map_err(|e| format!("probe.yaml:parse  {e}"))?;

    Ok(Some(LoadedProbe {
        digest: sha256_hex(&text),
        spec,
    }))
}

/// Read the raw content (including frontmatter) of a phase file.
pub fn read_phase_content(dir: &Path, phase_number: u8) -> Option<String> {
    std::fs::read_to_string(dir.join(format!("phase-{phase_number}.md"))).ok()
}

/// Parse a node directory into the pure `ParsedNode` the validator consumes.
pub fn parse_node_dir(dir: &Path) -> Result<ParsedNode, String> {
    let yaml_path = dir.join("node.yaml");
    let yaml_str = std::fs::read_to_string(&yaml_path)
        .map_err(|_| format!("node.yaml: file not found at {}", yaml_path.display()))?;

    let meta: NodeMeta =
        serde_saphyr::from_str(&yaml_str).map_err(|e| format!("node.yaml:parse  {e}"))?;

    let mut phase_files_found: Vec<u8> = Vec::new();
    let mut phase_headings: HashMap<u8, Vec<String>> = HashMap::new();
    let mut phase_estimated_minutes: HashMap<u8, u16> = HashMap::new();

    for n in 0u8..=6 {
        let phase_path = dir.join(format!("phase-{n}.md"));
        if let Ok(content) = std::fs::read_to_string(&phase_path) {
            phase_files_found.push(n);

            let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
            let parsed = matter.parse::<serde_json::Value>(&content);

            if let Ok(ref p) = parsed {
                if let Some(mins) = p
                    .data
                    .as_ref()
                    .and_then(|d| d.get("estimated_minutes"))
                    .and_then(|v| v.as_u64())
                {
                    phase_estimated_minutes.insert(n, mins as u16);
                }
            }

            // On parse error (malformed frontmatter), fall back to treating the
            // full file as the body.
            let body = parsed.map(|p| p.content).unwrap_or(content);
            phase_headings.insert(n, extract_h2_headings(&body));
        }
    }

    let probe = load_probe(dir)?.map(|p| p.spec);
    let known_concept_ids = if probe.is_some() {
        // Only check 21 needs the corpus, and only a probe can trigger it. A
        // node without one should not pay for a tree walk.
        discover_concept_ids(dir)
    } else {
        Vec::new()
    };

    Ok(ParsedNode {
        meta,
        phase_files_found,
        phase_headings,
        phase_estimated_minutes,
        probe,
        known_concept_ids,
    })
}

/// Lowercase hex sha256 of a string.
pub fn sha256_hex(text: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_is_stable_and_hex() {
        let a = sha256_hex("probe");
        assert_eq!(a.len(), 64);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(a, sha256_hex("probe"));
        assert_ne!(a, sha256_hex("probe "));
    }

    #[test]
    fn a_node_without_a_probe_loads_as_none() {
        let dir = Path::new("../../content/classical-mechanics");
        // Not a node dir; load_probe must not invent a file.
        assert!(load_probe(dir).unwrap().is_none());
    }
}
