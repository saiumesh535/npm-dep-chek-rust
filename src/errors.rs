use snafu::Snafu;
use std::env::VarError;
use std::io::Error as IOError;
use std::path::PathBuf;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("{}", message))]
    CustomErrorMessage { message: String },
    #[snafu(display("{}", message))]
    EnvErrorConfig { source: VarError, message: String },
    #[snafu(display("Unable to read configuration from {}: {}", path.display(), source))]
    ReadConfiguration { source: IOError, path: PathBuf },
    #[snafu(display("failed to deserialize: {}", source))]
    Deserialization { source: serde_json::Error },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
