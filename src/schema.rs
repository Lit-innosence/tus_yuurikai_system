// @generated automatically by Diesel CLI.

diesel::table! {
    auth (auth_token) {
        auth_token -> Text,
        main_student_id -> Text,
        main_family_name -> Text,
        main_given_name -> Text,
        co_student_id -> Text,
        co_family_name -> Text,
        co_given_name -> Text,
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

diesel::allow_tables_to_appear_in_same_query!(
    auth,
    student,
    student_pair,
);
