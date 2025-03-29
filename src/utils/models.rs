use leptos::prelude::RwSignal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Workout {
    pub id: i32,
    pub link: String,
    pub title: String,
    pub duration: i32,
    pub done: RwSignal<bool>,
    pub day_id: i32,
    pub done_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Day {
    pub id: i32,
    pub program_id: i32,
    pub done: bool,
    pub complete_date: Option<String>,
    pub day_number: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Program {
    pub id: i32,
    pub title: String,
    pub active: RwSignal<bool>,
    pub image: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Achievement {
    pub id: i32,
    pub program_id: i32,
    pub date: String,
}
