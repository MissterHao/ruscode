mod application;
mod domain;
mod infrastructure;
mod presentation;

use application::manage::run;
use std::{error::Error, time::Duration};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run(true).await?;
    Ok(())
}
