use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::workouts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Workout {
    pub id: i32,
    pub link: String,
    pub title: String,
    pub duration: i32,
    pub done: bool,
    pub day_id: i32,
    pub done_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::days)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Day {
    pub id: i32,
    pub program_id: i32,
    pub done: bool,
    pub complete_date: Option<String>,
    pub day_number: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::programs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Program {
    pub id: i32,
    pub title: String,
    pub active: bool,
    pub image: Option<String>,
}
