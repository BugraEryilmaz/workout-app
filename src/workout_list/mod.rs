mod program;

use crate::utils::invoke::invoke;
use crate::utils::models::Program;
use leptos::task::spawn_local;
use leptos::{html, prelude::*};
use program::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;

stylance::import_style!(
    #[allow(dead_code)]
    workout_list_style,
    "workout_list.css"
);

#[derive(serde::Serialize)]
struct CreateProgramArgs {
    title: String,
}

#[component]
pub fn WorkoutList(program_to_update: RwSignal<Option<Program>>) -> impl IntoView {
    let (programs, set_programs) = signal(Vec::<Program>::new());
    let action: RwSignal<Option<Program>> = RwSignal::new(None);

    spawn_local(async move {
        let progs = invoke("get_programs", JsValue::null()).await;
        let progs: Vec<Program> = serde_wasm_bindgen::from_value(progs).unwrap();
        set_programs.set(progs);
    });

    let input_element: NodeRef<html::Input> = NodeRef::new();

    return view! {
        <div>
            <div class=workout_list_style::program_list>
                <For
                    each = { move || programs.get() }
                    key = { |program| program.id }
                    children = { move |program| {
                        view! {
                            <div>
                                <ProgramCard program=program.clone() set_action=action programs=set_programs
                                    on:click={move |e: web_sys::MouseEvent| {
                                        if let Some(target) = e.target() {
                                            if let Some(current_target) = e.current_target() {
                                                if current_target.eq(&target){
                                                    program_to_update.set(Some(program.clone()));
                                                }
                                            }
                                        }
                                    }}
                                />
                            </div>
                        }
                    }}
                />
            </div>
            <form
                class=workout_list_style::add_program_form
                on:submit={move |e| {
                    e.prevent_default();
                    let title = input_element.get().expect("The input needs to be loaded").value();
                    if title.is_empty() {
                        return;
                    }
                    input_element.get().expect("The input needs to be loaded").set_value("");
                    spawn_local(async move {
                        let arg = CreateProgramArgs { title };
                        let program = invoke("create_program", to_value(&arg).unwrap()).await;
                        let program: Program = serde_wasm_bindgen::from_value(program).unwrap();
                        set_programs.update(|progs| {
                            progs.push(program)
                        });
                    });
                }
            }>
                <input type="text" placeholder="Title" class=workout_list_style::add_program_textbox node_ref=input_element/>
                <button class=workout_list_style::add_program_button>
                    {"Add Program"}
                </button>
                <i class=stylance::classes!("material-symbols-outlined", workout_list_style::add_program_restore_button)
                    on:click={
                        move |_| {
                        spawn_local(async move {
                            let res = invoke("restore_program", JsValue::null()).await;
                            let res: Option<Program> = serde_wasm_bindgen::from_value(res).unwrap();
                            if let Some(program) = res {
                                set_programs.update(|progs| {
                                    progs.push(program)
                                });
                            }
                        });
                    }}
                    >"settings_backup_restore"</i>
            </form>

            <div
                class=workout_list_style::action_box
                style:display=move || if action.get().is_some() { "initial" } else { "none" }
            >
                <p> Bir programı aktive etmek diğerlerini iptal eder. İptal edilmiş programlar ilerlemelerini saklamaya devam eder. Bu programı aktifleştirmek istediğine emin misin? </p>
                <div
                    class=workout_list_style::action_buttons
                >
                    <button
                        on:click=move |_| {
                            let program = action.get().unwrap();
                            action.set(None);
                            program.active.set(true);
                            programs.get().iter().for_each(|p| {
                                if p.id != program.id {
                                    p.active.set(false);
                                }
                            });
                            spawn_local(async move {
                                invoke("activate_program", to_value(&ProgramAction {
                                    program: program.clone()
                                }).unwrap()).await;
                        });
                        }
                    > Kaldığım yerden aktifleştir </button>
                    <button
                        on:click=move |_| {
                            let program = action.get().unwrap();
                            action.set(None);
                            program.active.set(true);
                            programs.get().iter().for_each(|p| {
                                if p.id != program.id {
                                    p.active.set(false);
                                }
                            });
                            spawn_local(async move {
                                invoke("clear_progress", to_value(&ProgramAction {
                                    program: program.clone()
                                }).unwrap()).await;
                                invoke("activate_program", to_value(&ProgramAction {
                                    program: program.clone()
                                }).unwrap()).await;
                            });
                        }
                    > Sıfırdan aktifleştir </button>
                    <button
                        on:click=move |_| {
                            action.set(None);
                        }
                    > İptal </button>
                </div>
            </div>
        </div>
    };
}
