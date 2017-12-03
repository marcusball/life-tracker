use database::DbConn;
use models::*;
use errors::*;
use logic::counters;

use std::path::{Path, PathBuf};
use diesel::prelude::*;
use rocket_contrib::{Json, Template};
use rocket::request::Form;
use rocket::response::status::NotFound;
use rocket::response::{NamedFile, Redirect};

use chrono::offset::Utc;

type StdResult<T, E> = ::std::result::Result<T, E>;

#[get("/")]
pub fn hello() -> Template {
    Template::render("index", ())
}

#[get("/counters", format = "application/json")]
pub fn counters_json(conn: DbConn) -> Json<Vec<Counter>> {
    Json(counters::counters(conn))
}

#[get("/counters", rank = 2)]
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

#[derive(Serialize)]
pub struct CounterResponse {
    id: i32,
    url: String,
    name: String,
    unit: String,
    events: String,
}

impl CounterResponse {
    fn new(counter: Counter) -> Self {
        CounterResponse {
            id: counter.id,
            name: counter.name,
            unit: counter.unit,
            events: format!("/counter/{}/events", &counter.url),
            url: counter.url,
        }
    }
}

#[get("/counter/<counter_url>", format = "application/json")]
pub fn counter_json(
    counter_url: String,
    conn: DbConn,
) -> StdResult<Json<CounterResponse>, NotFound<String>> {
    // Select the requested Counter
    let counter = Counter::from_url(&counter_url, &conn)
        .map_err(|_| NotFound(r##"{"error":"not found"}"##.to_owned()))?;

    let context = CounterResponse::new(counter);

    Ok(Json(context))
}


#[get("/counter/<counter_url>", rank = 2)]
pub fn counter(counter_url: String, conn: DbConn) -> StdResult<Template, Result<NotFound<String>>> {
    #[derive(Serialize)]
    pub struct Context {
        counter: Counter,
        events: Vec<CounterEvent>,
    }


    // Select the requested Counter
    let counter = Counter::from_url(&counter_url, &conn).map_err(|_| {
        Ok(NotFound("Could not find the requested counter!".to_owned()))
    })?;

    // Read its associated events
    let events = counter.events(&conn).map_err(|e| Err(e))?;

    let context = Context { counter, events };

    Ok(Template::render("counter", &context))
}


#[derive(FromForm)]
pub struct EventForm {
    quantity: i32,
}

#[post("/counter/<counter_url>", data = "<event>")]
pub fn counter_new_event(
    counter_url: String,
    event: Form<EventForm>,
    conn: DbConn,
) -> StdResult<Redirect, Result<NotFound<String>>> {
    let counter = Counter::from_url(&counter_url, &conn).map_err(|_| {
        Ok(NotFound("Could not find the requested counter!".to_owned()))
    })?;

    counter
        .add_event(event.into_inner().quantity, Utc::now(), &conn)
        .ok();

    Ok(Redirect::to(&format!("/counter/{}", counter_url)))
}
