use leptos::prelude::*;

use crate::{create_program::CreateProgram, current_workout::CurrentWorkout, utils::models::Program, workout_list::WorkoutList};



#[component]
pub fn App() -> impl IntoView {
    let (active_page, set_active_page) = signal("create-program".to_string());
    // let (active_page, set_active_page) = signal("current-workout".to_string());
    let program_to_update = Program {
        id:0,
        title:"".to_string(),
        active: RwSignal::new(false), 
        image: None 
    };

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
            <Show
                when=move || active_page.get() == "create-program"
            >
                <CreateProgram program=program_to_update.clone()/>
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
