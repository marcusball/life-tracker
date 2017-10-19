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
use models::*;

use std::path::{Path, PathBuf};
use diesel::prelude::*;
use rocket_contrib::{Json, Template};
use rocket::response::status::NotFound;
use rocket::response::NamedFile;

#[get("/")]
fn hello() -> Template {
    Template::render("index", ())
}

#[get("/counters")]
fn counters(conn: DbConn) -> Json<Vec<Counter>> {
    use schema::counters::dsl::*;

    let conn = conn.get();

    Json(counters.load::<Counter>(conn).unwrap())
}

#[get("/counter/<counter_url>")]
fn counter(counter_url: String, conn: DbConn) -> Result<Json<Vec<CounterEvent>>, NotFound<String>> {
    use schema::counters::dsl::*;
    use schema::counter_events::dsl::*;

    // Get the Counter for which this endpoint will be returning event data, if one exists
    let counter = counters
        .filter(url.eq(&counter_url))
        .first::<Counter>(conn.get())
        .map_err(|_| {
            warn!(
                "lol someone thought that \"{}\" was a valid counter, what a dumbass",
                counter_url
            );
            NotFound(format!("Counter \"{}\" does not exist!", counter_url))
        })?;


    // Query the event data for this counter and return it
    Ok(Json(
        counter_events
            .filter(cid.eq(&counter.id))
            .load::<CounterEvent>(conn.get())
            .unwrap(),
    ))
}

#[get("/static/<file..>")]
fn static_content(file: PathBuf) -> std::io::Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(file))
}

fn main() {
    rocket::ignite()
        .manage(database::init_pool())
        .mount("/", routes![hello, counters, counter, static_content])
        .attach(Template::fairing())
        .launch();
}
