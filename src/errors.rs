#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("password: {0}")]
    Password(#[from] passg_lib::errors::Error),
}
