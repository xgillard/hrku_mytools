use rocket::response::Responder;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("password: {0}")]
    Password(#[from] passg_lib::errors::Error),
    #[error("database: {0}")]
    Database(#[from] sqlx::error::Error),
    #[error("rocket: {0}")]
    Rocket(#[from] Box<rocket::error::Error>),
    #[error("environment: {0}")]
    Environment(#[from] std::env::VarError),
}

impl<'r, 'o: 'r> Responder<'o, 'static> for Error {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        self.to_string().respond_to(request)
    }
}
