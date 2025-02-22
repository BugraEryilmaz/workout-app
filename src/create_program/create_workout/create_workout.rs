use leptos::{prelude::*, task::spawn_local};
use serde::{Deserialize, Serialize};

use crate::utils::{invoke::invoke, models::Workout, video_metadata::get_thumbnail};

stylance::import_style!(#[allow(dead_code)] card_style, "card.css");

#[derive(Serialize, Deserialize, Debug)]
struct DeleteWorkoutArgs {
    workoutid: i32
}

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
                        <div class=card_style::card_body style="justify-content: space-between;">
                            <div class=card_style::card_body>
                                <div class=card_style::card_thumbnail>
                                    <img class=card_style::card_img src={ get_thumbnail(&workout.link).unwrap_or_else(|_| "https://miro.medium.com/v2/resize:fit:532/1*69aTahESxdQG3uHV8Y6Row.png".to_string()) } alt="Card image cap"/>
                                    <p class=card_style::card_duration>{ workout.duration/60 }:{workout.duration%60}</p>
                                    </div>
                                    <h3>{ workout.title.clone() }</h3>
                            </div>
                            <i class="material-icons"
                                on:click={move |_| {
                                    spawn_local(async move {
                                        let arg = DeleteWorkoutArgs { workoutid: workout.id };
                                        invoke("delete_workout", serde_wasm_bindgen::to_value(&arg).unwrap()).await;
                                    });
                                    workouts.update(|workouts| {
                                        workouts.retain(|w| w.id != workout.id);
                                    });
                                }}
                                style="color: red; cursor: pointer; margin: 0.5em;"
                            >"delete"</i>
                        </div>
                    }
                }}
            />
        </div>
    }
}