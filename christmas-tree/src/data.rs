use super::YEAR;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DayData {
    pub input: String,
}

pub fn get_data(day: u32) -> DayData {
    cache::get_data(day).unwrap_or_else(|| {
        println!("Fetching data for day {}", day);
        fetch_and_cache_data(day, &get_session_token_from_env())
    })
}

pub mod cache {
    use std::{
        fs::{create_dir_all, File},
        path::PathBuf,
    };

    use super::DayData;

    const CACHE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/.cache");

    pub fn file_path(day: u32) -> PathBuf {
        let dir = PathBuf::from(CACHE_DIR);
        dir.join(format!("day{}.ron", day))
    }

    pub fn get_data(day: u32) -> Option<DayData> {
        let file = File::open(file_path(day)).ok()?;

        Some(ron::de::from_reader(file).expect("Files should only be generated from this program"))
    }

    pub fn set_data(day: u32, data: &DayData) -> Result<(), std::io::Error> {
        create_dir_all(CACHE_DIR)?;
        let file = File::create(file_path(day))?;

        ron::ser::to_writer(file, data)
            .expect("There shouldn't be a problem with writing regular data");

        Ok(())
    }
}

pub fn fetch_and_cache_data(day: u32, session_token: &str) -> DayData {
    let data = fetch_data(day, session_token).unwrap();
    cache::set_data(day, &data).unwrap();

    data
}

fn fetch_data(day: u32, session_token: &str) -> reqwest::Result<DayData> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            YEAR, day
        ))
        .header(reqwest::header::COOKIE, format!("session={session_token}"))
        .send()?;

    let input = response.text()?;

    Ok(DayData { input })
}

fn get_session_token_from_env() -> String {
    dotenv::dotenv().unwrap();
    std::env::var("AOC_SESSION_TOKEN").unwrap()
}
