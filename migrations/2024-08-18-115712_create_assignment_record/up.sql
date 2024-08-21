-- Your SQL goes here
CREATE TABLE assignment_record(
    record_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pair_id UUID NOT NULL,
    locker_id TEXT NOT NULL,
    year INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (pair_id) REFERENCES student_pair(pair_id),
    FOREIGN KEY (locker_id) REFERENCES locker(locker_id)
);