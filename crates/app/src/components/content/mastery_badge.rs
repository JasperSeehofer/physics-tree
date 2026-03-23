//! MasteryBadge — inline tier badge for the concept page header.
//!
//! Shows bronze/silver/gold tier with XP progress toward next tier.
//! Hidden when mastery_xp < 50 (none tier).

use leptos::prelude::*;

/// Inline mastery tier badge shown on concept pages.
///
/// Tier thresholds:
/// - 0–49: none (hidden)
/// - 50–149: Bronze
/// - 150–299: Silver
/// - 300+: Gold
///
/// The `next_threshold` prop is optional — if None it is derived from the tier.
#[component]
pub fn MasteryBadge(
    mastery_xp: i32,
    #[prop(optional)] next_threshold: Option<i32>,
) -> impl IntoView {
    // Derive tier from XP
    let tier = if mastery_xp >= 300 {
        "gold"
    } else if mastery_xp >= 150 {
        "silver"
    } else if mastery_xp >= 50 {
        "bronze"
    } else {
        "none"
    };

    // Hidden when no tier
    if tier == "none" {
        return view! { <span /> }.into_any();
    }

    // Derive next threshold if not provided
    let threshold = next_threshold.unwrap_or_else(|| match tier {
        "bronze" => 150,
        "silver" => 300,
        _ => 0, // gold — no next threshold
    });

    let (badge_class, tier_label) = match tier {
        "bronze" => (
            "inline-flex items-center gap-1 rounded px-2 py-0.5 text-xs font-bold bg-sun-amber/20 text-sun-amber",
            "Bronze",
        ),
        "silver" => (
            "inline-flex items-center gap-1 rounded px-2 py-0.5 text-xs font-bold bg-mist/20 text-mist",
            "Silver",
        ),
        _ => (
            // gold
            "inline-flex items-center gap-1 rounded px-2 py-0.5 text-xs font-bold bg-leaf-green/20 text-leaf-green",
            "Gold",
        ),
    };

    let aria_label = format!("{} mastery", tier_label);

    let sub_label = if tier == "gold" {
        format!("{} XP - Mastered", mastery_xp)
    } else {
        format!("{} / {} XP to next tier", mastery_xp, threshold)
    };

    view! {
        <div class="inline-flex flex-col gap-0.5">
            <span
                class=badge_class
                aria-label=aria_label
            >
                {tier_label}
            </span>
            <span class="text-xs text-mist">{sub_label}</span>
        </div>
    }.into_any()
}
