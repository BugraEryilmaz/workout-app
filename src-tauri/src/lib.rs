pub mod models;
pub mod schema;

use diesel::prelude::*;
use models::*;
use schema::*;
use tauri::Manager;
use tauri_plugin_opener::open_url;


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
fn establish_connection(app_handle: &tauri::AppHandle) -> SqliteConnection {
    let path = app_handle
        .path()
        .data_dir()
        .unwrap()
        .join("workout-app/")
        .join("workouts.db");
    SqliteConnection::establish(path.to_str().unwrap()).expect("Error connecting to database")
}

#[tauri::command]
async fn get_workouts_of(date: chrono::NaiveDate , app_handle: tauri::AppHandle) -> Vec<Workout> {
    let conn = &mut establish_connection(&app_handle);
    let today = chrono::Local::now().date_naive();
    // Check if there is a workout finished at date for the active programs
    let workouts: Vec<Workout> = programs::table
        .inner_join(days::table.inner_join(workouts::table))
        .filter(programs::dsl::active.eq(true).and(days::dsl::complete_date.eq(Some(date.to_string()))))
        .select(workouts::all_columns)
        .load::<Workout>(conn)
        .expect("Error loading workouts");
    if workouts.len() == 0 {
        // If there is no workout finished at date, estimate the workouts for the day
        //   1. Get the difference between today and date diff, if it is in the past, return empty
        //   2. Get the unfinished days of the active programs, sort them by day_number and get the diff'th day
        //   3. Get the workouts of the day
        let diff = date.signed_duration_since(today).num_days();
        if diff < 0 {
            return vec![];
        }
        let day_id = days::table
            .inner_join(programs::table)
            .filter(programs::dsl::active.eq(true).and(days::dsl::done.eq(false)))
            .order_by(days::dsl::day_number)
            .limit(1)
            .offset(diff as i64)
            .select(days::id)
            .first::<i32>(conn);

        if day_id.is_err() {
            return vec![];
        }
        let day_id = day_id.unwrap();

        let workouts = workouts::table
            .filter(workouts::dsl::day_id.eq(day_id))
            .load::<Workout>(conn)
            .expect("Error loading workouts");

        return workouts;
    }
    return workouts;
}

#[tauri::command]
fn open(workout: Workout, app_handle: tauri::AppHandle) {
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
    .filter(workouts::dsl::day_id.eq(workout.day_id).and(workouts::dsl::done.eq(false)))
    .first::<Workout>(&mut establish_connection(&app_handle));
    if workouts.is_err() {
        diesel::update(days::dsl::days)
            .filter(days::dsl::id.eq(workout.day_id))
            .set((
                days::dsl::done.eq(true),
                days::dsl::complete_date.eq(Some(chrono::Local::now().date_naive().to_string()))
            ))
            .execute(&mut establish_connection(&app_handle))
            .expect("Error updating day");
    }

}

#[tauri::command]
async fn get_programs(app: tauri::AppHandle) -> Vec<Program> {
    let conn = &mut establish_connection(&app);
    let programs = programs::table
        .load::<Program>(conn)
        .expect("Error loading programs");
    programs
}

#[tauri::command]
async fn activate_program(program: Program, app: tauri::AppHandle) {
    let conn = &mut establish_connection(&app);
    diesel::update(programs::dsl::programs)
        .set(programs::dsl::active.eq(false))
        .execute(conn)
        .expect("Error updating programs");
    diesel::update(programs::dsl::programs)
        .filter(programs::dsl::id.eq(program.id))
        .set(programs::dsl::active.eq(true))
        .execute(conn)
        .expect("Error updating programs");
}

#[tauri::command]
async fn deactivate_program(program: Program, app: tauri::AppHandle) {
    let conn = &mut establish_connection(&app);
    diesel::update(programs::dsl::programs)
        .filter(programs::dsl::id.eq(program.id))
        .set(programs::dsl::active.eq(false))
        .execute(conn)
        .expect("Error updating programs");
}

#[tauri::command]
async fn clear_progress(program: Program, app: tauri::AppHandle) {
    let conn = &mut establish_connection(&app);
    diesel::update(days::dsl::days)
        .filter(days::dsl::program_id.eq(program.id))
        .set((
            days::dsl::done.eq(false),
            days::dsl::complete_date.eq(None::<String>)
        ))
        .execute(conn)
        .expect("Error updating days");
    diesel::update(workouts::dsl::workouts)
        .filter(workouts::dsl::day_id.eq_any(
            days::table
                .filter(days::dsl::program_id.eq(program.id))
                .select(days::dsl::id)
        ))
        .set(workouts::dsl::done.eq(false))
        .execute(conn)
        .expect("Error updating workouts");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let path = app
                .path()
                .data_dir()
                .unwrap()
                .join("workout-app/")
                .join("workouts.db");
            if !path.exists() {
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            }
            let mut conn = establish_connection(app.handle());
            // create the table if it does not exist
            diesel::sql_query(
                "CREATE TABLE IF NOT EXISTS workouts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            link TEXT NOT NULL,
            title TEXT NOT NULL,
            duration INTEGER NOT NULL,
            date DATE NOT NULL
        )",
            )
            .execute(&mut conn)
            .expect("Error creating table");
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_workouts_of, open, get_programs,
            activate_program, deactivate_program, clear_progress
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
