use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::utils::invoke::invoke;
use crate::utils::models::Workout;
use crate::utils::video_metadata::get_thumbnail;

#[derive(Serialize, Deserialize)]
struct OpenArgs {
    workout: Workout,
}

stylance::import_style!(
    #[allow(dead_code)]
    card_style,
    "card.css"
);

#[component]
pub fn card(workout: Workout) -> impl IntoView {
    view! {
        <div>
            <div on:click={move |_| {
                let workout_clone = workout.clone();
                workout.done.set(true);
                spawn_local( async move  {
                    println!("Opening url from ui: {}", workout_clone.link.as_str());
                    let arg = to_value(&OpenArgs { workout: workout_clone }).unwrap();
                    invoke("open", arg).await;
                });
            }}
            class=move || {stylance::classes!(card_style::card_body, if workout.done.get() {Some(card_style::done)} else {None})}
            >
                <div class=card_style::card_thumbnail>
                    <img class=card_style::card_img src={ get_thumbnail(&workout.link).unwrap_or_else(|_| "https://miro.medium.com/v2/resize:fit:532/1*69aTahESxdQG3uHV8Y6Row.png".to_string()) } alt="Card image cap"/>
                    <p class=card_style::card_duration>{ workout.duration/60 }:{workout.duration%60}</p>
                    <img class=move || {
                        stylance::classes!(
                            card_style::card_checkmark,
                            if workout.done.get() {Some(card_style::card_checkmark_done)} else {None}
                        )
                    } src="https://upload.wikimedia.org/wikipedia/commons/3/3b/Eo_circle_green_checkmark.svg"/>
                </div>
                <h3>{ workout.title.clone() }</h3>
            </div>
        </div>
    }
}
