-- Your SQL goes here
CREATE TABLE student_pair(
    pair_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id1 TEXT NOT NULL,
    student_id2 TEXT NOT NULL,
    year INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id1) REFERENCES student(student_id),
    FOREIGN KEY (student_id2) REFERENCES student(student_id)
);