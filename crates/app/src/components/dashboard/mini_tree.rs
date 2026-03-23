use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Per-node progress entry for the mini knowledge tree — mirrors NodeProgress from progress_repo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeProgress {
    pub node_id: String,
    pub slug: String,
    pub title: String,
    pub branch: String,
    pub depth_tier: String,
    pub mastery_level: i32,
}

/// SVG mini knowledge tree — nodes colored by mastery level, clickable to /graph/{slug}/learn.
#[component]
pub fn MiniTree(nodes: Vec<NodeProgress>) -> impl IntoView {
    // Empty state: no nodes or all nodes are unlearned
    let has_any_progress = nodes.iter().any(|n| n.mastery_level > 0);
    let is_empty = nodes.is_empty() || !has_any_progress;

    if is_empty {
        return view! {
            <div class="text-center py-12">
                <h3 class="text-xl font-bold text-petal-white mb-2">"Your tree is just a seed"</h3>
                <p class="text-sm text-mist">
                    "Complete a concept module to see your first bloom. "
                    <a href="/graph" class="text-sky-teal hover:underline">"Start with the graph explorer."</a>
                </p>
            </div>
        }.into_any();
    }

    // Group nodes by depth_tier for tree layout
    // Tier order: root (bottom) -> trunk -> branch -> leaf (top)
    let tier_order = ["root", "trunk", "branch", "leaf"];
    let svg_height = 480i32;
    let svg_width = 800i32;
    let tier_count = tier_order.len() as i32;

    // Vertical positions for each tier (bottom to top)
    // root at y=420, trunk at y=300, branch at y=180, leaf at y=60
    let tier_y = |tier: &str| -> i32 {
        match tier {
            "root" => 420,
            "trunk" => 300,
            "branch" => 180,
            "leaf" => 60,
            _ => 240,
        }
    };

    // Group nodes by tier
    let mut tiers: std::collections::HashMap<&str, Vec<&NodeProgress>> =
        std::collections::HashMap::new();
    for node in &nodes {
        tiers
            .entry(node.depth_tier.as_str())
            .or_default()
            .push(node);
    }

    // Build node positions: evenly distributed horizontally within each tier
    let node_positions: Vec<(f64, f64, &NodeProgress)> = tier_order
        .iter()
        .flat_map(|tier| {
            let tier_nodes = tiers.get(tier).cloned().unwrap_or_default();
            let count = tier_nodes.len();
            if count == 0 {
                return vec![];
            }
            let y = tier_y(tier) as f64;
            let spacing = svg_width as f64 / (count as f64 + 1.0);
            tier_nodes
                .iter()
                .enumerate()
                .map(|(i, node)| {
                    let x = spacing * (i as f64 + 1.0);
                    (x, y, *node)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    // Build connection lines between tiers (trunk line + branch lines)
    // Simple: draw vertical lines from each tier's centroid to the next tier's centroid
    let trunk_lines: Vec<(f64, f64, f64, f64)> = {
        let mut lines = vec![];
        for i in 0..(tier_order.len() - 1) {
            let tier_a = tier_order[i];
            let tier_b = tier_order[i + 1];
            let nodes_a: Vec<_> = node_positions
                .iter()
                .filter(|(_, _, n)| n.depth_tier == tier_a)
                .collect();
            let nodes_b: Vec<_> = node_positions
                .iter()
                .filter(|(_, _, n)| n.depth_tier == tier_b)
                .collect();
            if nodes_a.is_empty() || nodes_b.is_empty() {
                continue;
            }
            // Connect centroid of tier_a to centroid of tier_b
            let cx_a = nodes_a.iter().map(|(x, _, _)| x).sum::<f64>() / nodes_a.len() as f64;
            let cy_a = nodes_a[0].1;
            let cx_b = nodes_b.iter().map(|(x, _, _)| x).sum::<f64>() / nodes_b.len() as f64;
            let cy_b = nodes_b[0].1;
            lines.push((cx_a, cy_a, cx_b, cy_b));
        }
        lines
    };

    // Render node circles with mastery-based fill colors
    let node_elements: Vec<_> = node_positions
        .iter()
        .map(|(x, y, node)| {
            let (fill, opacity) = match node.mastery_level {
                0 => ("var(--color-bark-light)", "1"),
                1..=49 => ("var(--color-leaf-green)", "0.5"),
                _ => ("var(--color-leaf-green)", "1"),
            };
            let href = format!("/graph/{}/learn", node.slug);
            let aria_label = format!("{} \u{2014} open concept", node.title);
            let title_text = format!("{} ({})", node.title, node.branch);
            let cx = x.to_string();
            let cy = y.to_string();
            let fill = fill.to_string();
            let opacity = opacity.to_string();
            let _title = node.title.clone();
            view! {
                <a
                    href=href
                    aria-label=aria_label
                    class="cursor-pointer"
                >
                    <title>{title_text}</title>
                    <circle
                        cx=cx
                        cy=cy
                        r="12"
                        fill=fill
                        opacity=opacity
                    />
                </a>
            }
        })
        .collect();

    // Render trunk/branch connection lines
    let line_elements: Vec<_> = trunk_lines
        .iter()
        .map(|(x1, y1, x2, y2)| {
            view! {
                <line
                    x1=x1.to_string()
                    y1=y1.to_string()
                    x2=x2.to_string()
                    y2=y2.to_string()
                    stroke="var(--color-bark-mid)"
                    stroke-width="2"
                />
            }
        })
        .collect();

    // Render tier labels at the right edge
    let label_elements: Vec<_> = tier_order
        .iter()
        .filter_map(|tier| {
            let count = tiers.get(tier).map(|v| v.len()).unwrap_or(0);
            if count == 0 {
                return None;
            }
            let y = tier_y(tier) as f64 + 5.0; // center on circle
            let label = match *tier {
                "root" => "Root",
                "trunk" => "Trunk",
                "branch" => "Branch",
                "leaf" => "Leaf",
                _ => "",
            };
            Some(view! {
                <text
                    x="780"
                    y=y.to_string()
                    text-anchor="end"
                    font-size="11"
                    fill="var(--color-mist)"
                >
                    {label}
                </text>
            })
        })
        .collect();

    let _ = tier_count; // suppress unused warning

    view! {
        <svg
            viewBox=format!("0 0 {} {}", svg_width, svg_height)
            class="w-full"
            role="img"
            aria-label="Knowledge tree showing your learning progress"
        >
            // Connection lines (rendered first, behind circles)
            {line_elements}

            // Tier labels
            {label_elements}

            // Node circles
            {node_elements}
        </svg>
    }.into_any()
}
