use leptos::prelude::*;

use crate::{
    create_program::CreateProgram, current_workout::CurrentWorkout, utils::models::Program,
    workout_list::WorkoutList, achievements::AchivementList,
};

stylance::import_style!(
    #[allow(dead_code)]
    app_style,
    "app.css"
);

#[component]
pub fn App() -> impl IntoView {
    let (active_page, set_active_page) = signal("current-workout");
    // let (active_page, set_active_page) = signal("workout-list");
    let program_to_update: RwSignal<Option<Program>> = RwSignal::new(None);

    Effect::new(move || {
        if !program_to_update.get().is_none() {
            set_active_page.set("update-program");
        }
    });

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
                <WorkoutList
                    program_to_update=program_to_update.clone()
                />
            </Show>
            <Show
                when=move || active_page.get() == "update-program"
            >
                <CreateProgram program=program_to_update.get().unwrap().clone()/>
            </Show>
            <Show
                when=move || active_page.get() == "achievements-page"
            >
                <AchivementList/>
            </Show>
            <div
                class=app_style::nav_bar
            >
                <button on:click={move |_| {
                    set_active_page.set("current-workout");
                    }}
                    class=move || {
                        stylance::classes!(
                            app_style::nav_button,
                            if active_page.get() == "current-workout" {Some(app_style::nav_button_active)} else { None }
                        )
                    }
                >Current Workout</button>
                <button on:click={move |_| {
                    set_active_page.set("workout-list");
                    }}
                    class=move || {
                        stylance::classes!(
                            app_style::nav_button,
                            if active_page.get() == "workout-list" {Some(app_style::nav_button_active)} else { None }
                        )
                    }
                >Workout List</button>
                <button on:click={move |_| {
                    set_active_page.set("achievements-page");
                    }}
                    class=move || {
                        stylance::classes!(
                            app_style::nav_button,
                            if active_page.get() == "achievements-page" {Some(app_style::nav_button_active)} else { None }
                        )
                    }
                >Achievements</button>
            </div>
        </main>
    }
}
