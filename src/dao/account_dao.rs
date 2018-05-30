use errors::*;
use model::{Account, Profile};
use schema::{tbl_accounts, tbl_profiles};

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct ReturnResult {
    pub account: Account,
    pub profile: Profile,
}

pub fn get_account(connection: &PgConnection, id: i64) -> Result<Account> {
    tbl_accounts::table
        .filter(tbl_accounts::id.eq(id))
        .get_result(connection)
        .chain_err(|| "Error selecting account")
}


pub fn get_account_with_profile(connection: &PgConnection, id: i64) /*-> ReturnResult*/ {
    let joined_tuples = tbl_accounts::table
        .inner_join(tbl_profiles::table)
        .filter(tbl_accounts::id.eq(id))
        .load::<(Account, Profile)>(connection)
        .chain_err(|| "Error selecting account with profile");


    /*  match res {
          Ok(r) => {
              ReturnResult {
                  account: r[0].0,
                  profile: r[0].1,
              }
          }
          Err(_) => {}
      }*/
    //Ok(joined_tuples)
    print!("res: {:?}", joined_tuples)
}
