use diesel::prelude::*;

use crate::models::Workout;
use crate::schema::*;
use crate::utils::establish_connection;

#[tauri::command]
pub async fn delete_workout(workoutid: i32, app: tauri::AppHandle) {
    let conn = &mut establish_connection(&app);
    diesel::delete(workouts::dsl::workouts.filter(workouts::dsl::id.eq(workoutid)))
        .execute(conn)
        .expect("Error deleting workout");
}

#[tauri::command]
pub async fn create_workout(workout: Workout, app: tauri::AppHandle) -> Workout {
    let conn = &mut establish_connection(&app);
    // Get the day_id and create a new day if it does not exist
    let new_workout = diesel::insert_into(workouts::table)
        .values((
            workouts::dsl::link.eq(workout.link),
            workouts::dsl::title.eq(workout.title),
            workouts::dsl::duration.eq(workout.duration),
            workouts::dsl::day_id.eq(workout.day_id),
        ))
        .returning(workouts::all_columns)
        .get_result(conn)
        .expect("Error inserting new workout");
    new_workout
}

#[tauri::command]
pub async fn update_workout(workoutid: i32, title: String, app: tauri::AppHandle) -> Workout {
    let conn = &mut establish_connection(&app);
    let updated_workout =
        diesel::update(workouts::dsl::workouts.filter(workouts::dsl::id.eq(workoutid)))
            .set(workouts::dsl::title.eq(title))
            .get_result(conn)
            .expect("Error updating workout");
    updated_workout
}
