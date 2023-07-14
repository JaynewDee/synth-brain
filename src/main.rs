mod command;
mod consts;
mod error;
mod models;

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
        .subcommand(text_command())
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

fn text_command() -> Command {
    Command::new("text")
        .about("Request a text operation")
        .arg(
            Arg::new("operation")
                .help("The image-related operation to perform")
                .num_args(1)
                .required(true),
        )
        .arg(Arg::new("prompt").required(true))
}

async fn process_input(cli: ArgMatches) -> Result<(), anyhow::Error> {
    let op_prompt = |val: &ArgMatches| {
        let op = val.get_one::<String>("operation").unwrap();
        let prompt = val.get_one::<String>("prompt").unwrap();
        (op.to_owned(), prompt.to_owned())
    };

    match cli.subcommand() {
        Some(("image", val)) => {
            let (op, prompt) = op_prompt(val);

            if op == "generate" {
                Commander::generate_and_download(&prompt).await?;
            }
        }
        Some(("text", val)) => {
            let (op, prompt) = op_prompt(val);

            if op == "complete" {
                Commander::complete_and_write(&prompt).await?;
            }
        }
        Some((&_, _)) => println!("Unknown subcommands"),
        None => eprintln!("Failed to match subcommand @ process_input"),
    };

    Ok(())
}
