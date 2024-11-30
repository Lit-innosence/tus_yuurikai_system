-- Your SQL goes here
CREATE TABLE circle_auth_info(
    auth_id UUID PRIMARY KEY,
    main_student_id TEXT NOT NULL,
    main_family_name TEXT NOT NULL,
    main_given_name TEXT NOT NULL,
    main_email TEXT NOT NULL,
    main_phone TEXT NOT NULL,
    co_student_id TEXT NOT NULL,
    co_family_name TEXT NOT NULL,
    co_given_name TEXT NOT NULL,
    co_email TEXT NOT NULL,
    co_phone TEXT NOT NULL,
    b_doc TEXT NOT NULL,
    c_doc TEXT NOT NULL,
    d_doc TEXT NOT NULL,
    organization_name TEXT NOT NULL,
    organization_ruby TEXT NOT NULL,
    organization_email TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (auth_id) REFERENCES auth(auth_id)
);