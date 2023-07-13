use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Fatal error ::: command-line interface")]
    CliUsage,
    #[error("Fatal error ::: process curl command")]
    CurlRequest,
}
