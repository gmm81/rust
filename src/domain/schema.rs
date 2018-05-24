table! {
    tbl_accounts (id) {
        id -> Int4,
        fk_profile -> Int4,
        login -> Varchar,
        password -> Varchar,
        active -> Bool,
    }
}

table! {
    tbl_profiles (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        age -> Nullable<Int4>,
        sex -> Nullable<Bool>,
    }
}

joinable!(tbl_accounts -> tbl_profiles (fk_profile));

allow_tables_to_appear_in_same_query!(
    tbl_accounts,
    tbl_profiles,
);
