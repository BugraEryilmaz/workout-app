use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use tauri_plugin_dialog::DialogExt;

use crate::models::*;
use crate::schema::*;
use crate::utils::establish_connection;


#[derive(Serialize, Deserialize)]
struct ProgramWithWorkouts {
    program: Program,
    days: Vec<(Day, Workout)>,
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
    let json = serde_json::to_string(&ProgramWithWorkouts { program, days })
        .expect("Error serializing program");
    // Save this json to a file
    let path = app
        .dialog()
        .file()
        .blocking_save_file();
    if let Some(path) = path {
        let path = path.as_path().unwrap();
        std::fs::write
            (path, json)
            .expect("Error writing to file");
    }
}

#[tauri::command]
pub async fn restore_program(app: tauri::AppHandle) -> Option<Program> {
    let path = app
        .dialog()
        .file()
        .blocking_pick_file();
    if let Some(path) = path {
        let path = path.as_path().unwrap();
        let json = std::fs::read_to_string(path).expect("Error reading file");
        let program_with_workouts: ProgramWithWorkouts =
            serde_json::from_str(&json).expect("Error deserializing program");
        let conn = &mut establish_connection(&app);
        let new_program = diesel::insert_into(programs::table)
            .values(programs::dsl::title.eq(program_with_workouts.program.title))
            .returning(programs::all_columns)
            .get_result::<Program>(conn)
            .expect("Error inserting new program");
        // keep a hashmap of days to insert days only once to the program
        let mut days_map = std::collections::HashMap::new();
        let last_day_number = program_with_workouts.days.get(program_with_workouts.days.len()-1).unwrap().0.day_number.unwrap();
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
            let day_id = days_map
                .entry(day.day_number.unwrap()).or_default();
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
