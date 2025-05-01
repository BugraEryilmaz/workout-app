use chrono::NaiveDate;
use leptos::prelude::*;

use crate::utils::models::Day;

stylance::import_style!(
    #[allow(dead_code)]
    done_day_style,
    "done_day.css"
);

#[component]
pub fn done_day(day: ReadSignal<Option<Day>>, date: ReadSignal<NaiveDate>) -> impl IntoView {
    view! {
        <div>
            <h1>{ move || format!("Finished the Day {} on ", day.get().unwrap().day_number.unwrap())}
                <span>
                {move || date.get().format("%d %B").to_string()}
                </span>
            </h1>
            <img src="/public/done.gif" class=done_day_style::img />
        </div>
    }
}

#[component]
pub fn rest_day(date: ReadSignal<NaiveDate>) -> impl IntoView {
    view! {
        <div>
            <h1>{ move || format!("Rest day on {}", date.get().format("%d %B").to_string())}
            </h1>
            <img src="/public/rest.gif" class=done_day_style::rest />
        </div>
    }
}
