//! Pure XP, streak, and mastery functions — no database dependencies, fully unit-testable.

use chrono::NaiveDate;

/// Maximum number of freeze tokens a user can hold.
pub const MAX_FREEZE_TOKENS: u32 = 3;

/// Compute XP awarded for completing a quiz at a given depth tier with a given score.
///
/// Returns 0 if score_pct < 70 (failed quiz threshold per D-02).
/// Base XP varies by concept depth; score scales the reward; 100% without hints grants a 1.5x
/// perfect bonus. Using hints halves XP and disables the perfect bonus (D-10, D-11).
pub fn compute_xp(depth_tier: &str, score_pct: u32, hints_used: bool) -> u32 {
    if score_pct < 70 {
        return 0;
    }

    let base: u32 = match depth_tier {
        "root" => 15,
        "trunk" => 20,
        "branch" => 30,
        "leaf" => 40,
        _ => 20,
    };

    if score_pct == 100 && !hints_used {
        // Perfect bonus: 1.5x the base (only without hints)
        (base as f64 * 1.5).round() as u32
    } else if hints_used {
        // Hint penalty: 50% of normal score-scaled XP, no perfect bonus
        (base as f64 * score_pct as f64 / 100.0 * 0.5).round() as u32
    } else {
        (base as f64 * score_pct as f64 / 100.0).round() as u32
    }
}

/// Derive mastery tier label from cumulative XP for a concept.
///
/// Thresholds per D-09: 0-49=none, 50-149=bronze, 150-299=silver, 300+=gold.
pub fn xp_to_mastery_tier(xp: i32) -> &'static str {
    match xp {
        0..=49 => "none",
        50..=149 => "bronze",
        150..=299 => "silver",
        _ => "gold",
    }
}

/// Result of a streak update computation.
#[derive(Debug, PartialEq)]
pub struct StreakUpdate {
    pub new_streak: u32,
    pub new_freeze_tokens: u32,
    pub freeze_used: bool,
}

/// Compute updated streak state after a qualifying session on `today`.
///
/// Rules (per D-06, D-07):
/// - No prior activity → streak = 1, tokens unchanged, freeze_used = false
/// - Same day as last activity → no change (idempotent)
/// - Last activity was yesterday → streak + 1
/// - Last activity was 2 days ago AND freeze_tokens > 0 → streak + 1, tokens - 1, freeze_used = true
/// - Otherwise (gap > 1 day without freeze, or gap > 2 days) → streak resets to 1
pub fn update_streak(
    last_activity: Option<NaiveDate>,
    current_streak: u32,
    freeze_tokens: u32,
    today: NaiveDate,
) -> StreakUpdate {
    match last_activity {
        None => StreakUpdate {
            new_streak: 1,
            new_freeze_tokens: freeze_tokens,
            freeze_used: false,
        },
        Some(last) => {
            let days_gap = (today - last).num_days();

            if days_gap == 0 {
                // Same day — idempotent
                StreakUpdate {
                    new_streak: current_streak,
                    new_freeze_tokens: freeze_tokens,
                    freeze_used: false,
                }
            } else if days_gap == 1 {
                // Consecutive day
                StreakUpdate {
                    new_streak: current_streak + 1,
                    new_freeze_tokens: freeze_tokens,
                    freeze_used: false,
                }
            } else if days_gap == 2 && freeze_tokens > 0 {
                // One missed day, covered by freeze token
                StreakUpdate {
                    new_streak: current_streak + 1,
                    new_freeze_tokens: freeze_tokens - 1,
                    freeze_used: true,
                }
            } else {
                // Streak broken
                StreakUpdate {
                    new_streak: 1,
                    new_freeze_tokens: freeze_tokens,
                    freeze_used: false,
                }
            }
        }
    }
}

/// Returns true if `streak` is a milestone that awards a freeze token.
///
/// Milestones: 7, 14, 30, 60, 90, then every 90 thereafter (180, 270, ...).
pub fn check_streak_milestone(streak: u32) -> bool {
    matches!(streak, 7 | 14 | 30 | 60 | 90) || (streak > 90 && streak % 90 == 0)
}

/// Returns true if the score_pct represents a perfect attempt (100%).
pub fn is_perfect_score(score_pct: u32) -> bool {
    score_pct == 100
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── compute_xp ──────────────────────────────────────────────────────────

    #[test]
    fn compute_xp_root_85_returns_13() {
        assert_eq!(compute_xp("root", 85, false), 13);
    }

    #[test]
    fn compute_xp_trunk_100_perfect_returns_30() {
        assert_eq!(compute_xp("trunk", 100, false), 30);
    }

    #[test]
    fn compute_xp_branch_70_threshold_returns_21() {
        assert_eq!(compute_xp("branch", 70, false), 21);
    }

    #[test]
    fn compute_xp_leaf_100_perfect_returns_60() {
        assert_eq!(compute_xp("leaf", 100, false), 60);
    }

    #[test]
    fn compute_xp_branch_65_below_threshold_returns_0() {
        assert_eq!(compute_xp("branch", 65, false), 0);
    }

    #[test]
    fn compute_xp_branch_0_returns_0() {
        assert_eq!(compute_xp("branch", 0, false), 0);
    }

    #[test]
    fn compute_xp_unknown_tier_uses_default_base() {
        // default base = 20, score 80 → (20 * 0.8).round() = 16
        assert_eq!(compute_xp("unknown", 80, false), 16);
    }

    // ── xp_to_mastery_tier ──────────────────────────────────────────────────

    #[test]
    fn mastery_tier_0_is_none() {
        assert_eq!(xp_to_mastery_tier(0), "none");
    }

    #[test]
    fn mastery_tier_49_is_none() {
        assert_eq!(xp_to_mastery_tier(49), "none");
    }

    #[test]
    fn mastery_tier_50_is_bronze() {
        assert_eq!(xp_to_mastery_tier(50), "bronze");
    }

    #[test]
    fn mastery_tier_149_is_bronze() {
        assert_eq!(xp_to_mastery_tier(149), "bronze");
    }

    #[test]
    fn mastery_tier_150_is_silver() {
        assert_eq!(xp_to_mastery_tier(150), "silver");
    }

    #[test]
    fn mastery_tier_299_is_silver() {
        assert_eq!(xp_to_mastery_tier(299), "silver");
    }

    #[test]
    fn mastery_tier_300_is_gold() {
        assert_eq!(xp_to_mastery_tier(300), "gold");
    }

    #[test]
    fn mastery_tier_1000_is_gold() {
        assert_eq!(xp_to_mastery_tier(1000), "gold");
    }

    // ── update_streak ────────────────────────────────────────────────────────

    fn today() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 3, 23).unwrap()
    }

    fn yesterday() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 3, 22).unwrap()
    }

    fn two_days_ago() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 3, 21).unwrap()
    }

    fn three_days_ago() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 3, 20).unwrap()
    }

    #[test]
    fn streak_first_session_returns_1() {
        let result = update_streak(None, 0, 0, today());
        assert_eq!(result, StreakUpdate { new_streak: 1, new_freeze_tokens: 0, freeze_used: false });
    }

    #[test]
    fn streak_same_day_is_idempotent() {
        let result = update_streak(Some(today()), 5, 1, today());
        assert_eq!(result, StreakUpdate { new_streak: 5, new_freeze_tokens: 1, freeze_used: false });
    }

    #[test]
    fn streak_consecutive_day_increments() {
        let result = update_streak(Some(yesterday()), 5, 1, today());
        assert_eq!(result, StreakUpdate { new_streak: 6, new_freeze_tokens: 1, freeze_used: false });
    }

    #[test]
    fn streak_two_days_ago_with_freeze_token_uses_freeze() {
        let result = update_streak(Some(two_days_ago()), 5, 1, today());
        assert_eq!(result, StreakUpdate { new_streak: 6, new_freeze_tokens: 0, freeze_used: true });
    }

    #[test]
    fn streak_two_days_ago_without_freeze_token_breaks() {
        let result = update_streak(Some(two_days_ago()), 5, 0, today());
        assert_eq!(result, StreakUpdate { new_streak: 1, new_freeze_tokens: 0, freeze_used: false });
    }

    #[test]
    fn streak_three_days_ago_with_tokens_still_breaks() {
        let result = update_streak(Some(three_days_ago()), 5, 2, today());
        assert_eq!(result, StreakUpdate { new_streak: 1, new_freeze_tokens: 2, freeze_used: false });
    }

    // ── check_streak_milestone ───────────────────────────────────────────────

    #[test]
    fn milestone_7_days() {
        assert!(check_streak_milestone(7));
    }

    #[test]
    fn milestone_14_days() {
        assert!(check_streak_milestone(14));
    }

    #[test]
    fn milestone_30_days() {
        assert!(check_streak_milestone(30));
    }

    #[test]
    fn milestone_60_days() {
        assert!(check_streak_milestone(60));
    }

    #[test]
    fn milestone_90_days() {
        assert!(check_streak_milestone(90));
    }

    #[test]
    fn milestone_180_days_every_90_after_90() {
        assert!(check_streak_milestone(180));
    }

    #[test]
    fn milestone_270_days_every_90_after_90() {
        assert!(check_streak_milestone(270));
    }

    #[test]
    fn no_milestone_8_days() {
        assert!(!check_streak_milestone(8));
    }

    #[test]
    fn no_milestone_100_days() {
        assert!(!check_streak_milestone(100));
    }

    // ── is_perfect_score ─────────────────────────────────────────────────────

    #[test]
    fn perfect_score_100_returns_true() {
        assert!(is_perfect_score(100));
    }

    #[test]
    fn perfect_score_99_returns_false() {
        assert!(!is_perfect_score(99));
    }

    // ── compute_xp with hints_used (TDD — Feature 1) ─────────────────────────

    #[test]
    fn compute_xp_trunk_100_hints_used_returns_10() {
        // base=20, score=100, hints_used=true => 20*1.0*0.5=10, NO perfect bonus
        assert_eq!(compute_xp("trunk", 100, true), 10);
    }

    #[test]
    fn compute_xp_trunk_100_no_hints_returns_30() {
        // base=20, perfect bonus => 20*1.5=30
        assert_eq!(compute_xp("trunk", 100, false), 30);
    }

    #[test]
    fn compute_xp_leaf_85_hints_used_returns_17() {
        // base=40, 40*0.85*0.5=17
        assert_eq!(compute_xp("leaf", 85, true), 17);
    }

    #[test]
    fn compute_xp_leaf_85_no_hints_returns_34() {
        // base=40, 40*0.85=34
        assert_eq!(compute_xp("leaf", 85, false), 34);
    }

    #[test]
    fn compute_xp_branch_70_hints_used_returns_11() {
        // base=30, 30*0.70*0.5=10.5 rounds to 11
        assert_eq!(compute_xp("branch", 70, true), 11);
    }

    #[test]
    fn compute_xp_branch_65_hints_used_returns_0() {
        // below 70% threshold
        assert_eq!(compute_xp("branch", 65, true), 0);
    }

    #[test]
    fn compute_xp_root_85_no_hints_regression() {
        // regression: same as existing test (with hints_used=false)
        assert_eq!(compute_xp("root", 85, false), 13);
    }

    #[test]
    fn compute_xp_leaf_100_no_hints_regression() {
        // regression: same as existing test (with hints_used=false)
        assert_eq!(compute_xp("leaf", 100, false), 60);
    }
}
