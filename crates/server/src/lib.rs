/// Auth module — Argon2id hash/verify functions.
pub mod auth;
/// Content-tree I/O shared by the `validate` and `ingest` binaries.
pub mod content_fs;
/// HTTP handlers for all API endpoints.
pub mod handlers;
/// API route definitions.
pub mod routes;
