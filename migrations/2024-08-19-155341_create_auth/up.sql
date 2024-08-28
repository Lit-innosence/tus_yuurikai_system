-- Your SQL goes here
CREATE TABLE auth(
    auth_token TEXT PRIMARY KEY,
    student_id TEXT NOT NULL,
    family_name TEXT NOT NULL,
    given_name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);