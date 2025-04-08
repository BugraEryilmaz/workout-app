use diesel::prelude::*;
use image::ImageReader;
use serde::{Deserialize, Serialize};
use tauri_plugin_dialog::DialogExt;

use crate::models::*;
use crate::schema::*;
use crate::utils::establish_connection;

use super::upload_image;

#[derive(Serialize, Deserialize)]
struct ProgramBackwardCompatible {
    pub id: i32,
    pub title: String,
    pub active: bool,
    pub image: Option<String>,
    pub deleted: bool,
    pub info: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ProgramWithWorkouts {
    program: ProgramBackwardCompatible,
    days: Vec<(Day, Workout)>,
    image: Option<Vec<u8>>,
}

#[tauri::command]
pub async fn share_program(programid: i32, app: tauri::AppHandle) {
    let conn = &mut establish_connection(&app);
    let program = programs::table
        .filter(programs::dsl::id.eq(programid))
        .first::<Program>(conn)
        .expect("Error getting program");
    let days = days::table
        .inner_join(workouts::table)
        .filter(days::dsl::program_id.eq(programid))
        .order((days::dsl::day_number, workouts::dsl::id))
        .select((days::all_columns, workouts::all_columns))
        .load::<(Day, Workout)>(conn)
        .expect("Error loading days");
    let image = program.image.clone().map(|img| 
        ImageReader::open(&img)
            .expect("Error reading image")
            .decode()
            .expect("Error decoding image")
    );
    let image = image.as_ref().map(|img| {
        let mut image_data: Vec<u8> = Vec::new();
        img.to_rgb8().write_to(&mut std::io::Cursor::new(&mut image_data), image::ImageFormat::Jpeg)
            .expect("Error writing image");
        image_data
    });

    let program = ProgramBackwardCompatible {
        id: program.id,
        title: program.title,
        active: program.active,
        image: program.image,
        deleted: program.deleted,
        info: Some(program.info),
        created_at: Some(program.created_at),
    };

    let json = serde_json::to_string(&ProgramWithWorkouts { program, days, image })
        .expect("Error serializing program");
    // Save this json to a file
    let path = app.dialog().file().blocking_save_file();
    if let Some(path) = path {
        let path = path.as_path().unwrap();
        std::fs::write(path, json).expect("Error writing to file");
    }
}

#[tauri::command]
pub async fn restore_program(app: tauri::AppHandle) -> Option<Program> {
    let path = app.dialog().file().blocking_pick_file();
    if let Some(path) = path {
        let path = path.as_path().unwrap();
        let json = std::fs::read_to_string(path).expect("Error reading file");
        let program_with_workouts: ProgramWithWorkouts =
            serde_json::from_str(&json).expect("Error deserializing program");
        if program_with_workouts.days.len() == 0 {
            return None;
        }
        let conn = &mut establish_connection(&app);
        let image_path = if let Some(image) = program_with_workouts.image {
            let mut decoder = ImageReader::new(std::io::Cursor::new(image));
            decoder.set_format(image::ImageFormat::Jpeg);
            let image = decoder
                .decode()
                .expect("Error decoding image");
                
            Some(upload_image(&app, image))
        } else {
            None
        };
        let today = chrono::Utc::now().naive_utc().date().to_string();
        let new_program = diesel::insert_into(programs::table)
            .values((programs::dsl::title.eq(program_with_workouts.program.title), 
                    programs::dsl::image.eq(image_path),
                    programs::dsl::created_at.eq(program_with_workouts.program.created_at.unwrap_or(today)),
                    programs::dsl::info.eq(program_with_workouts.program.info.unwrap_or_default()),))
            .returning(programs::all_columns)
            .get_result::<Program>(conn)
            .expect("Error inserting new program");
        // upload the image if it exists
        // keep a hashmap of days to insert days only once to the program
        let mut days_map = std::collections::HashMap::new();
        let last_day_number = program_with_workouts
            .days
            .get(program_with_workouts.days.len() - 1)
            .unwrap()
            .0
            .day_number
            .unwrap();
        for day_number in 1..=last_day_number {
            let day_id = diesel::insert_into(days::table)
                .values((
                    days::dsl::program_id.eq(new_program.id),
                    days::dsl::day_number.eq(day_number),
                    days::dsl::done.eq(false),
                    days::dsl::complete_date.eq(None::<String>),
                ))
                .returning(days::dsl::id)
                .get_result::<i32>(conn)
                .expect("Error inserting new day");
            days_map.insert(day_number, day_id);
        }
        for (day, workout) in program_with_workouts.days {
            let day_id = days_map.entry(day.day_number.unwrap()).or_default();
            diesel::insert_into(workouts::table)
                .values((
                    workouts::dsl::link.eq(workout.link),
                    workouts::dsl::title.eq(workout.title),
                    workouts::dsl::duration.eq(workout.duration),
                    workouts::dsl::day_id.eq(*day_id),
                    workouts::dsl::done.eq(workout.done),
                ))
                .execute(conn)
                .expect("Error inserting new workout");
        }
        return Some(new_program);
    }
    None
}
