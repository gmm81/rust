extern crate diesel;
extern crate dotenv;
extern crate microservice;


use diesel::pg::PgConnection;
use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;

use microservice::errors::*;
use microservice::dao;

use std::env;
use std::time::Duration;
use dotenv::dotenv;

//TODO: @george don't forget to uncomment later
//#[macro_use]
//extern crate diesel_migrations;
// Migrations get pulled into the final binary. This makes it quite a bit
// easier to run them on remote clusters without trouble.
//embed_migrations!("./migrations");


// Время ожидания в секундах, по истечению которого, все неактивные соединения в пуле будут закрыты.
const IDLE_TIMEOUT: u64 = 10;
// Максимальное колличество соединений
const NUM_CONNECTIONS: u32 = 50;
// Время ожидания соединения с базой в секундах
const POOL_TIMEOUT: u64 = 10;

fn main() {
    println!("Hello internet");
    println!("Time to connect to database");
    test();
}

fn test() -> Result<()> {
    println!("Getting pool");
    let pool = pool()?;
    println!("Getting connection from pool");
    let conn = pool.get()?;
    println!("Getting profile");
    //let result = dao::profile_dao::DAO { connection: &*conn, first_name: "", last_name: "", email: "gmm81@gmail.com", age: 37, sex: true }.run();
    let result = dao::profile_dao::get_profile(&*conn, 1);
    match result {
        Ok(res) => {
            println!("Found user: {:?}", res.data);
        }
        Err(e) => {
            println!("Err {:?}", e);
        }
    }
    Ok(())
}

/// Инициализация пула соединений к базе
fn pool() -> Result<Pool<ConnectionManager<PgConnection>>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .connection_timeout(Duration::from_secs(POOL_TIMEOUT))
        .idle_timeout(Some(Duration::from_secs(IDLE_TIMEOUT)))
        .max_size(NUM_CONNECTIONS)
        .min_idle(Some(0))
        .build(manager)
        .map_err(Error::from)
}