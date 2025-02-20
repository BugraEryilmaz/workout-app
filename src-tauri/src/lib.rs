pub mod models;
pub mod schema;

use chrono::NaiveDate;
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
        let (mut program_id, last_done_day_number, last_done_date) = days::table
            .inner_join(programs::table)
            .filter(programs::dsl::active.eq(true).and(days::dsl::done.eq(true)))
            .order_by(days::dsl::day_number.desc())
            .limit(1)
            .select((days::program_id, days::day_number, days::complete_date))
            .first::<(i32, i32, Option<String>)>(conn)
            .optional()
            .expect("Error getting last done day")
            .unwrap_or((-1, 0, Some((today - chrono::Duration::days(1)).to_string())));
        if program_id == -1 {
            program_id = programs::table
                .filter(programs::dsl::active.eq(true))
                .select(programs::dsl::id)
                .first::<i32>(conn)
                .optional()
                .expect("Error getting active program")
                .unwrap_or(-1);
            if program_id == -1 {
                return vec![];
            }
        }
        let last_done_date = last_done_date.unwrap().parse::<NaiveDate>().unwrap();
        let diff = date.signed_duration_since(last_done_date).num_days();
        if diff <= 0 {
            return vec![];
        }
        let day_number = last_done_day_number + diff as i32;

        let workouts = workouts::table
            .inner_join(days::table)
            .filter(days::dsl::program_id.eq(program_id).and(days::dsl::day_number.eq(day_number)))
            .select(workouts::all_columns)
            .load::<Workout>(conn)
            .expect("Error loading workouts");

        return workouts;
    }
    return workouts;
}

fn complete_day(day_id: i32, finish_date: NaiveDate, conn: &mut SqliteConnection) {
    diesel::update(days::dsl::days)
        .filter(days::dsl::id.eq(day_id))
        .set((
            days::dsl::done.eq(true),
            days::dsl::complete_date.eq(Some(finish_date.to_string()))
        ))
        .execute(conn)
        .expect("Error updating day");
    // Check next day if it is rest day
    // Get current day's program_id and day_number
    let (current_day_program_id, current_day_number) = days::table
        .filter(days::dsl::id.eq(day_id))
        .select((days::dsl::program_id, days::dsl::day_number))
        .first::<(i32,i32)>(conn).expect("Error getting current day");
    // Get next day
    let next_day_number = current_day_number + 1;
    let next_day_id = days::table
        .filter(days::dsl::program_id.eq(current_day_program_id).and(days::dsl::day_number.eq(next_day_number)))
        .select(days::dsl::id)
        .first::<i32>(conn).optional().expect("Error getting next day");
    // If there is no next day, nothing to do
    if next_day_id.is_none() {
        return;
    }
    let next_day_id = next_day_id.unwrap();
    // If next day is rest day, complete it
    let workouts_of_next_day = workouts::table
        .filter(workouts::dsl::day_id.eq(next_day_id))
        .load::<Workout>(conn)
        .expect("Error loading workouts of next day");
    if workouts_of_next_day.len() == 0 {
        complete_day(
            next_day_id,
            finish_date + chrono::Duration::days(1),
            conn
        );
    }
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
        complete_day(workout.day_id, chrono::Local::now().date_naive(), &mut establish_connection(&app_handle));
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

#[tauri::command]
async fn last_workouts(app: tauri::AppHandle) -> Vec<String> {
    let conn = &mut establish_connection(&app);
    let workouts = workouts::table
        .order(workouts::dsl::id.desc())
        .limit(10)
        .distinct()
        .select(workouts::dsl::link)
        .load::<String>(conn)
        .expect("Error loading workouts");
    workouts
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
            activate_program, deactivate_program, clear_progress,
            last_workouts
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
