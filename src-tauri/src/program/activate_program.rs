use crate::models::Program;
use crate::schema::*;
use crate::utils::establish_connection;

use diesel::prelude::*;

#[tauri::command]
pub async fn activate_program(program: Program, app: tauri::AppHandle) {
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
pub async fn deactivate_program(program: Program, app: tauri::AppHandle) {
    let conn = &mut establish_connection(&app);
    diesel::update(programs::dsl::programs)
        .filter(programs::dsl::id.eq(program.id))
        .set(programs::dsl::active.eq(false))
        .execute(conn)
        .expect("Error updating programs");
}

#[tauri::command]
pub async fn clear_progress(program: Program, app: tauri::AppHandle) {
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
