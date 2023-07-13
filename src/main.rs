mod command;
mod error;

use clap::{Arg, ArgMatches, Command};
use command::Commander;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    let cli = Command::new("Synthbrain")
        .version("0.1.0")
        .about("Command-line AI assistance")
        .author("Joshua Newell Diehl")
        .subcommand(image_command())
        .arg_required_else_help(true)
        .get_matches();

    process_input(cli).await?;

    Ok(())
}

fn image_command() -> Command {
    Command::new("image")
        .about("Request an image operation")
        .arg(
            Arg::new("operation")
                .help("The image-related operation to perform")
                .num_args(1)
                .required(true),
        )
        .arg(Arg::new("prompt").required(true))
}

async fn process_input(cli: ArgMatches) -> Result<(), anyhow::Error> {
    match cli.subcommand() {
        Some(("image", val)) => {
            let op = val
                .get_many::<String>("operation")
                .unwrap()
                .collect::<Vec<&String>>();
            let prompts = val
                .get_many::<String>("prompt")
                .unwrap()
                .collect::<Vec<&String>>();
            if op[0] == "generate" {
                Commander::generate_and_download(prompts[0]).await?;
            }
        }
        Some((&_, _)) => println!("Unknown subcommands"),
        None => eprintln!("Failed to match subcommand @ process_input"),
    };

    Ok(())
}
