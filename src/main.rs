use advent_of_code::request_input;
use advent_of_code::result::Result;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let inputs_dir = Path::new("inputs");
    if !inputs_dir.exists() {
        fs::create_dir(inputs_dir)?;
        fs::write("inputs/.gitignore", "*")?;
    }

    for year in 2015..=2024 {
        let year_dir = inputs_dir.join(year.to_string());
        if !year_dir.exists() {
            fs::create_dir(year_dir)?;
        }
        for day in 1..=25 {
            request_input(year, day).await?;
        }
    }
    Ok(())
}
