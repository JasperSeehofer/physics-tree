//! Integration tests for Learning Room API endpoints.
//! Stubs created by Wave 0 — implementations filled by Plan 01 and Plan 05.

#[tokio::test]
#[ignore = "Wave 0 stub — implement in Plan 01"]
async fn test_get_learning_room_content_returns_phases() {
    // GET /api/learning-room/kinematics should return phases
    // VALIDATION ref: 11-01-01
    todo!("Implement: GET /api/learning-room/:slug returns phases for node with content")
}

#[tokio::test]
#[ignore = "Wave 0 stub — implement in Plan 01"]
async fn test_get_learning_room_content_404_for_unknown_slug() {
    // GET /api/learning-room/nonexistent should return 404
    // VALIDATION ref: 11-01-01
    todo!("Implement: GET /api/learning-room/:slug returns 404 for unknown slug")
}

#[tokio::test]
#[ignore = "Wave 0 stub — implement in Plan 01"]
async fn test_get_progress_empty_for_anonymous() {
    // GET /api/learning-room/kinematics/progress returns [] without auth
    // VALIDATION ref: 11-01-06
    todo!("Implement: anonymous progress returns empty array")
}

#[tokio::test]
#[ignore = "Wave 0 stub — implement in Plan 01"]
async fn test_post_progress_requires_auth() {
    // POST /api/learning-room/kinematics/progress returns 401 without auth
    // VALIDATION ref: 11-01-06
    todo!("Implement: POST progress requires authentication")
}

#[tokio::test]
#[ignore = "Wave 0 stub — implement in Plan 01"]
async fn test_post_progress_gate_enforced() {
    // POST phase 2 without phase 1 complete returns 403
    // VALIDATION ref: 11-01-06
    todo!("Implement: server-side gate blocks out-of-order phase completion")
}

#[tokio::test]
#[ignore = "Wave 0 stub — implement in Plan 05"]
async fn test_existing_content_endpoint_unchanged() {
    // GET /api/content/:slug still works (regression)
    // VALIDATION ref: 11-01-05
    todo!("Implement: existing content API unaffected by Learning Room changes")
}
