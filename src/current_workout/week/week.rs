use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use wasm_bindgen::prelude::*;
use chrono::Datelike;
use leptos::task::spawn_local;
use leptos::prelude::*;

fn get_monday_of_week(date: chrono::NaiveDate) -> chrono::NaiveDate {
    let today_offset = date.weekday().num_days_from_monday();
    let monday = date - chrono::Duration::days(today_offset as i64);
    monday
}

stylance::import_style!(#[allow(dead_code)] week, "week.css");

#[leptos::component]
pub fn Week(active_date: ReadSignal<chrono::NaiveDate>, set_active_date: WriteSignal<chrono::NaiveDate>) -> impl IntoView {
    let weekfn = move || {
        let monday = get_monday_of_week(active_date.get());
        let week: Vec<chrono::NaiveDate> = (0..7).map(|i| monday + chrono::Duration::days(i)).collect();
        return week;
    };

    return view! {
        <div class=week::calendar>
            <button class={format!("{} {}", week::btn, week::btn_today)} on:click={move |_| set_active_date.set(chrono::Local::now().date_naive())}>{"Today"}</button>
            <div class=week::week>
                <button class=week::btn on:click={move |_| set_active_date.update(|day| *day -= chrono::Duration::days((day.weekday().num_days_from_monday()+1).into()))}>{"<"}</button>
                <For
                each = {weekfn}
                key = { |date: &chrono::NaiveDate| *date }
                children = { move |date: chrono::NaiveDate| {
                    view! {
                        <button class=move || {
                            let mut class = format!("{} {}", week::btn, week::day);
                            if date == active_date.get() {
                                class.push_str(" ");
                                class.push_str(week::active);
                            }
                            if date == chrono::Local::now().date_naive() {
                                class.push_str(" ");
                                class.push_str(week::today);
                            }
                            class
                        }
                        class:week::btn
                        
                        on:click={move |_| {
                            leptos::leptos_dom::log!("{:?}", date);
                            set_active_date.set(date)
                        }}
                        > 
                        <span>{date.weekday().to_string()}</span>
                        <span>{date.day()}</span>
                        </button>
                    }
                }}
                />
                <button class=week::btn on:click={move |_| set_active_date.update(|day| *day += chrono::Duration::days((7-day.weekday().num_days_from_monday()).into()))}>{">"}</button>
                </div>
            </div>
        };
    }