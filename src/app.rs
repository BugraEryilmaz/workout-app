use leptos::prelude::*;

use crate::{current_workout::CurrentWorkout, workout_list::WorkoutList};



#[component]
pub fn App() -> impl IntoView {
    let (active_page, set_active_page) = signal("workout-list".to_string());
    // let (active_page, set_active_page) = signal("current-workout".to_string());

    view! {
        <main class="container">
            <Show
                when=move || active_page.get() == "current-workout"
            >
                <CurrentWorkout/>
            </Show>
            <Show
                when=move || active_page.get() == "workout-list"
            >
                <WorkoutList/>
            </Show>
            <div
                style="position: fixed; bottom: 1em; width: 100%; display: flex; justify-content: space-evenly;"
            >
                <button on:click={move |_| {
                    set_active_page.set("current-workout".to_string());
                    }}
                    style:border=move || {if active_page.get() == "current-workout" { "2px solid black" } else { "none" }}
                >Current Workout</button>
                <button on:click={move |_| {
                    set_active_page.set("workout-list".to_string());
                    }}
                    style:border=move || {if active_page.get() == "workout-list" { "2px solid black" } else { "none" }}
                >Workout List</button>
            </div>
        </main>
    }
}
