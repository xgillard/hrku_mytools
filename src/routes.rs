//! Dans ce module, j'ai défini les routes que je voulais implémenter dans mon
//! application.
use std::str::FromStr;

use passg_lib::prelude::*;

use crate::{
    database::{add, db, list, remove},
    error::Error,
};

#[rocket::get("/add/<text>")]
pub async fn task_add(text: String) -> String {
    let res = _task_add(text).await;
    match res {
        Ok(r) => r,
        Err(e) => format!("{}", e),
    }
}

#[rocket::get("/remove/<id>")]
pub async fn task_remove(id: i32) -> String {
    let res = _task_remove(id).await;
    match res {
        Ok(r) => r,
        Err(e) => format!("{}", e),
    }
}
#[rocket::get("/list")]
pub async fn task_list() -> String {
    let res = _task_list().await;
    match res {
        Ok(r) => r,
        Err(e) => format!("{}", e),
    }
}
async fn _task_add(text: String) -> Result<String, Error> {
    let pool = db().await?;
    add(&pool, &text).await?;
    //
    let data = list(&pool).await?;
    Ok(lst2str(data))
}
async fn _task_remove(id: i32) -> Result<String, Error> {
    let pool = db().await?;
    remove(&pool, id).await?;
    //
    let data = list(&pool).await?;
    Ok(lst2str(data))
}
async fn _task_list() -> Result<String, Error> {
    let pool = db().await?;
    let data = list(&pool).await?;
    Ok(lst2str(data))
}
fn lst2str(data: Vec<(i32, String)>) -> String {
    let mut out = String::new();
    for d in data {
        out.push_str(&format!("* ({:>10}) {}\n", d.0, d.1));
    }

    out
}

#[rocket::get("/?<length>&<alpha>&<digits>&<specials>")]
pub fn generate_password(
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

pub fn generator(
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
