use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::models::Workout;
use crate::schema::*;

pub fn complete_day(day_id: i32, finish_date: NaiveDate, conn: &mut SqliteConnection) {
    diesel::update(days::dsl::days)
        .filter(days::dsl::id.eq(day_id))
        .set((
            days::dsl::done.eq(true),
            days::dsl::complete_date.eq(Some(finish_date.to_string())),
        ))
        .execute(conn)
        .expect("Error updating day");
    // Check next day if it is rest day
    // Get current day's program_id and day_number
    let (current_day_program_id, current_day_number) = days::table
        .filter(days::dsl::id.eq(day_id))
        .select((days::dsl::program_id, days::dsl::day_number.nullable()))
        .first::<(i32, Option<i32>)>(conn)
        .expect("Error getting current day");
    // Get next day
    let next_day_number = current_day_number.unwrap() + 1;
    let next_day_id = days::table
        .filter(
            days::dsl::program_id
                .eq(current_day_program_id)
                .and(days::dsl::day_number.eq(next_day_number)),
        )
        .select(days::dsl::id)
        .first::<i32>(conn)
        .optional()
        .expect("Error getting next day");
    // If there is no next day, create and achievement
    if next_day_id.is_none() {
        // Create achievement
        let _achievement = diesel::insert_into(achievements::table)
            .values((
                achievements::dsl::program_id.eq(current_day_program_id),
                achievements::dsl::date.eq(finish_date.to_string()),
            ))
            .returning(achievements::all_columns)
            .get_result::<crate::models::Achievement>(conn)
            .expect("Error creating achievement");
        return;
    }
    let next_day_id = next_day_id.unwrap();
    // If next day is rest day, complete it
    let workouts_of_next_day = workouts::table
        .filter(workouts::dsl::day_id.eq(next_day_id))
        .load::<Workout>(conn)
        .expect("Error loading workouts of next day");
    if workouts_of_next_day.len() == 0 {
        complete_day(next_day_id, finish_date + chrono::Duration::days(1), conn);
    }
}
