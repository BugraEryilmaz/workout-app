use leptos::leptos_dom::logging::console_log;
use leptos::task::spawn_local;
use leptos::{html, prelude::*};
use serde_wasm_bindgen::to_value;

use crate::utils::invoke::invoke;
use crate::utils::models::Program;

stylance::import_style!(#[allow(dead_code)] pub program_style, "program.css");

#[derive(serde::Serialize)]
pub struct ProgramAction {
    pub program: Program,
}

#[derive(serde::Serialize)]
struct DeleteProgramArgs {
    programid: i32,
}

#[derive(serde::Serialize)]
struct UpdateProgramArgs {
    id: i32,
    title: String,
}

#[component]
pub fn ProgramCard(
    program: Program,
    set_action: RwSignal<Option<Program>>,
    programs: WriteSignal<Vec<Program>>,
) -> impl IntoView {
    let title_input: NodeRef<html::Div> = NodeRef::new();
    let form_ref: NodeRef<html::Form> = NodeRef::new();
    return view! {
        <div
            class=stylance::classes!(
                    program_style::program
        )>
            <form
                class=program_style::program_title_form
                on:submit={
                    let program_clone = program.clone();
                    move |event| {
                    event.prevent_default();
                    console_log(&format!("Submitting form: {:?}", program_clone));
                    let title = title_input.get().unwrap().inner_text();
                    let arg = UpdateProgramArgs { id: program_clone.id, title: title };
                    spawn_local(async move {
                        let program = invoke("update_program", to_value(&arg).unwrap()).await;
                        let program: Program = serde_wasm_bindgen::from_value(program).unwrap();
                        programs.update(|programs| {
                            programs.iter_mut().find(|p| p.id == program_clone.id).map(|p| {
                                *p = program.clone();
                            });
                        });
                    });
                }}
                node_ref=form_ref
                on:focusout=move |e| {
                    // submit form on focus out
                    e.prevent_default();
                    let _ = form_ref.get().unwrap().request_submit();
                }
            >
                <div
                    class=program_style::program_title
                    node_ref=title_input
                    contenteditable=true
                    on:focusout=move |e| {
                        // submit form on focus out
                        e.prevent_default();
                        let _ = form_ref.get().unwrap().request_submit();
                    }
                    on:keypress=move |e| {
                        if e.key() == "Enter" {
                            e.prevent_default();
                            let _ = form_ref.get().unwrap().request_submit();
                        }
                    }
                >
                    { program.title.clone() }
                </div>
            </form>
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
                        on:input:target={
                            let program_clone = program.clone();
                            move |event| {
                            event.stop_propagation();
                            let target = event.target().checked();
                            if target {
                                event.target().set_checked(!target);
                                set_action.set(Some(program_clone.clone()));
                            } else {
                                let program_clone = program_clone.clone();
                                spawn_local(async move {
                                    // console_log(&format!("Deactivating program: {:?}", to_value(&program_clone)));
                                    invoke("deactivate_program", to_value(&ProgramAction {
                                        program: program_clone
                                    }).unwrap()).await;
                                });
                            }
                        }}
                    />
                    <span class={
                        stylance::classes!(
                            program_style::slider,
                            program_style::round
                        )
                    }></span>
                </label>
                <i class=stylance::classes!("material-symbols", program_style::share_icon)
                    on:click={
                        let program_clone = program.clone();
                        move |_| {
                        spawn_local(async move {
                            let arg = DeleteProgramArgs { programid: program_clone.id };
                            invoke("share_program", serde_wasm_bindgen::to_value(&arg).unwrap()).await;
                        });
                    }}
                    style="cursor: pointer; margin: 0.5em;"
                >"ios_share"</i>
                <i class=stylance::classes!(program_style::delete_icon, "material-symbols")
                    on:click={
                        let program_clone = program.clone();
                        move |_| {
                        spawn_local(async move {
                            let arg = DeleteProgramArgs { programid: program_clone.id };
                            invoke("delete_program", serde_wasm_bindgen::to_value(&arg).unwrap()).await;
                        });
                        programs.update(|programs| {
                            programs.retain(|p| p.id != program_clone.id);
                        });
                    }}
                >"delete"</i>
            </div>
        </div>
    };
}
