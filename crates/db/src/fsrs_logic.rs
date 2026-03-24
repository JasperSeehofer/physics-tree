//! Pure FSRS scheduling functions — no database dependencies, fully unit-testable.
//!
//! Follows the same pattern as xp_logic.rs: pure functions + #[cfg(test)] mod tests.

use chrono::{DateTime, Utc};
use rs_fsrs::{Card, Rating, State, FSRS};

/// Serializable representation of an FSRS card state persisted in the progress table.
#[derive(Debug, Clone, PartialEq)]
pub struct FsrsCard {
    pub stability: f64,
    pub difficulty: f64,
    pub elapsed_days: i64,
    pub scheduled_days: i64,
    pub reps: i32,
    pub lapses: i32,
    /// "New" | "Learning" | "Review" | "Relearning"
    pub state: String,
    /// None for brand-new cards that have never been reviewed.
    pub last_review: Option<DateTime<Utc>>,
    pub due: DateTime<Utc>,
}

/// Convert a score percentage to an FSRS rating.
///
/// Thresholds per D-03:
/// - <70   → Again
/// - 70–84 → Hard
/// - 85–94 → Good
/// - 95+   → Easy
pub fn score_to_rating(score_pct: u32) -> Rating {
    match score_pct {
        0..=69 => Rating::Again,
        70..=84 => Rating::Hard,
        85..=94 => Rating::Good,
        _ => Rating::Easy,
    }
}

/// Diminishing-returns XP multiplier for review events per D-08.
///
/// - review_count 0 (first pass)  → 1.0  (full XP)
/// - review_count 1               → 0.5  (half XP)
/// - review_count 2+              → 0.25 (quarter XP)
pub fn review_xp_multiplier(review_count: u32) -> f64 {
    match review_count {
        0 => 1.0,
        1 => 0.5,
        _ => 0.25,
    }
}

/// Convert an `FsrsCard` to the rs_fsrs `Card` type for scheduling.
pub fn fsrs_card_to_rs(card: &FsrsCard) -> Card {
    let state = match card.state.as_str() {
        "Learning" => State::Learning,
        "Review" => State::Review,
        "Relearning" => State::Relearning,
        _ => State::New,
    };

    let last_review = card.last_review.unwrap_or_else(Utc::now);

    Card {
        due: card.due,
        stability: card.stability,
        difficulty: card.difficulty,
        elapsed_days: card.elapsed_days,
        scheduled_days: card.scheduled_days,
        reps: card.reps,
        lapses: card.lapses,
        state,
        last_review,
    }
}

/// Convert an rs_fsrs `Card` back to `FsrsCard` for storage.
pub fn rs_to_fsrs_card(card: &Card) -> FsrsCard {
    let state = match card.state {
        State::New => "New",
        State::Learning => "Learning",
        State::Review => "Review",
        State::Relearning => "Relearning",
    }
    .to_string();

    FsrsCard {
        stability: card.stability,
        difficulty: card.difficulty,
        elapsed_days: card.elapsed_days,
        scheduled_days: card.scheduled_days,
        reps: card.reps,
        lapses: card.lapses,
        state,
        last_review: Some(card.last_review),
        due: card.due,
    }
}

/// Create a default new FSRS card (state = New, no review history).
pub fn new_fsrs_card() -> FsrsCard {
    rs_to_fsrs_card(&Card::new())
}

/// Schedule the next review for a card given a score percentage and current time.
///
/// Converts the score to an FSRS rating, runs the FSRS algorithm, and returns
/// the updated card state with the new `due` date.
pub fn schedule_review(card: FsrsCard, score_pct: u32, now: DateTime<Utc>) -> FsrsCard {
    let fsrs = FSRS::default();
    let rs_card = fsrs_card_to_rs(&card);
    let rating = score_to_rating(score_pct);
    let record_log = fsrs.repeat(rs_card, now);
    let info = &record_log[&rating];
    rs_to_fsrs_card(&info.card)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    // ── score_to_rating ──────────────────────────────────────────────────────

    #[test]
    fn score_0_maps_to_again() {
        assert_eq!(score_to_rating(0), Rating::Again);
    }

    #[test]
    fn score_69_maps_to_again() {
        assert_eq!(score_to_rating(69), Rating::Again);
    }

    #[test]
    fn score_70_maps_to_hard() {
        assert_eq!(score_to_rating(70), Rating::Hard);
    }

    #[test]
    fn score_84_maps_to_hard() {
        assert_eq!(score_to_rating(84), Rating::Hard);
    }

    #[test]
    fn score_85_maps_to_good() {
        assert_eq!(score_to_rating(85), Rating::Good);
    }

    #[test]
    fn score_94_maps_to_good() {
        assert_eq!(score_to_rating(94), Rating::Good);
    }

    #[test]
    fn score_95_maps_to_easy() {
        assert_eq!(score_to_rating(95), Rating::Easy);
    }

    #[test]
    fn score_100_maps_to_easy() {
        assert_eq!(score_to_rating(100), Rating::Easy);
    }

    // ── review_xp_multiplier ─────────────────────────────────────────────────

    #[test]
    fn multiplier_0_reviews_is_1_0() {
        assert_eq!(review_xp_multiplier(0), 1.0);
    }

    #[test]
    fn multiplier_1_review_is_0_5() {
        assert_eq!(review_xp_multiplier(1), 0.5);
    }

    #[test]
    fn multiplier_2_reviews_is_0_25() {
        assert_eq!(review_xp_multiplier(2), 0.25);
    }

    #[test]
    fn multiplier_5_reviews_is_0_25() {
        assert_eq!(review_xp_multiplier(5), 0.25);
    }

    // ── schedule_review ──────────────────────────────────────────────────────

    #[test]
    fn schedule_new_card_good_rating_due_is_in_future() {
        let card = new_fsrs_card();
        let now = Utc::now();
        let result = schedule_review(card, 85, now);
        assert!(
            result.due > now,
            "Expected due date {} to be after now {}",
            result.due,
            now
        );
    }

    #[test]
    fn schedule_new_card_again_rating_reps_incremented() {
        let card = new_fsrs_card();
        let now = Utc::now();
        let result = schedule_review(card, 50, now);
        // After Again rating, reps should be incremented (FSRS tracks attempts)
        assert!(result.reps > 0, "Expected reps to be incremented after scheduling");
    }

    #[test]
    fn schedule_good_rating_preserves_nonzero_stability() {
        let card = new_fsrs_card();
        let now = Utc::now();
        let result = schedule_review(card, 85, now);
        assert!(
            result.stability > 0.0,
            "Expected non-zero stability after Good rating, got {}",
            result.stability
        );
    }

    // ── round-trip ───────────────────────────────────────────────────────────

    #[test]
    fn fsrs_card_round_trip() {
        let original = FsrsCard {
            stability: 2.5,
            difficulty: 5.0,
            elapsed_days: 3,
            scheduled_days: 7,
            reps: 2,
            lapses: 0,
            state: "Review".to_string(),
            last_review: Some(Utc::now() - Duration::days(3)),
            due: Utc::now() + Duration::days(4),
        };

        let rs_card = fsrs_card_to_rs(&original);
        let restored = rs_to_fsrs_card(&rs_card);

        assert_eq!(restored.stability, original.stability);
        assert_eq!(restored.difficulty, original.difficulty);
        assert_eq!(restored.elapsed_days, original.elapsed_days);
        assert_eq!(restored.scheduled_days, original.scheduled_days);
        assert_eq!(restored.reps, original.reps);
        assert_eq!(restored.lapses, original.lapses);
        assert_eq!(restored.state, original.state);
    }

    #[test]
    fn new_fsrs_card_has_new_state() {
        let card = new_fsrs_card();
        assert_eq!(card.state, "New");
        assert_eq!(card.reps, 0);
        assert_eq!(card.lapses, 0);
    }
}
