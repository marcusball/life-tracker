use chrono::DateTime;
use chrono::offset::Utc;

#[derive(Queryable)]
pub struct Counter {
    pub id: i32,
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
