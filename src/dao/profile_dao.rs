use errors::*;
use model::Profile;
use schema::tbl_profiles;

use diesel::pg::PgConnection;
use diesel::prelude::*;


pub struct DAO<'a> {
    pub connection: &'a PgConnection,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub age: i32,
    pub sex: bool,
}

pub struct RunResult {
    pub data: Profile,
}

impl<'a> DAO<'a> {
    pub fn run(&mut self) -> Result<RunResult> {
        //self.connection.transaction::<_, Error, _>(|| self.run_inner())
        self.run_inner()
    }

    fn run_inner(&mut self) -> Result<RunResult> {
        self.params_validate()?;
        let profile = self.select_profile(self.email)?;
        match profile {
            Some(data) => {
                Ok(RunResult { data })
            }
            None => {
                //info!("No profile with that email");
                bail!(user_errors::validation(
                "No profile matched that email address."
            ))
            }
        }
    }
    /// Проверка параметров
    fn params_validate(&mut self) -> Result<()> {
        if self.email.is_empty() {
            bail!(user_errors::validation("Please specify an email address."))
        }
        Ok(())
    }

    //Поиск профайла по электронной почте
    fn select_profile(&mut self, email: &str) -> Result<Option<Profile>> {
        tbl_profiles::table
            .filter(tbl_profiles::email.eq(email))
            .first(self.connection)
            .optional()
            .chain_err(|| "Error selecting profile")
    }
}
