-- Your SQL goes here
CREATE TABLE locker_auth_info(
    auth_id UUID PRIMARY KEY,
    main_student_id TEXT NOT NULL,
    main_family_name TEXT NOT NULL,
    main_given_name TEXT NOT NULL,
    co_student_id TEXT NOT NULL,
    co_family_name TEXT NOT NULL,
    co_given_name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (auth_id) REFERENCES auth(auth_id)
);