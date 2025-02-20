use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use wasm_bindgen::prelude::*;
use chrono::Datelike;
use leptos::task::spawn_local;
use leptos::prelude::*;

stylance::import_style!(#[allow(dead_code)] calendar, "calendar.css");

#[leptos::component]
pub fn Calendar(active_date: RwSignal<i32>) -> impl IntoView {
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
                        > 
                        <span>{date}</span>
                        </button>
                    }
                }}
                />
            <button class=calendar::btn on:click={move |_| {active_week.update(|week| *week += 1); active_day.set(1);}}>{">"}</button>
        </div>
    };
}