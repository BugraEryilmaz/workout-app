use chrono::NaiveDate;
use leptos::{leptos_dom::logging::console_log, prelude::*, task::spawn_local};
use serde_wasm_bindgen::to_value;

use crate::utils::invoke::invoke;
use crate::utils::models::Program;

stylance::import_style!(#[allow(dead_code)] pub program_style, "program.css");


#[derive(serde::Serialize, Debug)]
struct UpdateProgramImageArgs {
    programid: i32,
}

#[derive(serde::Serialize, Debug)]
struct GetProgramImageArgs {
    imagepath: String,
}


#[component]
pub fn program_img(image: RwSignal<Option<String>>, program: Program, program_to_update: RwSignal<Option<Program>>) -> impl IntoView {
    let image_b64 = RwSignal::new(None);
    let get_image = move || {
        if let Some(img) = image.get() {
            spawn_local(async move {
                let arg = GetProgramImageArgs { imagepath: img.clone() };
                let img_b64 = invoke("get_program_image", to_value(&arg).unwrap()).await;
                let img_b64: Option<String> = serde_wasm_bindgen::from_value(img_b64).unwrap();
                image_b64.set(img_b64);
            });
        }
    };
    let date = NaiveDate::parse_from_str(&program.created_at, "%Y-%m-%d").unwrap();
    let date_str = date.format("%d %b %y").to_string();
    get_image();
    view! {
        <div class=stylance::classes!(program_style::program_image_container)>
            <img src={move || if image_b64.get().is_none() {"public/no_image.svg".to_string()} else {image_b64.get().unwrap_or_default()}}
                class=stylance::classes!(program_style::program_image)
                alt="program image"
                on:click={
                    let program_clone = program.clone();
                    move |_| {
                    program_to_update.set(Some(program_clone.clone()));
                }}
            />
            <div class=stylance::classes!(program_style::program_image_overlay)
                on:click={
                    let program_clone = program.clone();
                    move |e| {
                    if let Some(target) = e.target() {
                        if let Some(current_target) = e.current_target() {
                            if target == current_target {
                                program_to_update.set(Some(program_clone.clone()));
                            }
                        }
                    }
                }}
            >
                <img src="public/image_up.svg" class=stylance::classes!(program_style::program_image_icon) alt="program image icon"
                    on:click={
                        let program_clone = program.clone();
                        move |_| {
                        let arg = UpdateProgramImageArgs { programid: program_clone.id };
                        spawn_local(async move {
                            console_log(&format!("Updating program image: {:?}", arg));
                            let img_path = invoke("update_program_image", to_value(&arg).unwrap()).await;
                            let img_path: Option<String> = serde_wasm_bindgen::from_value(img_path).unwrap();
                            console_log(&format!("Updated program image: {:?}", img_path));
                            image.set(img_path);
                            get_image();
                        });
                    }}
                 />
                <p> { date_str } </p>
            </div>
        </div>
    }
}