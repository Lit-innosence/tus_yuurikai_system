// @generated automatically by Diesel CLI.

diesel::table! {
    assignment_record (record_id) {
        record_id -> Uuid,
        pair_id -> Uuid,
        locker_id -> Text,
        year -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    locker (locker_id) {
        locker_id -> Text,
        location -> Text,
    }
}

diesel::table! {
    student (student_id) {
        student_id -> Text,
        family_name -> Text,
        given_name -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    student_pair (pair_id) {
        pair_id -> Uuid,
        student_id1 -> Text,
        student_id2 -> Text,
        year -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(assignment_record -> locker (locker_id));
diesel::joinable!(assignment_record -> student_pair (pair_id));

diesel::allow_tables_to_appear_in_same_query!(
    assignment_record,
    locker,
    student,
    student_pair,
);
