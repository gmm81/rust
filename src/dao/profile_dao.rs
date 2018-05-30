use errors::*;
use model::Profile;
use schema::tbl_profiles;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct RunResult {
    pub data: Profile
}

pub fn get_profile(connection: &PgConnection, id: i64) -> Result<Profile> {
    tbl_profiles::table
        .filter(tbl_profiles::id.eq(id))
        .get_result(connection)
        .chain_err(|| "Error selecting profile")
}
