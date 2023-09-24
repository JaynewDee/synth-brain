mod cli;
mod consts;
mod error;
mod models;
mod operation;

use cli::{process_input, Cli};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    let cli = Cli::main();

    process_input(cli).await?;

    Ok(())
}
