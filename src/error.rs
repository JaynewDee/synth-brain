use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Fatal error ::: command-line interface")]
    CliUsage(String),
}

impl std::convert::From<clap::error::Error> for Error {
    fn from(value: clap::error::Error) -> Self {
        Error::CliUsage(value.to_string())
    }
}
