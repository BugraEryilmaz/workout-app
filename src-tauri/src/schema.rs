// @generated automatically by Diesel CLI.

diesel::table! {
    days (id) {
        id -> Integer,
        program_id -> Integer,
        done -> Bool,
        complete_date -> Nullable<Date>,
        day_number -> Integer,
    }
}

diesel::table! {
    programs (id) {
        id -> Integer,
        title -> Text,
        active -> Bool,
        image -> Nullable<Text>,
    }
}

diesel::table! {
    workouts (id) {
        id -> Integer,
        link -> Text,
        title -> Text,
        duration -> Integer,
        done -> Bool,
        day_id -> Integer,
    }
}

diesel::joinable!(days -> programs (program_id));
diesel::joinable!(workouts -> days (day_id));

diesel::allow_tables_to_appear_in_same_query!(
    days,
    programs,
    workouts,
);
