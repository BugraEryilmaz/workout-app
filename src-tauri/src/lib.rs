mod day;
pub mod models;
mod program;
pub mod schema;
pub mod utils;
mod workout;

use day::*;
use program::*;
use tauri_plugin_updater::UpdaterExt;
use utils::establish_connection;
use workout::*;

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations/");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
              update(handle).await.unwrap();
            });      
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

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
      let mut downloaded = 0;
  
      // alternatively we could also call update.download() and update.install() separately
      update
        .download_and_install(
          |chunk_length, content_length| {
            downloaded += chunk_length;
            println!("downloaded {downloaded} from {content_length:?}");
          },
          || {
            println!("download finished");
          },
        )
        .await?;
  
      println!("update installed");
      app.restart();
    }
  
    Ok(())
  }
  