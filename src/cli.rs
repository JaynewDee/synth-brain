use crate::operation::Operation;
use clap::{Arg, ArgMatches, Command};
pub struct Cli;

impl Cli {
    pub fn matches() -> ArgMatches {
        Command::new("Synthbrain")
            .version("0.1.0")
            .about("Command-line AI assistance")
            .author("Joshua Newell Diehl")
            .subcommand(Self::image_command())
            .subcommand(Self::text_command())
            .subcommand(Self::speech_command())
            .arg_required_else_help(true)
            .get_matches()
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

    fn speech_command() -> Command {
        Command::new("speech")
            .about("Request speech-to-text transcription")
            .arg(
                Arg::new("filepath")
                    .help("The path to your audio file")
                    .num_args(1)
                    .required(true),
            )
    }
}

pub async fn process_input(cli: ArgMatches) -> Result<(), anyhow::Error> {
    let op_prompt = |val: &ArgMatches| {
        let op = val.get_one::<String>("operation").unwrap();
        let prompt = val.get_one::<String>("prompt").unwrap();
        (op.to_owned(), prompt.to_owned())
    };

    match cli.subcommand() {
        Some(("image", val)) => {
            let (op, prompt) = op_prompt(val);

            if op == "generate" {
                Operation::generate_and_download(&prompt).await?;
            }
        }

        Some(("text", val)) => {
            let (op, prompt) = op_prompt(val);

            if op == "complete" {
                Operation::complete_and_write(&prompt).await?;
            }
        }

        Some(("speech", val)) => {
            let filepath = val.get_one::<String>("filepath").unwrap();

            Operation::speech_to_text(&filepath)?;
        }
        Some((&_, _)) => println!("Unknown subcommands"),
        None => eprintln!("Failed to match subcommand @ process_input"),
    };

    Ok(())
}
