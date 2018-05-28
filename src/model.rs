use schema::*;

#[table_name = "tbl_accounts"]
#[derive(Clone, Debug, Queryable, Insertable, AsChangeset)]
pub struct Account {
    pub id: i32,
    pub login: String,
    pub password: String,
    pub active: bool,
}

#[table_name = "tbl_profiles"]
#[derive(Clone, Debug, Queryable, Insertable, AsChangeset)]
pub struct Profile {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

/*
pub mod insertable {
    use schema::{tbl_accounts, tbl_profiles};

    #[derive(Insertable)]
    #[table_name = "tbl_accounts"]
    pub struct Account {
        pub login: String,
        pub password: String,
        pub active: bool,
    }

    #[derive(Insertable)]
    #[table_name = "tbl_profiles"]
    pub struct Profile {
        pub first_name: String,
        pub last_name: String,
        pub email: String,
    }
}*/
