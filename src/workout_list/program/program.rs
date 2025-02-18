use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;

use crate::utils::models::Program;
use crate::utils::invoke::invoke;

stylance::import_style!(#[allow(dead_code)] program_style, "program.css");

#[derive(serde::Serialize)]
pub struct ProgramAction {
    pub program: Program
}

#[component]
pub fn ProgramCard(
    program: Program,
    set_action: RwSignal<Option<Program>>
) -> impl IntoView {
    return view! {
        <div
            class={
                stylance::classes!(
                    program_style::program
                )
            }
        >
            <h1>{ program.title.clone() }</h1>
            <div
                class={
                    stylance::classes!(
                        program_style::program_info
                    )
                }
            >
                <label class=program_style::switch>
                    <input type="checkbox" 
                        prop:checked=move || {program.active.get()}
                        on:input:target=move |event| {
                            let target = event.target().checked();
                            if target {
                                event.target().set_checked(!target);
                                set_action.set(Some(program.clone()));
                            } else {
                                let program_clone = program.clone();
                                spawn_local(async move {
                                    // console_log(&format!("Deactivating program: {:?}", to_value(&program_clone)));
                                    invoke("deactivate_program", to_value(&ProgramAction {
                                        program: program_clone
                                    }).unwrap()).await;
                                });
                            }
                        }
                    />
                    <span class={
                        stylance::classes!(
                            program_style::slider,
                            program_style::round
                        )
                    }></span>
                </label>
            </div>
        </div>
    };

}