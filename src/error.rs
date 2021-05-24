use thiserror::Error;

#[derive(Error, Debug)]
pub enum WorkonError {
    #[error("Config is invalid, reason: {0}")]
    InvalidConfig(&'static str),
    #[error("Specified project name is not present in the config")]
    InvalidProjectNameError,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ConfigSerializeError(#[from] serde_yaml::Error)
}

pub type WorkonResult<T> = Result<T, WorkonError>;
