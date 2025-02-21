mod recents;
mod calendar;
mod create_workout;

use leptos::leptos_dom::logging::{console_error, console_log};
use leptos::{html, prelude::*};
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};

use crate::utils::invoke::invoke;
use crate::utils::models::{Program, Workout};
use crate::utils::video_metadata::{get_metadata, VideoMetadata};
use recents::*;
use calendar::*;
use create_workout::*;

#[derive(Serialize, Deserialize, Debug)]
struct InsertWorkoutArgs {
    workout: Workout
}

#[derive(Serialize, Deserialize, Debug)]
struct GetDayIdsArgs {
    programid: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct GetWorkoutsDayArgs {
    daynumber: i32,
    programid: i32
}

#[leptos::component]
pub fn CreateProgram(
    program: Program
) -> impl IntoView {
    let active_date = RwSignal::new(0);
    let day_ids = RwSignal::new(vec![]);
    let workouts: RwSignal<Vec<Workout>> = RwSignal::new(vec![]);
    let recents: RwSignal<Vec<Workout>> = RwSignal::new(vec![]);
    let input_element: NodeRef<html::Input> = NodeRef::new();
    spawn_local(async move {
        let arg = GetDayIdsArgs { programid: program.id };
        let get_day_ids = invoke("get_day_ids", serde_wasm_bindgen::to_value(&arg).unwrap()).await;
        let get_day_ids: Vec<i32> = serde_wasm_bindgen::from_value(get_day_ids).unwrap();
        day_ids.set(get_day_ids);
    });
    Effect::new(move || {
        let active_date = active_date.get();
        let program_id = program.id;
        spawn_local(async move {
            let arg = GetWorkoutsDayArgs { daynumber: active_date, programid: program_id };
            let get_workouts_day = invoke("get_workouts_day", serde_wasm_bindgen::to_value(&arg).unwrap()).await;
            let get_workouts_day: Vec<Workout> = serde_wasm_bindgen::from_value(get_workouts_day).unwrap();
            workouts.set(get_workouts_day);
        });
    });
    active_date.set(1);
    return view! {
        <div>
            <h1>{ program.title }</h1>
            <Calendar 
                active_date=active_date 
                day_ids=day_ids
                program_id=program.id
            />
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
                        let new_workout = InsertWorkoutArgs { 
                            workout: Workout {
                                id: workouts.get().len() as i32,
                                title: metadata.title,
                                link: link.clone(),
                                duration: metadata.duration as i32,
                                done: RwSignal::new(false),
                                day_id: day_ids.get()[active_date.get() as usize - 1],
                            }
                        };
                        let new_workout = invoke("create_workout", serde_wasm_bindgen::to_value(&new_workout).unwrap()).await;
                        let new_workout: Workout = serde_wasm_bindgen::from_value(new_workout).unwrap();

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