table! {
    tbl_accounts (id) {
        id -> Int8,
        fk_profile -> Int8,
        created_at -> Timestamptz,
        email -> Nullable<Text>,
        password -> Text,
        last_ip -> Text,
        last_seen_at -> Timestamptz,
        active -> Bool,
    }
}

table! {
    tbl_profiles (id) {
        id -> Int8,
        first_name -> Text,
        last_name -> Text,
        birth_date -> Nullable<Timestamptz>,
        sex -> Nullable<Bool>,
    }
}

joinable!(tbl_accounts -> tbl_profiles (fk_profile));

allow_tables_to_appear_in_same_query!(
    tbl_accounts,
    tbl_profiles,
);
