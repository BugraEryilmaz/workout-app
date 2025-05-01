use diesel::prelude::*;
use futures_util::StreamExt;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder,
};
use semver::Version;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri_plugin_dialog::FilePath;
use tauri_plugin_updater::{RemoteRelease, UpdaterExt};

use crate::{models::{AddAutoPrograms, Program}, schema::{add_auto_programs, programs}, utils::establish_connection};

use super::restore_program_from_path;

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoAddPrograms {
    pub programs: Vec<AutoAddProgram>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoAddProgram {
    pub program_name: String,
    pub url: String,
}

pub fn get_programs_from_latest(raw_json: &serde_json::Value) -> AutoAddPrograms {
    AutoAddPrograms::deserialize(raw_json).unwrap_or_else(|_| {
        println!("Failed to parse auto add programs from JSON");
        AutoAddPrograms { programs: vec![] }
    })
}

// Test
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_programs_from_latest() {
        let json_data = json!({
          "version": "0.3.0",
          "notes": "See the assets to download this version and install.",
          "pub_date": "2025-04-08T21:26:53.726Z",
          "platforms": {
            "darwin-aarch64": {
              "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVSMklnL3p4QjhqWkJGa1FUNk93RmFReHJXY1BIcFMyWDNxZmxGdlYwOWVlT3VWaFQ2djVlQlNuOGRmMEdqckYrd2krelUwb3Z1SVJqeTlSTWtFQjRIMHlHY0x6VVJ6bndRPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzQ0MTQ3NTg2CWZpbGU6d29ya291dC5hcHAudGFyLmd6CnBnUUNyUjE2Wlg4VEEvTmhxcWt3ZGtJcEIwbjdnMDhINWs0dEdrR2xGaGxTOU5MeXk5SEl1bCtXQXAvYnFwY0grbnpZR3ZMajN0bTYwN1RVa2UwVkRBPT0K",
              "url": "https://github.com/BugraEryilmaz/workout-app/releases/download/app-v0.2.4/workout_aarch64.app.tar.gz"
            },
            "windows-x86_64": {
              "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVSMklnL3p4QjhqWkVEM1FKOGJ6Rmk5eGhPS2d4STBsdXVJUnJHWHd3OVlTdDJZTTBrc0Rud1hxdkRlaVh4WDBKcmhtQTF3ZnM4cFRMVEtSSFVoUnZNeXM2OWVQYy9jTkFBPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzQ0MTQ3NjA5CWZpbGU6d29ya291dF8wLjIuNF94NjRfZW4tVVMubXNpCmxiMjZYeUdRS2piVUhqRXp2cHBLRFZnWVowZzBVWEdsQisrQ3B4RzhxdEZzdENLaXlzbzdPdE44aGozeU5YS3JHYUljS1dQTndndXp2Wmg3WGx1WEJnPT0K",
              "url": "https://github.com/BugraEryilmaz/workout-app/releases/download/app-v0.2.4/workout_0.2.4_x64_en-US.msi"
            }
          },
          "programs": [
            {
              "program_name": "Program 1",
              "url": "http://example.com/program1"
            },
            {
              "program_name": "Program 2",
              "url": "http://example.com/program2"
            }
          ]
        });

        let programs = get_programs_from_latest(&json_data);
        let programs = programs.programs;
        assert_eq!(programs.len(), 2);
        assert_eq!(programs[0].program_name, "Program 1");
        assert_eq!(programs[0].url, "http://example.com/program1");
    }
}

pub async fn add_programs_to_db_for_later(app: &tauri::AppHandle, programs: AutoAddPrograms) {
    let mut conn = establish_connection(app);
    for program in programs.programs {
        let program_name = program.program_name;
        let url = program.url;
        // add to the database
        diesel::insert_into(add_auto_programs::table)
            .values((
                add_auto_programs::program_name.eq(program_name.clone()),
                add_auto_programs::url.eq(url.clone()),
                add_auto_programs::done.eq(false),
            ))
            .execute(&mut conn)
            .expect("Error inserting new program");
    }
}

fn always_true(_: Version, _: RemoteRelease) -> bool {
    true
}

async fn get_latest_info(
    app: &tauri::AppHandle,
) -> Result<serde_json::Value, tauri_plugin_updater::Error> {
    // We create a new updater instance that always thinks there is an update available.
    // This is to reuse the functionality of the updater to get the latest version info.
    let updater = app
        .updater_builder()
        .version_comparator(always_true)
        .build()?;
    let latest = updater.check().await?.unwrap();
    Ok(latest.raw_json)
}

pub async fn check_first_time(app: &tauri::AppHandle) {
    let mut conn = establish_connection(app);
    let scheduled_programs = add_auto_programs::table
        .first::<AddAutoPrograms>(&mut conn)
        .optional()
        .expect("Error loading scheduled programs");
    if scheduled_programs.is_none() {
        println!("Scheduled programs is empty, checking for updates...");
        // get the programs from raw json
        let latest_info = get_latest_info(app).await.unwrap();
        let programs = get_programs_from_latest(&latest_info);
        println!("Programs: {:?}", programs);
        add_programs_to_db_for_later(app, programs).await;
        println!("Added programs to db for later");
    }
}

pub async fn download_and_add_programs(app: &tauri::AppHandle) {
    let mut conn = establish_connection(app);
    let scheduled_programs = add_auto_programs::table
        .filter(add_auto_programs::done.eq(false))
        .load::<AddAutoPrograms>(&mut conn)
        .expect("Error loading scheduled programs");

    println!("Scheduled programs: {:?}", scheduled_programs);
    for program in scheduled_programs {
        let program_name = program.program_name;
        let url = program.url;
        // Check if the program already exists in the database
        let existing_program = programs::table
            .filter(programs::title.eq(program_name.clone()))
            .first::<Program>(&mut conn)
            .optional().expect("Error checking existing program");
        if existing_program.is_some() {
            // Mark the program as done in the database
            diesel::update(add_auto_programs::table.filter(
                add_auto_programs::program_name.eq(program_name.clone()),
            ))
            .set(add_auto_programs::done.eq(true))
            .execute(&mut conn)
            .expect("Error updating program status");
            println!("Program already exists, skipping download: {}", program_name);
            continue;
        }
        // download the file in the url
        let downloaded_path = app
            .path()
            .data_dir()
            .unwrap()
            .join("workout-app/")
            .join(program_name.clone());
        if !downloaded_path.exists() {
            std::fs::create_dir_all(downloaded_path.parent().unwrap()).unwrap();
            download_file(&url, downloaded_path.to_str().unwrap())
                .await
                .unwrap();
            println!(
                "Downloaded program: {} from URL: {}",
                program_name, url
            );
            // Insert the new program into the database
            restore_program_from_path(FilePath::Path(downloaded_path.clone()), app.clone())
                .await
                .expect("Error restoring program from path");
            println!(
                "Restored program from path: {}",
                downloaded_path.to_str().unwrap()
            );
            // Mark the program as done in the database
            diesel::update(add_auto_programs::table.filter(
                add_auto_programs::program_name.eq(program_name.clone()),
            ))
            .set(add_auto_programs::done.eq(true))
            .execute(&mut conn)
            .expect("Error updating program status");
            println!("Marked program as done: {}", program_name);
        }
    }
}

async fn download_file(url: &str, downloaded_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // set our headers
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        "Accept",
        HeaderValue::from_str("application/octet-stream").unwrap(),
    );

    let mut request = ClientBuilder::new().user_agent("WorkoutApp");
    request = request.timeout(std::time::Duration::from_secs(60));
    let response = request.build()?.get(url).headers(headers).send().await?;

    if !response.status().is_success() {
        return Err("Download request failed with status: {}".into());
    }

    let mut buffer = Vec::new();

    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        buffer.extend(chunk);
    }
    // Save the file to the specified path
    let mut file = std::fs::File::create(downloaded_path)?;
    std::io::copy(&mut buffer.as_slice(), &mut file)?;
    Ok(())
}
