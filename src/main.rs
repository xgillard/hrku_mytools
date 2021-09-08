use crate::database::{db, init_db};

mod database;
mod error;
mod routes;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    init_db(&db().await?).await?;

    rocket::build()
        .mount("/pass", rocket::routes![routes::generate_password])
        .mount(
            "/todo",
            rocket::routes![routes::task_add, routes::task_remove, routes::task_list],
        )
        .launch()
        .await
        .map_err(Box::new)?;
    Ok(())
}
