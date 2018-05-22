use domain::profile::*;

pub struct Account {
    pub id: i32,
    pub login: String,
    pub password: String,
    pub profile: Profile
}