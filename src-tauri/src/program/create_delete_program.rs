use crate::models::Program;
use crate::schema::*;
use crate::utils::establish_connection;

use diesel::prelude::*;

#[tauri::command]
pub async fn create_program(title: String, app: tauri::AppHandle) -> Program {
    let conn = &mut establish_connection(&app);
    let new_program = diesel::insert_into(programs::table)
        .values(programs::dsl::title.eq(title))
        .returning(programs::all_columns)
        .get_result(conn)
        .expect("Error inserting new program");

    new_program
}

#[tauri::command]
pub async fn delete_program(programid: i32, app: tauri::AppHandle) {
    let conn = &mut establish_connection(&app);
    diesel::delete(programs::dsl::programs.filter(programs::dsl::id.eq(programid)))
        .execute(conn)
        .expect("Error deleting program");
}
