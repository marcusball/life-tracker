use chrono::DateTime;
use chrono::offset::Utc;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Counter {
    pub id: i32,
    pub url: String,
    pub name: String,
    pub unit: String,
}

#[derive(Queryable)]
pub struct CounterEvent {
    pub id: i32,
    pub cid: i32,
    pub quantity: i32,
    pub timestamp: DateTime<Utc>,
}
