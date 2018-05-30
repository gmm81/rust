use chrono::{DateTime, Utc};
use schema::*;

#[derive(Clone, Debug, Queryable)]
pub struct Account {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub email: Option<String>,
    pub password: String,
    pub last_ip: String,
    pub last_seen_at: DateTime<Utc>,
    pub active: bool,
}

#[table_name = "tbl_profiles"]
#[derive(Clone, Debug, Queryable,Identifiable, Associations)]
#[belongs_to(Account, foreign_key = "account_id")]
pub struct Profile {
    pub id: i64,
    pub account_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<DateTime<Utc>>,
    pub sex: Option<bool>,
}


pub mod insertable {
    use chrono::{DateTime, Utc};
    use schema::{tbl_accounts, tbl_profiles};

    #[derive(Insertable)]
    #[table_name = "tbl_accounts"]
    pub struct Account {
        pub created_at: Option<DateTime<Utc>>,
        pub email: String,
        pub password: String,
        pub last_ip: String,
        pub last_seen_at: Option<DateTime<Utc>>,
        pub active: bool,
    }

    #[derive(Insertable)]
    #[table_name = "tbl_profiles"]
    pub struct Profile {
        pub account_id: i64,
        pub first_name: String,
        pub last_name: String,
        pub birth_date: Option<DateTime<Utc>>,
        pub sex: Option<bool>,
    }
}
