-- Your SQL goes here
CREATE TABLE representatives(
    student_id TEXT PRIMARY KEY,
    family_name TEXT NOT NULL,
    given_name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);