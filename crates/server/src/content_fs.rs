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
use domain::glossary::{prose_convention_rows, scan_term_tags, BranchConventions};
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

/// The branch directory a node directory sits in (`content/{branch}/{slug}`).
fn branch_dir(node_dir: &Path) -> Option<PathBuf> {
    node_dir.parent().map(|p| p.to_path_buf())
}

/// Read `content/{branch}/conventions.yaml`, if the branch has one (v1.5).
///
/// A missing file is `Ok(None)` — the pre-v1.5 shape and the shape of every
/// branch that has not yet had its conventions table lifted out of prose. A
/// *malformed* file is an error, for the same reason `probe.yaml` is: the
/// schema is `deny_unknown_fields`, so a typo must not silently drop the row
/// that carries a convention trap.
pub fn load_branch_conventions(node_dir: &Path) -> Result<Option<BranchConventions>, String> {
    let Some(branch) = branch_dir(node_dir) else {
        return Ok(None);
    };
    let path = branch.join("conventions.yaml");
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Ok(None);
    };
    let parsed: BranchConventions =
        serde_saphyr::from_str(&text).map_err(|e| format!("conventions.yaml:parse  {e}"))?;
    Ok(Some(parsed))
}

/// The directory name of a node's branch, for the `conventions.yaml` branch
/// check.
pub fn branch_name(node_dir: &Path) -> Option<String> {
    branch_dir(node_dir)?
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
}

/// Every `::term[key]` occurrence in one node directory's phase files, as
/// `(phase_number, key)`.
pub fn node_term_tags(node_dir: &Path) -> Vec<(u8, String)> {
    let mut tags = Vec::new();
    for n in 0u8..=6 {
        let Some(body) = read_phase_content(node_dir, n) else {
            continue;
        };
        for tag in scan_term_tags(&body) {
            tags.push((n, tag.key));
        }
    }
    tags
}

/// Every term the node's branch declares, as `(owner concept_id, key)`, and
/// every key tagged anywhere in that branch.
///
/// One walk, two answers, because both come from reading the same seven-ish
/// node directories and doing it twice would double the cost of every ingest.
/// A branch that cannot be read yields two empty vecs, which the validator
/// reads as "not supplied" and skips — the `known_concept_ids` convention.
pub fn discover_branch_glossary(node_dir: &Path) -> (Vec<(String, String)>, Vec<String>) {
    let mut declared: Vec<(String, String)> = Vec::new();
    let mut tagged: Vec<String> = Vec::new();

    let Some(branch) = branch_dir(node_dir) else {
        return (declared, tagged);
    };
    let Ok(entries) = std::fs::read_dir(&branch) else {
        return (declared, tagged);
    };

    let mut dirs: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir() && p.join("node.yaml").exists())
        .collect();
    dirs.sort();

    for dir in dirs {
        if let Ok(text) = std::fs::read_to_string(dir.join("node.yaml")) {
            if let Ok(meta) = serde_saphyr::from_str::<NodeMeta>(&text) {
                for term in &meta.terms {
                    declared.push((meta.concept_id.clone(), term.key.clone()));
                }
            }
        }
        for (_, key) in node_term_tags(&dir) {
            if !tagged.contains(&key) {
                tagged.push(key);
            }
        }
    }

    (declared, tagged)
}

/// The slugified row keys of a node's `### Conventions` prose table, wherever
/// in its seven phases it lives (phase 2 in practice, but the spec does not
/// pin it there).
fn node_prose_convention_rows(node_dir: &Path) -> Vec<String> {
    for n in 0u8..=6 {
        let Some(body) = read_phase_content(node_dir, n) else {
            continue;
        };
        let rows = prose_convention_rows(&body);
        if !rows.is_empty() {
            return rows;
        }
    }
    Vec::new()
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

    // The v1.5 glossary inputs. `term_tags` is always cheap (seven files
    // already read once above, and re-read here rather than threaded through,
    // because `parse_node_dir` is called a handful of times per run). The
    // branch walk is paid for only when this node has something glossary-shaped
    // to say — a node with neither declarations nor tags cannot fail checks
    // 23–26, so it should not pay for the corpus.
    let term_tags = node_term_tags(dir);
    let conventions = load_branch_conventions(dir)?;
    let needs_branch = !meta.terms.is_empty() || !term_tags.is_empty() || conventions.is_some();

    let (branch_terms, branch_term_tags) = if needs_branch {
        discover_branch_glossary(dir)
    } else {
        (Vec::new(), Vec::new())
    };

    let known_concept_ids = if probe.is_some() || conventions.is_some() {
        // Check 21 (route targets) and check 26 (conventions opened_by /
        // closed_by) both resolve a concept_id against the corpus.
        discover_concept_ids(dir)
    } else {
        Vec::new()
    };

    let prose_convention_rows = if conventions.is_some() {
        node_prose_convention_rows(dir)
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
        term_tags,
        branch_terms,
        branch_term_tags,
        conventions,
        prose_convention_rows,
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
