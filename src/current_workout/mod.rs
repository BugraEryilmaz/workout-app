mod card;
mod week;

use card::card::*;
use serde::{Deserialize, Serialize};
use week::week::*;
use leptos::{prelude::*, task::spawn_local};

use crate::utils::models::Workout;
use crate::utils::invoke::invoke;

#[derive(Serialize, Deserialize)]
struct DateArgs {
    date: chrono::NaiveDate,
}

#[component]
pub fn CurrentWorkout() -> impl IntoView {
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
    
    set_active_day.set(chrono::Local::now().date_naive());

    return view! {
        <div>
            <Week active_date={active_day} set_active_date={set_active_day}/>
            <For
                each = { move || today_workouts.get() }
                key = { |workout| (workout.id, workout.done) }
                children = { |mut workout| {
                    view! {
                        <Card workout={workout} />
                    }
                }}
            />
        </div>
    }
}