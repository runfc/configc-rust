use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {

    #[error("Configc: {0}")]
    Configc(String),

    #[error("Configc Sysctl: {0}")]
    Sysctl(String),

    #[error("Configc File {0}")]
    File(String),

    #[error("`{0}`")]
    Other(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
