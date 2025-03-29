mod program;

use crate::utils::invoke::invoke;
use crate::utils::models::{Achievement, Program};
use leptos::task::spawn_local;
use leptos::prelude::*;
use program::ProgramCard;
use wasm_bindgen::JsValue;

stylance::import_style!(
    #[allow(dead_code)]
    workout_list_style,
    "workout_list.css"
);

#[component]
pub fn AchivementList() -> impl IntoView {
    let (programs, set_programs) = signal(Vec::<(Program, Achievement)>::new());
    spawn_local(async move {
        let progs = invoke("get_achievements", JsValue::null()).await;
        let progs: Vec<(Program, Achievement)> = serde_wasm_bindgen::from_value(progs).unwrap();
        set_programs.set(progs);
    });

    return view! {
        <div>
            <div class=workout_list_style::program_list>
                <For
                    each = { move || programs.get() }
                    key = { |program| program.1.id }
                    children = { move |program| {
                        view! {
                            <div>
                                <ProgramCard program=program.0 achievement=program.1 programs=set_programs/>
                            </div>
                        }
                    }}
                />
                <Show when=move || programs.get().is_empty()>
                    <div>
                        "No Achievements"
                    </div>
                </Show>
            </div>
        </div>
    };
}
