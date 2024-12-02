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

    for day in 1..=25 {
        request_input(day).await?;
    }
    Ok(())
}
