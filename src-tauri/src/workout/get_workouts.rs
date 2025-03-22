use chrono::NaiveDate;
use diesel::prelude::*;

use crate::models::*;
use crate::schema::*;
use crate::utils::establish_connection;

#[tauri::command]
pub async fn get_workouts_date(
    date: chrono::NaiveDate,
    app_handle: tauri::AppHandle,
) -> (Vec<Workout>, Option<Day>) {
    let conn = &mut establish_connection(&app_handle);
    let today = chrono::Local::now().date_naive();

    // Check if there is a day finished at date for the active programs
    let workouts: Vec<(Workout, Day)> = programs::table
        .inner_join(days::table.inner_join(workouts::table))
        .filter(
            programs::dsl::active
                .eq(true)
                .and(days::dsl::complete_date.eq(Some(date.to_string())))
                .and(workouts::dsl::done_date.eq(Some(date.to_string()))),
        )
        .select((workouts::all_columns, days::all_columns))
        .load::<(Workout, Day)>(conn)
        .expect("Error loading workouts");
    if workouts.len() != 0 {
        let day = workouts.first().unwrap().1.clone();
        let workouts = workouts.into_iter().map(|(w, _)| w).collect();
        return (workouts, Some(day));
    }

    // If the date is in the past
    //    there is a workout done at the date, return the workouts
    //    there is no workout done at the date, return empty
    if date < today {
        let workouts: Vec<(Workout, Day)> = programs::table
            .inner_join(days::table.inner_join(workouts::table))
            .filter(
                programs::dsl::active
                    .eq(true)
                    .and(workouts::dsl::done_date.eq(Some(date.to_string()))),
            )
            .select((workouts::all_columns, days::all_columns))
            .load::<(Workout, Day)>(conn)
            .expect("Error loading workouts");
        if workouts.len() != 0 {
            let day = workouts.first().unwrap().1.clone();
            let workouts = workouts.into_iter().map(|(w, _)| w).collect();
            return (workouts, Some(day));
        } else {
            return (vec![], None);
        }
    }

    // If the date is not in the past, adjust the schedule properly
    //   1. Get the last done day of the active programs
    //   2. Calculate the day_number of the date
    //   3. Get the workouts of the day
    let diff = date.signed_duration_since(today).num_days();
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
            return (vec![], None);
        }
    }
    let last_done_date = last_done_date.unwrap().parse::<NaiveDate>().unwrap();
    // day_number = last_done_day_number + (date-today) + (min(today, last_done_date + 1)-last_done_date)
    // To adjust for skipped days, we have the min part
    let day_number =
        last_done_day_number.unwrap() + (diff + (today - last_done_date).num_days().min(1)) as i32;

    let workouts = workouts::table
        .inner_join(days::table)
        .filter(
            days::dsl::program_id
                .eq(program_id)
                .and(days::dsl::day_number.eq(day_number))
                .and(
                    workouts::dsl::done
                        .eq(false)
                        .or(workouts::dsl::done_date.eq(Some(today.to_string()))),
                ),
        )
        .select((workouts::all_columns, days::all_columns))
        .load::<(Workout, Day)>(conn)
        .expect("Error loading workouts");
    if workouts.len() != 0 {
        let day = workouts.first().unwrap().1.clone();
        let workouts = workouts.into_iter().map(|(w, _)| w).collect();
        return (workouts, Some(day));
    }

    // If there is no workout for the day, return empty
    return (vec![], None);
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
pub async fn last_workouts(app: tauri::AppHandle) -> Vec<(String, i32, String)> {
    let conn = &mut establish_connection(&app);
    let workouts = workouts::table
        .order(workouts::dsl::id.desc())
        .limit(4)
        .distinct()
        .select((
            workouts::dsl::link,
            workouts::dsl::duration,
            workouts::dsl::title,
        ))
        .load(conn)
        .expect("Error loading workouts");
    workouts
}
