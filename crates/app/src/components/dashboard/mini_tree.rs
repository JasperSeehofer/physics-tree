//! MiniTree — animated SVG botanical knowledge tree on the dashboard.
//!
//! Nodes render as tier-aware botanical shapes based on cumulative mastery XP:
//! - Seed  (0..=49):    small dim dot — not yet learned
//! - Sprout/bronze (50..=149):  bronze petal stubs with animate-fade-in
//! - Leaf/silver (150..=299):   silver diamond shape with animate-fade-in
//! - Bloom/gold (300+):         green flower with 6 petals + glow filter, animate-scale-in
//!
//! Entrance animations stagger by index * 50ms. Bloom nodes are rendered first
//! (highest visual priority), then leaf, then sprout/seed.
//!
//! Animations disabled via `prefers-reduced-motion: reduce` CSS media query in main.css.

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
    /// Cumulative concept XP (mastery_level column in DB). Tiers derived at render time.
    pub mastery_level: i32,
}

/// SVG mini knowledge tree — nodes rendered as botanical shapes by mastery tier.
/// Clickable to /graph/{slug}/learn.
#[component]
pub fn MiniTree(nodes: Vec<NodeProgress>) -> impl IntoView {
    // Empty state: no nodes or all nodes are unlearned
    let has_any_progress = nodes.iter().any(|n| n.mastery_level >= 50);
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

    // Build connection lines between tiers
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
            let cx_a = nodes_a.iter().map(|(x, _, _)| x).sum::<f64>() / nodes_a.len() as f64;
            let cy_a = nodes_a[0].1;
            let cx_b = nodes_b.iter().map(|(x, _, _)| x).sum::<f64>() / nodes_b.len() as f64;
            let cy_b = nodes_b[0].1;
            lines.push((cx_a, cy_a, cx_b, cy_b));
        }
        lines
    };

    // Sort nodes by visual priority: bloom first, then leaf, then sprout, then seed
    // This ensures higher-mastery nodes appear on top
    let mut sorted_positions: Vec<(f64, f64, &NodeProgress, usize)> = node_positions
        .iter()
        .enumerate()
        .map(|(i, (x, y, n))| (*x, *y, *n, i))
        .collect();
    sorted_positions.sort_by_key(|(_, _, n, _)| {
        // Reversed: higher mastery = lower sort key = rendered last (on top)
        match n.mastery_level {
            300.. => 3,
            150..=299 => 2,
            50..=149 => 1,
            _ => 0,
        }
    });

    // Build node SVG elements with botanical shapes per mastery tier
    let node_elements: Vec<_> = sorted_positions
        .iter()
        .enumerate()
        .map(|(stagger_idx, (x, y, node, _))| {
            let xf = *x;
            let yf = *y;
            let href = format!("/graph/{}/learn", node.slug);
            let mastery = node.mastery_level;
            let delay_ms = stagger_idx * 50;
            let delay_style = format!("animation-delay: {}ms", delay_ms);

            let tooltip = match mastery {
                0..=49 => format!("{} - not yet learned", node.title),
                50..=149 => format!("{} - Bronze - {} XP", node.title, mastery),
                150..=299 => format!("{} - Silver - {} XP", node.title, mastery),
                _ => format!("{} - Gold - Mastered", node.title),
            };

            let aria_label = format!("{} \u{2014} open concept", node.title);

            match mastery {
                // ── Seed (0..=49): small dim dot, no animation ────────────────
                0..=49 => {
                    let cx = xf.to_string();
                    let cy = yf.to_string();
                    view! {
                        <a href=href aria-label=aria_label class="cursor-pointer">
                            <title>{tooltip}</title>
                            <circle
                                cx=cx
                                cy=cy
                                r="4"
                                fill="var(--color-bark-light)"
                            />
                        </a>
                    }.into_any()
                }

                // ── Sprout/bronze (50..=149): circle + petal stubs, fade-in ──
                50..=149 => {
                    let cx = xf.to_string();
                    let cy = yf.to_string();
                    // Petal stub paths (relative to node center)
                    let left_petal = format!("M {} {} L {} {}",
                        xf, yf - 6.0, xf - 2.0, yf - 10.0);
                    let center_petal = format!("M {} {} L {} {}",
                        xf, yf - 6.0, xf, yf - 11.0);
                    let right_petal = format!("M {} {} L {} {}",
                        xf, yf - 6.0, xf + 2.0, yf - 10.0);
                    view! {
                        <a href=href aria-label=aria_label class="cursor-pointer">
                            <title>{tooltip}</title>
                            <g class="animate-fade-in" style=delay_style>
                                <circle
                                    cx=cx
                                    cy=cy
                                    r="6"
                                    fill="var(--color-sun-amber)"
                                    opacity="0.8"
                                />
                                <path
                                    d=left_petal
                                    stroke="var(--color-sun-amber)"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    fill="none"
                                />
                                <path
                                    d=center_petal
                                    stroke="var(--color-sun-amber)"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    fill="none"
                                />
                                <path
                                    d=right_petal
                                    stroke="var(--color-sun-amber)"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    fill="none"
                                />
                            </g>
                        </a>
                    }.into_any()
                }

                // ── Leaf/silver (150..=299): diamond shape, fade-in ───────────
                150..=299 => {
                    // 4-point diamond: M x y-8 L x+7 y L x y+8 L x-7 y Z
                    let diamond = format!(
                        "M {} {} L {} {} L {} {} L {} {} Z",
                        xf, yf - 8.0,
                        xf + 7.0, yf,
                        xf, yf + 8.0,
                        xf - 7.0, yf
                    );
                    view! {
                        <a href=href aria-label=aria_label class="cursor-pointer">
                            <title>{tooltip}</title>
                            <g class="animate-fade-in" style=delay_style>
                                <path
                                    d=diamond
                                    fill="var(--color-mist)"
                                />
                            </g>
                        </a>
                    }.into_any()
                }

                // ── Bloom/gold (300+): flower with 6 petals + glow, scale-in ─
                _ => {
                    // 6 petal circles at 60-degree intervals, radius 3, offset 10px from center
                    let petals: Vec<(f64, f64)> = (0..6)
                        .map(|i| {
                            let angle = std::f64::consts::PI * 2.0 / 6.0 * i as f64;
                            let px = xf + 10.0 * angle.cos();
                            let py = yf + 10.0 * angle.sin();
                            (px, py)
                        })
                        .collect();

                    let cx = xf.to_string();
                    let cy = yf.to_string();

                    view! {
                        <a href=href aria-label=aria_label class="cursor-pointer">
                            <title>{tooltip}</title>
                            <g class="animate-scale-in" style=delay_style filter="url(#bloom-glow)">
                                // Center circle
                                <circle
                                    cx=cx.clone()
                                    cy=cy.clone()
                                    r="8"
                                    fill="var(--color-leaf-green)"
                                />
                                // 6 petal circles
                                {petals.into_iter().map(|(px, py)| {
                                    view! {
                                        <circle
                                            cx=px.to_string()
                                            cy=py.to_string()
                                            r="3"
                                            fill="var(--color-leaf-green)"
                                        />
                                    }
                                }).collect_view()}
                            </g>
                        </a>
                    }.into_any()
                }
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
            let y = tier_y(tier) as f64 + 5.0;
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
            // SVG defs: bloom glow filter
            <defs>
                <filter id="bloom-glow">
                    <feGaussianBlur stdDeviation="3" result="blur"/>
                    <feComposite in_="SourceGraphic" in2="blur" operator="over"/>
                </filter>
            </defs>

            // Connection lines (rendered first, behind nodes)
            {line_elements}

            // Tier labels
            {label_elements}

            // Node botanical shapes (sorted by mastery tier — bloom on top)
            {node_elements}
        </svg>
    }.into_any()
}
