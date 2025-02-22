use diesel::prelude::*;

use crate::schema::*;
use crate::utils::establish_connection;

#[tauri::command]
pub async fn create_day(programid: i32, app: tauri::AppHandle) -> i32 {
    let conn = &mut establish_connection(&app);
    let day_id = diesel::insert_into(days::table)
        .values(days::dsl::program_id.eq(programid))
        .returning(days::dsl::id)
        .get_result::<i32>(conn)
        .expect("Error inserting new day");
    day_id
}

#[tauri::command]
pub async fn get_day_ids(programid: i32, app: tauri::AppHandle) -> Vec<i32> {
    let conn = &mut establish_connection(&app);
    let day_ids = days::table
        .filter(days::dsl::program_id.eq(programid))
        .select(days::dsl::id)
        .load::<i32>(conn)
        .expect("Error loading day ids");
    day_ids
}
