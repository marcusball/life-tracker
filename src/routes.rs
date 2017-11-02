use database::DbConn;
use models::*;
use errors::*;
use logic::counters;

use std::path::{Path, PathBuf};
use diesel::prelude::*;
use rocket_contrib::{Json, Template};
use rocket::response::status::NotFound;
use rocket::response::NamedFile;


#[get("/")]
pub fn hello() -> Template {
    Template::render("index", ())
}

#[get("/counters")]
pub fn counters(conn: DbConn) -> Template {
    #[derive(Serialize)]
    struct Context {
        counters: Vec<Counter>,
    };

    let context = Context {
        counters: counters::counters(conn),
    };

    Template::render("counters", &context)
}

#[get("/counter/<counter_url>")]
pub fn counter(counter_url: String, conn: DbConn) -> Result<Json<Vec<CounterEvent>>> {
    Ok(Json(counters::counter(counter_url, conn)?))
}
