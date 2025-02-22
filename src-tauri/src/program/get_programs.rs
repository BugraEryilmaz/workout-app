use crate::models::Program;
use crate::schema::*;
use crate::utils::establish_connection;

use diesel::prelude::*;

#[tauri::command]
pub async fn get_programs(app: tauri::AppHandle) -> Vec<Program> {
    let conn = &mut establish_connection(&app);
    let programs = programs::table
        .load::<Program>(conn)
        .expect("Error loading programs");
    programs
}
