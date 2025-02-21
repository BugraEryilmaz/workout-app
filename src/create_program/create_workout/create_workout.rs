use leptos::prelude::*;

use crate::utils::{models::Workout, video_metadata::get_thumbnail};

stylance::import_style!(#[allow(dead_code)] card_style, "card.css");

#[component]
pub fn WorkoutCard(
    workouts: RwSignal<Vec<Workout>>
) -> impl IntoView {
    view! {
        <div>
            <For
                each = move || workouts.get()
                key = { |workout| workout.id }
                children = { move |workout| {
                    view! {
                        <div class=card_style::card_body>
                            <div class=card_style::card_thumbnail>
                                <img class=card_style::card_img src={ get_thumbnail(&workout.link).unwrap_or_else(|_| "https://miro.medium.com/v2/resize:fit:532/1*69aTahESxdQG3uHV8Y6Row.png".to_string()) } alt="Card image cap"/>
                                <p class=card_style::card_duration>{ workout.duration/60 }:{workout.duration%60}</p>
                            </div>
                            <h3>{ workout.title.clone() }</h3>
                        </div>
                    }
                }}
            />
        </div>
    }
}