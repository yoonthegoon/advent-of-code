use crate::error::Error;
use crate::result::Result;
use dotenvy::dotenv;
use std::env;
use std::fs;

pub mod error;
pub mod result;
mod year2024;

pub fn get_input(year: usize, day: usize) -> Result<String> {
    let input = fs::read_to_string(format!("inputs/{}/{:02}.txt", year, day))?;
    Ok(input)
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
    fs::write(format!("inputs/{}/{:02}.txt", year, day), input)?;
    Ok(())
}
