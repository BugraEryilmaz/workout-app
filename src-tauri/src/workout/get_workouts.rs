use chrono::NaiveDate;
use diesel::prelude::*;

use crate::models::*;
use crate::schema::*;
use crate::utils::establish_connection;

#[tauri::command]
pub async fn get_workouts_date(
    date: chrono::NaiveDate,
    app_handle: tauri::AppHandle,
) -> Vec<Workout> {
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
pub async fn get_workouts_day(
    daynumber: i32,
    programid: i32,
    app: tauri::AppHandle,
) -> Vec<Workout> {
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

#[tauri::command]
pub async fn last_workouts(app: tauri::AppHandle) -> Vec<String> {
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
