-- Your SQL goes here
CREATE TABLE auth(
    auth_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    main_auth_token TEXT NOT NULL,
    co_auth_token TEXT NOT NULL,
    phase TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);