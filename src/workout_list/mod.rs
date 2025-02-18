mod program;

use program::*;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use crate::utils::models::Program;
use crate::utils::invoke::invoke;

stylance::import_style!(#[allow(dead_code)] workout_list_style, "workout_list.css");

#[component]
pub fn WorkoutList() -> impl IntoView {
    let (programs, set_programs) = signal(Vec::<Program>::new());
    let action: RwSignal<Option<Program>> = RwSignal::new(None);

    spawn_local(async move {
        let progs = invoke("get_programs", JsValue::null()).await;
        let progs: Vec<Program> = serde_wasm_bindgen::from_value(progs).unwrap();
        set_programs.set(progs);
    });

    return view! {
        <div>
            <For
                each = { move || programs.get() }
                key = { |program| program.id }
                children = { move |program| {
                    view! {
                        <ProgramCard program=program set_action=action/>
                    }
                }}
            />
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
    }
}