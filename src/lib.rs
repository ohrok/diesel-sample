pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[cfg(not(windows))]
pub const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
pub const EOF: &'static str = "CTRL+Z";
