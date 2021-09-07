use passg::prelude::*;

#[rocket::get("/")]
fn generate_password() -> String {
    Generator::default().generate()
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::routes![generate_password])
}
