use database::DbConn;
use models::*;
use errors::*;
use logic::counters;

use std::path::{Path, PathBuf};
use diesel::prelude::*;
use rocket_contrib::{Json, Template};
use rocket::response::status::NotFound;
use rocket::response::NamedFile;

type StdResult<T, E> = ::std::result::Result<T, E>;

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
pub fn counter(counter_url: String, conn: DbConn) -> StdResult<Template, Result<NotFound<String>>> {
    use schema::counters::dsl::*;

    #[derive(Serialize)]
    struct Context {
        counter: Counter,
        events: Vec<CounterEvent>,
    };

    // Select the requested Counter
    let counter = counters
        .filter(url.eq(&counter_url))
        .first::<Counter>(conn.get())
        .map_err(|_| {
            Ok(NotFound("Could not find the requested counter!".to_owned()))
        })?;

    // Read its associated events
    let events = counter.events(&conn).map_err(|e| Err(e))?;

    let context = Context { counter, events };

    Ok(Template::render("counter", &context))
}
