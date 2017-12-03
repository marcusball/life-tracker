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
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use(log, info, debug, trace, warn)]
extern crate log;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod schema;
pub mod models;
mod database;
mod logic;
mod errors;
mod routes;

use database::DbConn;
use models::*;
use logic::counters;

use std::path::{Path, PathBuf};
use diesel::prelude::*;
use rocket_contrib::{Json, Template};
use rocket::response::status::NotFound;
use rocket::response::NamedFile;


#[get("/static/<file..>")]
fn static_content(file: PathBuf) -> std::io::Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(file))
}

fn main() {
    rocket::ignite()
        .manage(database::init_pool())
        .mount(
            "/",
            routes![
                routes::hello,
                routes::counters,
                routes::counters_json,
                routes::counter,
                routes::counter_json,
                routes::counter_events_json,
                routes::counter_new_event,
                static_content
            ],
        )
        .attach(Template::fairing())
        .launch();
}
