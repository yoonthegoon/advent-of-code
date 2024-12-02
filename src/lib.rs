use crate::error::Error;
use crate::result::Result;
use dotenvy::dotenv;
use std::env;
use std::fs;

pub mod error;
pub mod result;
mod year2024;

pub fn get_lines(year: usize, day: usize) -> Result<Vec<String>> {
    let input = fs::read_to_string(format!("inputs/{}/day{:02}.txt", year, day))?;
    let lines: Vec<_> = input.lines().map(|line| line.to_string()).collect();
    Ok(lines)
}

pub async fn request_input(year: usize, day: usize) -> Result<()> {
    dotenv().ok();
    let cookie = env::var("COOKIE").unwrap();

    let client = reqwest::Client::new();
    let res = client
        .get(&format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .header("Cookie", cookie)
        .send()
        .await?;

    if res.status().is_client_error() {
        return Err(Error::PageNotFound);
    }

    let input = res.text().await?;
    fs::write(format!("inputs/{}/day{:02}.txt", year, day), input)?;
    Ok(())
}
