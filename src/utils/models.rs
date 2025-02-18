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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
    pub id: i32,
    pub program_id: i32,
    pub done: bool,
    pub complete_date: Option<String>,
    pub day_number: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Program {
    pub id: i32,
    pub title: String,
    pub active: RwSignal<bool>,
    pub image: Option<String>,
}