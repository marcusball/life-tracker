#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate lazy_static;
#[macro_use(log, info, debug, trace, warn)]
extern crate log;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod schema;
pub mod models;
mod database;

use database::DbConn;
use models::Counter;

use diesel::prelude::*;
use rocket_contrib::Json;

#[get("/")]
fn hello(conn: DbConn) -> &'static str {
    let _ = conn.get();

    "Suh, dude"
}

#[get("/counters")]
fn counters(conn: DbConn) -> Json<Vec<Counter>> {
    use schema::counters::dsl::*;

    let conn = conn.get();

    Json(counters.load::<Counter>(conn).unwrap())
}

fn main() {
    rocket::ignite()
        .manage(database::init_pool())
        .mount("/", routes![hello, counters])
        .launch();
}
