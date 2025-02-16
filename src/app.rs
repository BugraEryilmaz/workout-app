use std::time::Duration;

use leptos::task::spawn_local;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::models::Workout;
use crate::card::Card;
use crate::week::Week;

use crate::invoke::invoke;

#[derive(Serialize, Deserialize)]
struct DateArgs {
    date: chrono::NaiveDate,
}

#[component]
pub fn App() -> impl IntoView {
    let (active_day, set_active_day) = signal(chrono::Local::now().date_naive());
    let (today_workouts, set_today_workouts) = signal(Vec::<Workout>::new());

    Effect::new(move || {
        println!("Effect is being called");
        let day = active_day.get();
        spawn_local(async move {
            let arg = serde_wasm_bindgen::to_value(&DateArgs {
                date: day,
            }).expect("datetime should be serializable to JSvalue");
            let workouts = invoke("get_workouts_of", arg).await;
            let workouts: Vec<Workout> = serde_wasm_bindgen::from_value(workouts).unwrap();
            set_today_workouts.set(workouts);
        });
    });
    
    spawn_local(async move {
        // let name = String::from("Hello, ! You've been greeted from Rusasddsat!");
        let arg = serde_wasm_bindgen::to_value(&DateArgs {
            date: chrono::Local::now().date_naive(),
        }).expect("datetime should be serializable to JSvalue");
        let name_json = invoke("get_workouts_of", arg).await;
        let name: Vec<Workout> = serde_wasm_bindgen::from_value(name_json).unwrap();
        println!("name: {:?}", name);
        set_today_workouts.set(name);
    });

    view! {
        <main class="container">
            <Week active_date={active_day} set_active_date={set_active_day}/>
            // <Week/>
            <For
                each = { move || today_workouts.get() }
                key = { |workout| (workout.id, workout.done) }
                children = { |mut workout| {
                    view! {
                        <Card workout={workout} />
                    }
                }}
            />
        </main>
    }
}
