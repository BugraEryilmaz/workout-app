use leptos::prelude::*;

use crate::current_workout::CurrentWorkout;



#[component]
pub fn App() -> impl IntoView {
    let (active_page, set_active_page) = signal("current-workout".to_string());

    view! {
        <main class="container">
            <div class="workout-list"
                style:visibility=move || {if active_page.get() == "current-workout" { "visible" } else { "hidden" }}
            >
                <CurrentWorkout />
            </div>
            <div class="workout-list"
                style:visibility=move || {if active_page.get() == "workout-list" { "visible" } else { "hidden" }}
            >
            </div>
            <div
                style="position: fixed; bottom: 1em; width: 100%; display: flex; justify-content: space-evenly;"
            >
                <button on:click={move |_| {
                    set_active_page.set("current-workout".to_string());
                }}>Current Workout</button>
                <button on:click={move |_| {
                    set_active_page.set("workout-list".to_string());
                }}>Workout List</button>
            </div>
        </main>
    }
}
