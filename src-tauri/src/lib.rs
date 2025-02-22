pub mod models;
pub mod schema;
pub mod utils;
mod program;
mod workout;
mod day;

use program::*;
use workout::*;
use day::*;
use utils::establish_connection;

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations/");

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
            let _ = conn.run_pending_migrations(MIGRATIONS);

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
