// @generated automatically by Diesel CLI.

diesel::table! {
    ninja_turtles (user_id) {
        user_id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 50]
        weapon -> Varchar,
        created_on -> Nullable<Timestamp>,
    }
}
