use crate::database::{db, init_db};

mod database;
mod error;
mod routes;

#[rocket::main]
async fn main() {
    match init_db(&db().await?).await {
        Ok(()) => println!("db initialized"),
        Err(e) => println!("{}", e),
    }

    let rkt = rocket::build()
        .mount("/pass", rocket::routes![routes::generate_password])
        .mount(
            "/todo",
            rocket::routes![routes::task_add, routes::task_remove, routes::task_list],
        )
        .launch()
        .await
        .map_err(Box::new);

    match rkt {
        Ok(()) => println!("rocket launched"),
        Err(e) => println!("launch {}", e),
    }
}
