mod application;
mod domain;
mod infrastructure;
mod presentation;

use application::manage::run;
use clap::Parser;
use std::{error::Error, time::Duration};
use tokio;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Show splash screen or not
    #[arg(short, long, default_value_t = false)]
    disable_splash_screen: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Read arguments from cli by clap
    let args = CliArgs::parse();

    run(!args.disable_splash_screen).await?;
    Ok(())
}
