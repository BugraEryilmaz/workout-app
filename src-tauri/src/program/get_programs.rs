use crate::models::{Achievement, Program};
use crate::schema::*;
use crate::utils::establish_connection;

use diesel::prelude::*;

#[tauri::command]
pub async fn get_programs(app: tauri::AppHandle) -> Vec<Program> {
    let conn = &mut establish_connection(&app);
    let programs = programs::table
        .filter(programs::dsl::deleted.eq(false))
        .order(programs::dsl::id.asc())
        .load::<Program>(conn)
        .expect("Error loading programs");
    programs
}

#[tauri::command]
pub async fn get_achievements(app: tauri::AppHandle) -> Vec<(Program, Achievement)> {
    let conn = &mut establish_connection(&app);
    let achievements = achievements::table
        .inner_join(programs::table.on(programs::dsl::id.eq(achievements::dsl::program_id)))
        .order(achievements::dsl::id.asc())
        .select((programs::all_columns, achievements::all_columns))
        .load::<(Program, Achievement)>(conn)
        .expect("Error loading achievements");
    achievements
}