mod calendar;
mod create_workout;
mod recents;

use leptos::leptos_dom::logging::console_error;
use leptos::task::spawn_local;
use leptos::{html, prelude::*};
use serde::{Deserialize, Serialize};

use crate::utils::invoke::invoke;
use crate::utils::models::{Program, Workout};
use crate::utils::recently_used::LruCache;
use crate::utils::video_metadata::{get_metadata, VideoMetadata};
use calendar::*;
use create_workout::*;
use recents::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InsertWorkoutArgs {
    workout: Workout,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetDayIdsArgs {
    programid: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetWorkoutsDayArgs {
    daynumber: i32,
    programid: i32,
}

stylance::import_style!(
    #[allow(dead_code)]
    style,
    "create_program.css"
);

#[leptos::component]
pub fn CreateProgram(program: Program) -> impl IntoView {
    let active_date = RwSignal::new(0);
    let day_ids = RwSignal::new(vec![]);
    let workouts: RwSignal<Vec<Workout>> = RwSignal::new(vec![]);
    let recents_list: RwSignal<LruCache<(String, i32, String), Workout>> =
        RwSignal::new(LruCache::new(4));
    let input_element: NodeRef<html::Input> = NodeRef::new();
    spawn_local(async move {
        let arg = GetDayIdsArgs {
            programid: program.id,
        };
        let get_day_ids = invoke("get_day_ids", serde_wasm_bindgen::to_value(&arg).unwrap()).await;
        let get_day_ids: Vec<i32> = serde_wasm_bindgen::from_value(get_day_ids).unwrap();
        day_ids.set(get_day_ids);
    });

    spawn_local(async move {
        let get_recents = invoke("last_workouts", serde_wasm_bindgen::to_value(&()).unwrap()).await;
        let get_recents: Vec<(String, i32, String)> =
            serde_wasm_bindgen::from_value(get_recents).unwrap();
        let mut cache = LruCache::new(4);
        for (link, duration, title) in get_recents.iter().rev() {
            cache.put(
                (link.clone(), duration.clone(), title.clone()),
                Workout {
                    id: -1,
                    title: title.clone(),
                    duration: *duration,
                    link: link.clone(),
                    done: RwSignal::new(false),
                    day_id: -1,
                },
            );
        }
        let get_recents = cache;
        recents_list.set(get_recents);
    });

    Effect::new(move || {
        let active_date = active_date.get();
        let program_id = program.id;
        spawn_local(async move {
            let arg = GetWorkoutsDayArgs {
                daynumber: active_date,
                programid: program_id,
            };
            let get_workouts_day = invoke(
                "get_workouts_day",
                serde_wasm_bindgen::to_value(&arg).unwrap(),
            )
            .await;
            let get_workouts_day: Vec<Workout> =
                serde_wasm_bindgen::from_value(get_workouts_day).unwrap();
            workouts.set(get_workouts_day);
        });
    });
    active_date.set(1);
    return view! {
        <div class=style::create_program_container>
            <h1 class=style::program_title >{ program.title }</h1>
            <Calendar
                active_date=active_date
                day_ids=day_ids
                program_id=program.id
            />
            <div class=style::add_workout_section>
                <div class=style::workout_list>
                    <div class=style::workout_list_contents>
                        <WorkoutCard workouts=workouts.clone()/>
                    </div>
                </div>
                <div class=style::recents_list>
                    <div class=style::recents_list_contents>
                        <RecentsList recents_list=recents_list.clone() workout_list=workouts.clone() day_id=move || {
                            day_ids.get()[active_date.get() as usize - 1]
                        }/>
                    </div>
                </div>
            </div>
            <form
                class=style::add_workout_form
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
                                title: metadata.title.clone(),
                                link: link.clone(),
                                duration: metadata.duration as i32,
                                done: RwSignal::new(false),
                                day_id: day_ids.get()[active_date.get() as usize - 1],
                            }
                        };
                        let new_workout = invoke("create_workout", serde_wasm_bindgen::to_value(&new_workout).unwrap()).await;
                        let new_workout: Workout = serde_wasm_bindgen::from_value(new_workout).unwrap();

                        recents_list.update(|recents| {
                            recents.put((link.clone(), metadata.duration as i32, metadata.title.clone()), new_workout.clone());
                        });

                        workouts.update(move |workouts| {
                            workouts.push(new_workout);
                        });
                    });
                }
            }>
                <input type="text" placeholder="Link" class=style::add_workout_link_textbox node_ref=input_element/>
                <button class=style::add_workout_button>
                    {"Add Workout"}
                </button>
            </form>
        </div>
    };
}
