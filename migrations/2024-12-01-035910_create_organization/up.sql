-- Your SQL goes here
CREATE TABLE organization(
    organization_id TEXT PRIMARY KEY,
    organization_name TEXT NOT NULL,
    organization_ruby TEXT NOT NULL,
    organization_email TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);