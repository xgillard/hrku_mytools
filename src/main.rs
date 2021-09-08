use crate::database::{db, init_db};

mod database;
mod error;
mod routes;

#[rocket::main]
async fn main() {
    {
        let pool = db().await;
        match pool {
            Ok(pool) => match init_db(&pool).await {
                Ok(()) => println!("db initialized"),
                Err(e) => println!("{}", e),
            },
            Err(e) => println!("could not fetch the pool {}", e),
        }
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
