// @generated automatically by Diesel CLI.

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

diesel::allow_tables_to_appear_in_same_query!(
    student,
    student_pair,
);
