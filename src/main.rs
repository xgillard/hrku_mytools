mod errors;

use std::str::FromStr;

use passg_lib::prelude::*;

#[rocket::get("/?<length>&<alpha>&<digits>&<specials>")]
fn generate_password(
    length: Option<usize>,
    alpha: Option<String>,
    digits: Option<String>,
    specials: Option<String>,
) -> String {
    let gen = generator(length, alpha, digits, specials);
    match gen {
        Ok(gen) => gen.generate(),
        Err(e) => format!("{:?}", e),
    }
}

fn generator(
    length: Option<usize>,
    alpha: Option<String>,
    digit: Option<String>,
    special: Option<String>,
) -> Result<Generator, Error> {
    let mut builder = GeneratorBuilder::default();
    if let Some(lenght) = length {
        builder.length(lenght);
    }
    if let Some(alpha) = alpha {
        builder.alpha(Alpha::from_str(&alpha)?);
    }
    if let Some(digit) = digit {
        builder.digit(Digit::from_str(&digit)?);
    }
    if let Some(special) = special {
        builder.special(Special::from_str(&special)?);
    }
    Ok(builder.build().unwrap())
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::routes![generate_password])
}
