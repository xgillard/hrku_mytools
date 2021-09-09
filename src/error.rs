use std::sync::Arc;

use rocket::response::Responder;

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("password: {0}")]
    Password(#[from] Arc<passg_lib::errors::Error>),
    #[error("database: {0}")]
    Database(#[from] Arc<sqlx::error::Error>),
    #[error("rocket: {0}")]
    Rocket(#[from] Arc<rocket::error::Error>),
    #[error("environment: {0}")]
    Environment(#[from] Arc<std::env::VarError>),
}

impl<'r, 'o: 'r> Responder<'o, 'static> for Error {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        self.to_string().respond_to(request)
    }
}
impl From<passg_lib::errors::Error> for Error {
    fn from(e: passg_lib::errors::Error) -> Self {
        Self::Password(Arc::new(e))
    }
}
impl From<sqlx::error::Error> for Error {
    fn from(e: sqlx::error::Error) -> Self {
        Self::Database(Arc::new(e))
    }
}
impl From<rocket::error::Error> for Error {
    fn from(e: rocket::error::Error) -> Self {
        Self::Rocket(Arc::new(e))
    }
}
impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        Self::Environment(Arc::new(e))
    }
}
