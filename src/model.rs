use serde::{Deserialize, Serialize};
use time::Date;

time::serde::format_description!(date_ftm, Date, "[year]-[month]-[day]");

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct Person {
    id: i32,
    name: String,
    nick: String,
    #[serde(with = "date_ftm")]
    birth_date: Date,
    stack: Option<Vec<String>>,
}

#[derive(Clone, Deserialize)]
pub struct NewPerson {
    pub name: String,
    pub nick: String,
    #[serde(with = "date_ftm")]
    pub birth_date: Date,
    pub stack: Option<Vec<String>>,
}
