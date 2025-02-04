-- Your SQL goes here
CREATE TABLE registration(
    organization_id INT NOT NULL,
    year INT NOT NULL,
    main_student_id TEXT NOT NULL,
    co_student_id TEXT NOT NULL,
    status_acceptance TEXT NOT NULL,
    status_authentication TEXT NOT NULL,
    status_form_confirmation TEXT NOT NULL,
    status_registration_complete TEXT NOT NULL,
    b_doc TEXT NOT NULL,
    c_doc TEXT NOT NULL,
    d_doc TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (organization_id, year),
    FOREIGN KEY (organization_id) REFERENCES organization(organization_id),
    FOREIGN KEY (main_student_id) REFERENCES representatives(student_id),
    FOREIGN KEY (co_student_id) REFERENCES representatives(student_id)
);