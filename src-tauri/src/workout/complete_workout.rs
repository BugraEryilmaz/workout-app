use diesel::prelude::*;
use tauri_plugin_opener::open_url;

use crate::models::{Day, Workout};
use crate::schema::*;
use crate::utils::{complete_day, establish_connection};

#[tauri::command]
pub async fn open(workout: Workout, app_handle: tauri::AppHandle) -> Option<Day> {
    let url = workout.link.as_str();
    println!("Opening url: {}", url);
    open_url(url, None::<&str>).unwrap_or_else(|e| {
        eprintln!("Error opening url: {}", e);
    });
    diesel::update(workouts::dsl::workouts)
        .filter(workouts::dsl::id.eq(workout.id))
        .set((
            workouts::dsl::done.eq(true),
            workouts::dsl::done_date.eq(chrono::Local::now().naive_local().date().to_string()),
        ))
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
    let day = days::table
        .filter(days::dsl::id.eq(workout.day_id))
        .first::<Day>(&mut establish_connection(&app_handle))
        .expect("Error getting day");
    Some(day)
}
