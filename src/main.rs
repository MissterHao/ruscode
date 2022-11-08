mod application;
mod domain;
mod infrastructure;
mod presentation;
use application::manage::run;
use std::{error::Error, time::Duration};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run(Duration::from_millis(250u64), true).await?;
    Ok(())
}
