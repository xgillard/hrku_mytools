#[macro_use]
extern crate rocket;
extern crate passg;

use passg::prelude::*;

#[get("/")]
fn generate_password() -> String {
    Generator::default().generate()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![generate_password])
}
