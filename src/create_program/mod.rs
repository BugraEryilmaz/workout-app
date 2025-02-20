mod recents;
mod calendar;
mod create_workout;

use leptos::leptos_dom::logging::console_error;
use leptos::{html, prelude::*};
use leptos::task::spawn_local;

use crate::utils::models::{Program, Workout};
use crate::utils::video_metadata::{get_metadata, VideoMetadata};
use recents::*;
use calendar::*;
use create_workout::*;

#[leptos::component]
pub fn CreateProgram(
    program: Program
) -> impl IntoView {
    let active_date = RwSignal::new(0);
    let workouts: RwSignal<Vec<Workout>> = RwSignal::new(vec![]);
    let recents: RwSignal<Vec<Workout>> = RwSignal::new(vec![]);
    let input_element: NodeRef<html::Input> = NodeRef::new();
    return view! {
        <div>
            <Calendar active_date=active_date />
            <hr 
                style="display: block; height: 60vh; width: 70%; border: 0; border-right: 3px solid rgba(10, 10, 10, 0.3); margin: 1em 0; padding: 0;"           
            />
            <div
                style="width: 70%; position: relative; top: -60vh;"
            >
                <WorkoutCard workouts=workouts 
            />
            </div>
            <form
                style="position: fixed; bottom: 5em; width: 70%; flex-direction: row; display: flex; align-items: center;"
                on:submit={move |e| {
                    e.prevent_default();
                    let link = input_element.get().expect("The input needs to be loaded").value();
                    input_element.get().expect("The input needs to be loaded").set_value("");
                    spawn_local(async move {
                        let metadata = get_metadata(link.as_str()).await.unwrap_or_else(|e| {
                            console_error(format!("Error getting metadata: {}", e).as_str());
                            VideoMetadata {
                                title: "New Workout".to_string(),
                                duration: 30
                            }
                        });
                        let new_workout = Workout {
                            id: workouts.get().len() as i32,
                            title: metadata.title,
                            link: link.clone(),
                            duration: metadata.duration as i32,
                            done: RwSignal::new(false),
                            day_id: active_date.get(),
                        };
                        workouts.update(move |workouts| {
                            workouts.push(new_workout);
                        });
                    });
                }
            }>
                <input type="text" placeholder="Link" style="margin: 1em; margin-left: 5em" node_ref=input_element/>
                <button
                    style="border-radius: 1em; height: 100%; margin: 1em;"
                >
                    {"Add Workout"}
                </button>
            </form>
        </div>
    };
}