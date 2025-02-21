use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use wasm_bindgen::prelude::*;
use chrono::Datelike;
use leptos::task::spawn_local;
use leptos::prelude::*;

use crate::utils::invoke::invoke;

stylance::import_style!(#[allow(dead_code)] calendar, "calendar.css");

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDayArgs {
    programid: i32
}

#[leptos::component]
pub fn Calendar(active_date: RwSignal<i32>, day_ids: RwSignal<Vec<i32>>, program_id: i32) -> impl IntoView {
    let active_week = RwSignal::new(0);
    let active_day = RwSignal::new(1);
    Effect::new(move || {
        active_date.set(active_week.get() * 7 + active_day.get());
    });
    return view! {
        <div class=calendar::calendar>
            <button class=calendar::btn on:click={move |_| if active_week.get() > 0 {active_week.update(|week| *week -= 1); active_day.set(7);}}>{"<"}</button>
            <For
                each = move || {(1..8).map(|i| active_week.get() * 7 + i).collect::<Vec<i32>>()}
                key = { |day: &i32| *day }
                children = { move |date: i32| {
                    view! {
                        <button class=move || {
                            stylance::classes!(
                                calendar::btn,
                                calendar::day,
                                (date == active_day.get() + active_week.get() * 7).then_some(calendar::active)
                            )
                        }
                        
                        on:click={move |_| {
                            active_day.set(date - active_week.get() * 7);
                        }}
                        style:display={move || if date > day_ids.get().len() as i32 {"none"} else {"initial"}}
                        > 
                        <span>{date}</span>
                        </button>
                    }
                }}
                />
            <button class={calendar::btn} 
                on:click={move |_| {active_week.update(|week| *week += 1); active_day.set(1);}}
                style:display={move || if active_week.get() == (day_ids.get().len() / 7) as i32 {"none"} else {"initial"}}
            >{">"}</button>
            <button class={calendar::btn} 
                on:click={move |_| {
                    spawn_local(async move {
                        let args = CreateDayArgs { programid: program_id };
                        let day_id = invoke("create_day", to_value(&args).unwrap()).await;
                        let day_id: i32= serde_wasm_bindgen::from_value(day_id).unwrap();
                        day_ids.update(|ids| {ids.push(day_id);} );
                    });
                }}
                style:display={move || if active_week.get() == (day_ids.get().len() / 7) as i32 {"initial"} else {"none"}}
            >{"+"}</button>
        </div>
    };
}