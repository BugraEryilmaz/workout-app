pub mod models;
pub mod schema;

use chrono::NaiveDate;
use diesel::prelude::*;
use models::*;
use schema::*;
use serde::{Deserialize, Serialize};
use tauri::{window, Manager};
use tauri_plugin_opener::open_url;
use tauri_plugin_dialog::DialogExt;

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
async fn get_workouts_date(date: chrono::NaiveDate, app_handle: tauri::AppHandle) -> Vec<Workout> {
    let conn = &mut establish_connection(&app_handle);
    let today = chrono::Local::now().date_naive();
    // Check if there is a workout finished at date for the active programs
    let workouts: Vec<Workout> = programs::table
        .inner_join(days::table.inner_join(workouts::table))
        .filter(
            programs::dsl::active
                .eq(true)
                .and(days::dsl::complete_date.eq(Some(date.to_string()))),
        )
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
            .first::<(i32, Option<i32>, Option<String>)>(conn)
            .optional()
            .expect("Error getting last done day")
            .unwrap_or((
                -1,
                Some(0),
                Some((today - chrono::Duration::days(1)).to_string()),
            ));
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
        // day_number = last_done_day_number + (date-today) + (min(today, last_done_date + 1)-last_done_date)
        // To adjust for skipped days, we have the min part
        let day_number = last_done_day_number.unwrap()
            + (diff + (today - last_done_date).num_days().min(1)) as i32;

        let workouts = workouts::table
            .inner_join(days::table)
            .filter(
                days::dsl::program_id
                    .eq(program_id)
                    .and(days::dsl::day_number.eq(day_number)),
            )
            .select(workouts::all_columns)
            .load::<Workout>(conn)
            .expect("Error loading workouts");

        return workouts;
    }
    return workouts;
}

#[tauri::command]
async fn get_workouts_day(daynumber: i32, programid: i32, app: tauri::AppHandle) -> Vec<Workout> {
    let conn = &mut establish_connection(&app);
    let workouts = workouts::table
        .inner_join(days::table)
        .filter(
            days::dsl::program_id
                .eq(programid)
                .and(days::dsl::day_number.eq(daynumber)),
        )
        .select(workouts::all_columns)
        .load::<Workout>(conn)
        .expect("Error loading workouts");
    workouts
}

fn complete_day(day_id: i32, finish_date: NaiveDate, conn: &mut SqliteConnection) {
    diesel::update(days::dsl::days)
        .filter(days::dsl::id.eq(day_id))
        .set((
            days::dsl::done.eq(true),
            days::dsl::complete_date.eq(Some(finish_date.to_string())),
        ))
        .execute(conn)
        .expect("Error updating day");
    // Check next day if it is rest day
    // Get current day's program_id and day_number
    let (current_day_program_id, current_day_number) = days::table
        .filter(days::dsl::id.eq(day_id))
        .select((days::dsl::program_id, days::dsl::day_number.nullable()))
        .first::<(i32, Option<i32>)>(conn)
        .expect("Error getting current day");
    // Get next day
    let next_day_number = current_day_number.unwrap() + 1;
    let next_day_id = days::table
        .filter(
            days::dsl::program_id
                .eq(current_day_program_id)
                .and(days::dsl::day_number.eq(next_day_number)),
        )
        .select(days::dsl::id)
        .first::<i32>(conn)
        .optional()
        .expect("Error getting next day");
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
        complete_day(next_day_id, finish_date + chrono::Duration::days(1), conn);
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
            days::dsl::complete_date.eq(None::<String>),
        ))
        .execute(conn)
        .expect("Error updating days");
    diesel::update(workouts::dsl::workouts)
        .filter(
            workouts::dsl::day_id.eq_any(
                days::table
                    .filter(days::dsl::program_id.eq(program.id))
                    .select(days::dsl::id),
            ),
        )
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

#[tauri::command]
async fn create_workout(workout: Workout, app: tauri::AppHandle) -> Workout {
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
async fn create_day(programid: i32, app: tauri::AppHandle) -> i32 {
    let conn = &mut establish_connection(&app);
    let day_id = diesel::insert_into(days::table)
        .values(days::dsl::program_id.eq(programid))
        .returning(days::dsl::id)
        .get_result::<i32>(conn)
        .expect("Error inserting new day");
    day_id
}

#[tauri::command]
async fn get_day_ids(programid: i32, app: tauri::AppHandle) -> Vec<i32> {
    let conn = &mut establish_connection(&app);
    let day_ids = days::table
        .filter(days::dsl::program_id.eq(programid))
        .select(days::dsl::id)
        .load::<i32>(conn)
        .expect("Error loading day ids");
    day_ids
}

#[tauri::command]
async fn delete_workout(workoutid: i32, app: tauri::AppHandle) {
    let conn = &mut establish_connection(&app);
    diesel::delete(workouts::dsl::workouts.filter(workouts::dsl::id.eq(workoutid)))
        .execute(conn)
        .expect("Error deleting workout");
}

#[tauri::command]
async fn create_program(title: String, app: tauri::AppHandle) -> Program {
    let conn = &mut establish_connection(&app);
    let new_program = diesel::insert_into(programs::table)
        .values(programs::dsl::title.eq(title))
        .returning(programs::all_columns)
        .get_result(conn)
        .expect("Error inserting new program");

    new_program
}

#[tauri::command]
async fn delete_program(programid: i32, app: tauri::AppHandle) {
    let conn = &mut establish_connection(&app);
    diesel::delete(programs::dsl::programs.filter(programs::dsl::id.eq(programid)))
        .execute(conn)
        .expect("Error deleting program");
}

#[derive(Serialize, Deserialize)]
struct ProgramWithWorkouts {
    program: Program,
    days: Vec<(Day, Workout)>,
}

#[tauri::command]
async fn share_program(programid: i32, app: tauri::AppHandle) {
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
async fn restore_program(app: tauri::AppHandle) -> Option<Program> {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
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
                std::fs::File::create(&path).unwrap();
            }
            let mut conn = establish_connection(app.handle());
            // create the table if it does not exist
            diesel::sql_query("
            CREATE TABLE IF NOT EXISTS programs (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                title VARCHAR(255) NOT NULL,
                active BOOLEAN NOT NULL DEFAULT FALSE,
                image VARCHAR(255)
            );")
            .execute(&mut conn)
            .expect("Error creating table");

            diesel::sql_query("
            CREATE TABLE IF NOT EXISTS days (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                program_id INTEGER NOT NULL,
                done BOOLEAN NOT NULL DEFAULT FALSE,
                complete_date DATE,
                day_number INTEGER,
                FOREIGN KEY (program_id) REFERENCES programs(id) ON DELETE CASCADE,
                UNIQUE (program_id, day_number)
            );
            ")
            .execute(&mut conn)
            .expect("Error creating table");

            diesel::sql_query("
            CREATE TRIGGER IF NOT EXISTS auto_increment_trigger
            AFTER INSERT ON days
            WHEN new.day_number IS NULL
            BEGIN
                UPDATE days
                SET day_number = (SELECT IFNULL(MAX(day_number), 0) + 1 FROM days WHERE program_id = new.program_id)
                WHERE id = new.id;
            END;")
            .execute(&mut conn)
            .expect("Error creating table");

            diesel::sql_query("
            CREATE TABLE IF NOT EXISTS workouts (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                link VARCHAR(255) NOT NULL,
                title VARCHAR(255) NOT NULL,
                duration INT NOT NULL,
                done BOOLEAN NOT NULL DEFAULT FALSE,
                day_id INTEGER NOT NULL,
                FOREIGN KEY (day_id) REFERENCES days(id) ON DELETE CASCADE
            );")
            .execute(&mut conn)
            .expect("Error creating table");
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_workouts_date,
            open,
            get_programs,
            activate_program,
            deactivate_program,
            clear_progress,
            last_workouts,
            create_workout,
            create_day,
            get_day_ids,
            get_workouts_day,
            delete_workout,
            create_program,
            delete_program,
            share_program,
            restore_program,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
