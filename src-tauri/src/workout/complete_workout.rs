use diesel::prelude::*;
use tauri_plugin_opener::open_url;

use crate::models::Workout;
use crate::schema::*;
use crate::utils::{complete_day, establish_connection};


#[tauri::command]
pub async fn open(workout: Workout, app_handle: tauri::AppHandle) {
    let url = workout.link.as_str();
    println!("Opening url: {}", url);
    open_url(url, None::<&str>).unwrap_or_else(|e| {
        eprintln!("Error opening url: {}", e);
    });
    diesel::update(workouts::dsl::workouts)
        .filter(workouts::dsl::id.eq(workout.id))
        .set(workouts::dsl::done.eq(true))
        .execute(&mut establish_connection(&app_handle))
        .expect("Error updating workout");

    let workouts = workouts::table
        .filter(
            workouts::dsl::day_id
                .eq(workout.day_id)
                .and(workouts::dsl::done.eq(false)),
        )
        .first::<Workout>(&mut establish_connection(&app_handle));
    if workouts.is_err() {
        complete_day(
            workout.day_id,
            chrono::Local::now().date_naive(),
            &mut establish_connection(&app_handle),
        );
    }
}
