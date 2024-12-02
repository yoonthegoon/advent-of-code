use crate::error::Error;
use crate::result::Result;
use dotenvy::dotenv;
use std::env;
use std::fs;

pub mod error;
pub mod result;
mod year2024;

pub fn get_lines(day: usize) -> Result<Vec<String>> {
    let input = fs::read_to_string(format!("inputs/day{:02}.txt", day))?;
    let lines: Vec<_> = input.lines().map(|line| line.to_string()).collect();
    Ok(lines)
}

pub async fn request_input(day: usize) -> Result<()> {
    dotenv().ok();
    let cookie = env::var("COOKIE").unwrap();

    let client = reqwest::Client::new();
    let res = client
        .get(&format!("https://adventofcode.com/2024/day/{}/input", day))
        .header("Cookie", cookie)
        .send()
        .await?;

    let input = res.text().await?;
    if input.starts_with("Please don't repeatedly request this endpoint before it unlocks!") {
        return Err(Error::PageNotFound);
    }
    fs::write(format!("inputs/day{:02}.txt", day), input)?;
    Ok(())
}
