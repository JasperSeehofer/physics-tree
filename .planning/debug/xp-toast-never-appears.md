---
status: diagnosed
trigger: "XP toast never appears after hint-assisted correct answer"
created: 2026-03-25T00:00:00Z
updated: 2026-03-25T00:00:00Z
---

## Current Focus

hypothesis: XP award requires authentication (POST /api/progress/award-xp returns 401 for unauthenticated users) and the user is likely not logged in, so post_award_xp returns None silently
test: Trace the award_xp handler auth check and the client-side response handling
expecting: The handler returns 401, gloo-net gets a non-ok response, post_award_xp returns None, toast never fires
next_action: diagnosed — return root cause

## Symptoms

expected: XP toast should appear showing reduced XP with "hint penalty applied" in teal after hint-assisted correct answer
actual: Just shows "Correct!" — no XP toast
errors: none visible to user
reproduction: Answer wrong, see hint, then answer correctly — no toast
started: After 999.1 XP toast feature was added

## Eliminated

- hypothesis: checkpoint_passed not tracking hint_used
  evidence: checkpoint.rs lines 46-48 pass hint_used from the quiz component to on_answered callback as (true, hint_used). multiple_choice.rs line 59 passes hint_shown.get() which is set to true on line 63 when first wrong attempt triggers ShowHint state.
  timestamp: 2026-03-25

- hypothesis: any_hints_used not aggregated correctly
  evidence: concept.rs line 241: `let any_hints_used = passed.iter().any(|p| matches!(p, Some((_, true))))` — correctly checks if ANY checkpoint had hint_used=true
  timestamp: 2026-03-25

- hypothesis: XpToast component not rendered
  evidence: concept.rs line 514: `<XpToast data=xp_toast_data />` is rendered unconditionally at the bottom of ConceptPage. The component uses visibility toggling (opacity-0/opacity-100), not conditional mounting.
  timestamp: 2026-03-25

- hypothesis: XP award Effect never triggers
  evidence: The Effect (concept.rs:222-270) depends on checkpoint_passed.get() and quiz_questions.get(). It checks all_answered, score >= 70%, and node_id non-empty. These conditions CAN be met for a valid quiz attempt.
  timestamp: 2026-03-25

## Evidence

- timestamp: 2026-03-25
  checked: award_xp handler auth check (progress.rs:121-131)
  found: Lines 125-131 extract user_id from session. If no user_id in session, returns (StatusCode::UNAUTHORIZED, "Not authenticated."). This is a hard auth gate — no XP without login.
  implication: If user is not logged in, POST returns 401.

- timestamp: 2026-03-25
  checked: post_award_xp client function (concept.rs:130-144)
  found: Line 140: `if !resp.ok() { return None; }` — a 401 response is not ok, so it returns None silently. No error logging.
  implication: Auth failure is silently swallowed. User sees no toast and no error.

- timestamp: 2026-03-25
  checked: XP award Effect (concept.rs:222-270)
  found: Line 256: `if let Some(response) = post_award_xp(...).await` — if post_award_xp returns None (auth failure), the if-let doesn't match and xp_toast_data is never set.
  implication: No toast when not authenticated. This is the most likely cause.

- timestamp: 2026-03-25
  checked: Whether user needs to be logged in to view quizzes
  found: Concept content (get_content) and quiz (get_quiz) endpoints have NO auth checks. Anyone can view content and take quizzes. Only award-xp requires auth.
  implication: A user can complete the entire quiz flow without logging in, but XP toast will never appear because the award POST silently fails.

- timestamp: 2026-03-25
  checked: User report: "cant reproduce, if I answer wrong and then correctly it just shows correct!"
  found: This matches exactly — quiz works fine (shows Correct!), but no XP toast because the XP award fails silently.
  implication: The user was almost certainly not logged in during testing.

- timestamp: 2026-03-25
  checked: XP Effect timing — does it fire on EACH checkpoint answer or only when ALL are answered?
  found: concept.rs:231: `if !all_answered { return; }` — the Effect only proceeds when ALL checkpoints are answered. For a concept with 5 quiz questions, answering just one won't trigger the XP flow.
  implication: Even if logged in, user must answer ALL questions before XP awards. If user only tested one question, toast wouldn't appear.

## Resolution

root_cause: Two compounding issues prevent the XP toast from appearing:

1. **Authentication requirement (primary):** The POST /api/progress/award-xp endpoint requires a session with user_id (progress.rs:125-131). If the user is not logged in, it returns 401. The client-side post_award_xp function (concept.rs:140) silently returns None on any non-ok response, so xp_toast_data is never set and the toast never renders. There is no feedback to the user that login is required for XP.

2. **All-checkpoints gate (secondary):** The XP award Effect (concept.rs:228-233) only fires when ALL quiz checkpoints are answered (`all_answered` check). If the user only answers one question and expects to see a toast, it won't appear until every checkpoint on the page is cleared.

The user's report "cant reproduce, if I answer wrong and then correctly it just shows correct!" is consistent with either: (a) not being logged in, or (b) not completing all checkpoints on the page. The most likely cause is (a) since the quiz flow gives no indication that login is needed for XP.

fix: empty — diagnosis only
verification: empty
files_changed: []
