#![recursion_limit = "128"]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate chrono;
extern crate r2d2;

mod model;
mod schema;

pub mod errors;
pub mod dao;