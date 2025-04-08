use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use image::{save_buffer, ImageFormat};
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use base64::{engine::general_purpose, Engine as _};
use image::DynamicImage;
use std::io::Cursor;


use crate::{models::Program, schema::programs, utils::establish_connection};

pub fn upload_image(
    app: &tauri::AppHandle,
    image: DynamicImage,
) -> String {
    let image_path = format!(
        "{}/workout-app/{}.jpeg",
        app.path().data_dir().unwrap().to_str().unwrap(),
        uuid::Uuid::new_v4().to_string()
    );
    let image_x = image.width() as i32;
    let image_y = image.height() as i32;
    let image = image.to_rgb8().into_raw();
    save_buffer(
        &image_path,
        &image,
        image_x as u32,
        image_y as u32,
        image::ColorType::Rgb8,
    )
    .expect("Error saving image");
    image_path
}

#[tauri::command]
pub async fn update_program_image(
    app: tauri::AppHandle,
    programid: i32,
) -> Option<String> {
    println!("Updating program image for program id: {}", programid);
    let img_path = app.dialog()
        .file()
        .blocking_pick_file()
        .map(|path| {
            let image = image::open(path.as_path().unwrap()).expect("Error opening image");
            upload_image(&app, image)
        });
    let conn = &mut establish_connection(&app);
    let old_program: Program = programs::dsl::programs
        .filter(programs::dsl::id.eq(programid))
        .get_result(conn)
        .expect("Error getting program");
    if img_path.is_some() {
        let _program = diesel::update(programs::table).filter(programs::dsl::id.eq(programid))
            .set(programs::image.eq(img_path.clone()))
            .get_result::<Program>(conn)
            .expect("Error updating program image");
        if let Some(old_image_path) = old_program.image {
            std::fs::remove_file(old_image_path).expect("Error deleting old program image");
        }
        img_path
    } else {
        old_program.image
    }
}

fn image_to_base64(img: &DynamicImage) -> String {
    let mut image_data: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut image_data), ImageFormat::Jpeg)
        .unwrap();
    let res_base64 = general_purpose::STANDARD.encode(image_data);
    format!("data:image/png;base64,{}", res_base64)
}

#[tauri::command]
pub async fn get_program_image(
    imagepath: String,
) -> String {
    let image = image::open(imagepath).expect("Error opening image");
    image_to_base64(&image)
}