use diesel::{Connection, SqliteConnection};
use tauri::Manager;



pub fn establish_connection(app_handle: &tauri::AppHandle) -> SqliteConnection {
    let path = app_handle
        .path()
        .data_dir()
        .unwrap()
        .join("workout-app/")
        .join("workouts.db");
    SqliteConnection::establish(path.to_str().unwrap()).expect("Error connecting to database")
}
