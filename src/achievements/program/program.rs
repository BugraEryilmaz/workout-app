use chrono::NaiveDate;
use leptos::task::spawn_local;
use leptos::prelude::*;

use crate::utils::invoke::invoke;
use crate::utils::models::{Achievement, Program};

stylance::import_style!(#[allow(dead_code)] pub program_style, "program.css");

#[derive(serde::Serialize)]
pub struct ProgramAction {
    pub program: Program,
}

#[derive(serde::Serialize)]
struct DeleteAchievementArgs {
    achievementid: i32,
}

#[component]
pub fn ProgramCard(
    program: Program,
    achievement: Achievement,
    programs: WriteSignal<Vec<(Program, Achievement)>>,
) -> impl IntoView {
    let date: NaiveDate = NaiveDate::parse_from_str(&achievement.date, "%Y-%m-%d").unwrap();
    return view! {
        <div
            class=stylance::classes!(
                    program_style::program
        )>
            
            <div class=program_style::program_title>
                { program.title.clone() }
            </div>
            <div
                class={
                    stylance::classes!(
                        program_style::program_info
                    )
                }
            >
                <div class=program_style::program_date>
                    { format!("Finished: {}", date.format("%d %b %y")) }
                </div>
                <i class=stylance::classes!(program_style::delete_icon, "material-symbols")
                    on:click={
                        move |_| {
                        spawn_local(async move {
                            let arg = DeleteAchievementArgs { achievementid: achievement.id };
                            invoke("delete_achievement", serde_wasm_bindgen::to_value(&arg).unwrap()).await;
                        });
                        programs.update(|programs| {
                            programs.retain(|p| p.1.id != achievement.id);
                        });
                    }}
                >"delete"</i>
            </div>
        </div>
    };
}
