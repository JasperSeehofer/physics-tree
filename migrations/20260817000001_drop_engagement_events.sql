-- Gate 9 (D-G9b, 2026-08-17): drop `engagement_events`. Its only writer,
-- POST /api/progress/event, was registered in v1.0 but never called by any
-- client and was retired in M13; the table is empty by construction and its
-- purpose is superseded by probe_sittings / phase_sessions.
DROP TABLE IF EXISTS engagement_events;
