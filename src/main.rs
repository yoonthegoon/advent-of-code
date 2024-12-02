use advent_of_code::request_input;
use advent_of_code::result::Result;

#[tokio::main]
async fn main() -> Result<()> {
    for day in 1..=25 {
        request_input(day).await?;
    }
    Ok(())
}
