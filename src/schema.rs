// @generated automatically by Diesel CLI.

diesel::table! {
    T1 (id) {
        id -> Integer,
        #[max_length = 64]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    T2 (id) {
        id -> Integer,
        #[max_length = 64]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    T3 (id) {
        id -> Integer,
        #[max_length = 64]
        name -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(T1, T2, T3,);
