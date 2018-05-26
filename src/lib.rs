#![recursion_limit = "128"]
#![feature(custom_attribute, extern_prelude)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate error_chain;
extern crate futures;

mod model;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod schema;

pub mod errors;
pub mod dao;