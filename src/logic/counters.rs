use database::DbConn;
use models::*;
use errors::*;

use diesel::prelude::*;

use rocket::request::FromParam;
use rocket::http::RawStr;

impl Counter {
    pub fn events(&self, conn: &DbConn) -> Result<Vec<CounterEvent>> {
        use schema::counters::dsl::*;
        use schema::counter_events::dsl::*;

        counter_events
            .filter(cid.eq(&self.id))
            .load::<CounterEvent>(conn.get())
            .map_err(|e| {
                Error::with_chain(e, "Unable to load events for this counter")
            })
    }

    pub fn from_url<U>(counter_url: U, conn: &DbConn) -> Result<Self> where U: AsRef<str> {
        use schema::counters::dsl::*;

        Ok(counters
            .filter(url.eq(counter_url.as_ref()))
            .first::<Counter>(conn.get())?)
    }
}

pub fn counters(conn: DbConn) -> Vec<Counter> {
    use schema::counters::dsl::*;

    let conn = conn.get();

    counters.load::<Counter>(conn).unwrap()
}

pub fn counter(counter_url: String, conn: DbConn) -> Result<Vec<CounterEvent>> {
    use schema::counters::dsl::*;
    use schema::counter_events::dsl::*;

    // Get the Counter for which this endpoint will be returning event data, if one exists
    let counter = Counter::from_url(&counter_url, &conn)
        .map_err(|e| Error::with_chain(e, "Counter not found!"))?;

    counter.events(&conn)
}
