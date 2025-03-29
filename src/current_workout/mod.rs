mod card;
mod done_day;
mod week;

use card::card::*;
use done_day::*;
use leptos::{prelude::*, task::spawn_local};
use serde::{Deserialize, Serialize};
use week::week::*;

use crate::utils::invoke::invoke;
use crate::utils::models::{Day, Workout};

#[derive(Serialize, Deserialize)]
struct DateArgs {
    date: chrono::NaiveDate,
}

stylance::import_style!(
    #[allow(dead_code)]
    current_workout_style,
    "current_workout.css"
);

#[component]
pub fn CurrentWorkout() -> impl IntoView {
    let (active_day, set_active_day) = signal(chrono::Local::now().date_naive());
    let (today_workouts, set_today_workouts) = signal(Vec::<Workout>::new());
    let (day, set_day) = signal(None::<Day>);

    Effect::new(move || {
        println!("Effect is being called");
        let day = active_day.get();
        spawn_local(async move {
            let arg = serde_wasm_bindgen::to_value(&DateArgs { date: day })
                .expect("datetime should be serializable to JSvalue");
            let workouts = invoke("get_workouts_date", arg).await;
            let workout_days: (Vec<Workout>, Option<Day>) =
                serde_wasm_bindgen::from_value(workouts).unwrap();
            let (workouts, day) = workout_days;
            set_today_workouts.set(workouts);
            set_day.set(day);
        });
    });

    set_active_day.set(chrono::Local::now().date_naive());

    return view! {
        <div class=current_workout_style::current_workout_container>
            <Week active_date={active_day} set_active_date={set_active_day}/>
            <For
                each = { move || today_workouts.get() }
                key = { |workout| (workout.id, workout.done) }
                children = { move |mut workout| {
                    view! {
                        <Card workout={workout} set_day={set_day}/>
                    }
                }}
            />
            <Show when=move || {day.get().is_some() && day.get().unwrap().complete_date == Some(active_day.get().to_string())}>
                <DoneDay day={day} date={active_day} />
            </Show>

        </div>
    };
}
