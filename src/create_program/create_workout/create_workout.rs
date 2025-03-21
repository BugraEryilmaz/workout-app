use leptos::{html, leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use serde::{Deserialize, Serialize};

use crate::utils::{invoke::invoke, models::Workout, video_metadata::get_thumbnail};

stylance::import_style!(
    #[allow(dead_code)]
    card_style,
    "card.css"
);

#[derive(Serialize, Deserialize, Debug)]
struct DeleteWorkoutArgs {
    workoutid: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct UpdateWorkoutArgs {
    workoutid: i32,
    title: String,
}

#[component]
pub fn WorkoutCard(workouts: RwSignal<Vec<Workout>>) -> impl IntoView {
    view! {
        <div>
            <For
                each = move || workouts.get()
                key = { |workout| workout.id }
                children = { move |workout| {
                    let form_ref: NodeRef<html::Form> = NodeRef::new();
                    let title_ref: NodeRef<html::Div> = NodeRef::new();
                    view! {
                        <div class=card_style::card style="justify-content: space-between;">
                            <div class=card_style::card_body>
                                <div class=card_style::card_thumbnail>
                                    <img class=card_style::card_img src={ get_thumbnail(&workout.link).unwrap_or_else(|_| "https://miro.medium.com/v2/resize:fit:532/1*69aTahESxdQG3uHV8Y6Row.png".to_string()) } alt="Card image cap"/>
                                    <p class=card_style::card_duration>{ workout.duration/60 }:{ format!("{:02}", workout.duration%60) }</p>
                                </div>
                                <form
                                    class=card_style::card_title_form
                                    on:submit= move |e| {
                                        e.prevent_default();
                                        let title = title_ref.get().unwrap().inner_text();
                                        let arg = UpdateWorkoutArgs { workoutid: workout.id, title };
                                        spawn_local(async move {
                                            let workout = invoke("update_workout", serde_wasm_bindgen::to_value(&arg).unwrap()).await;
                                            let workout: Workout = serde_wasm_bindgen::from_value(workout).unwrap();
                                            console_log(&format!("updated workout: {:?}", workout));
                                            workouts.update(|workouts| {
                                                workouts.iter_mut().find(|w| w.id == workout.id).map(|w| {
                                                    *w = workout.clone();
                                                });
                                            });
                                        });
                                    }
                                    node_ref=form_ref
                                >
                                    <div
                                        class=card_style::card_title
                                        node_ref=title_ref
                                        contenteditable=true
                                        on:focusout=move |e| {
                                            // submit form on focus out
                                            e.prevent_default();
                                            let _ = form_ref.get().unwrap().request_submit();
                                        }
                                        on:keypress=move |e| {
                                            if e.key() == "Enter" {
                                                e.prevent_default();
                                                let _ = form_ref.get().unwrap().request_submit();
                                            }
                                        }
                                    >
                                        { workout.title.clone() }
                                    </div>
                                </form>
                                // <h3>{ workout.title.clone() }</h3>
                            </div>
                            <i class="material-symbols"
                                on:click={move |_| {
                                    spawn_local(async move {
                                        let arg = DeleteWorkoutArgs { workoutid: workout.id };
                                        invoke("delete_workout", serde_wasm_bindgen::to_value(&arg).unwrap()).await;
                                    });
                                    workouts.update(|workouts| {
                                        workouts.retain(|w| w.id != workout.id);
                                    });
                                }}
                                style="cursor: pointer; margin: 0.5em;"
                            >"delete"</i>
                        </div>
                    }
                }}
            />
        </div>
    }
}
