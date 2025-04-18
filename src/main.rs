mod app;
mod utils;
mod current_workout;
mod workout_list;
mod create_program;
mod achievements;

use app::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
