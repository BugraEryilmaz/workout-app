use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::create_program::recents::card::RecentsCard;
use crate::create_program::InsertWorkoutArgs;
use crate::utils::invoke::invoke;
use crate::utils::models::Workout;
use crate::utils::recently_used::LruCache;

stylance::import_style!(
    #[allow(dead_code)]
    recents_style,
    "recents.css"
);

#[component]
pub fn RecentsList(
    recents_list: RwSignal<LruCache<(String, i32, String), Workout>>,
    workout_list: RwSignal<Vec<Workout>>,
    day_id: impl Fn() -> i32 + 'static + Clone + Send,
) -> impl IntoView {
    return view! {
        <div class=recents_style::recents_container>
            <For
                each = move || {
                    let recents: Vec<((String, i32, String), Workout)> = recents_list.get().iter().map(|(key, workout)| (key.clone(), workout.clone())).collect();
                    recents
                }
                key = { |&(ref key, _)| key.clone() }
                children = { move |(key, workout)| {
                    view! {
                        <RecentsCard workout={workout.clone()}
                            on:click= {
                            let value = day_id.clone();
                            move |e| {
                                e.prevent_default();
                                let mut workout = workout.clone();
                                workout.day_id = value();
                                console_log(&format!("workout: {:?}", workout));
                                let key = key.clone();
                                spawn_local(async move {
                                    let arg = InsertWorkoutArgs { workout: workout };
                                    let new_workout = invoke("create_workout", serde_wasm_bindgen::to_value(&arg).unwrap()).await;
                                    let new_workout: Workout = serde_wasm_bindgen::from_value(new_workout).unwrap();
                                    workout_list.update(|workouts| {
                                        workouts.push(new_workout.clone());
                                    });
                                    recents_list.update(|recents| {
                                        recents.put(key.clone(), new_workout.clone());
                                    });
                                });
                            }
                            }
                        />
                    }
                }}
            />
        </div>
    };
}
