// @generated automatically by Diesel CLI.

diesel::table! {
    person_diesel (id) {
        id -> Int4,
        name -> Varchar,
        age -> Int4,
    }
}