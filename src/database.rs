//! Ici je mets le trucs relatifs à la creation de la base de données

use async_once::AsyncOnce;
use lazy_static::lazy_static;
use std::env;

use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, PgConnection, PgPool, Postgres, Row};

use crate::error::{self, Error};

static SCHEMA: &str = include_str!("schema.sql");

lazy_static! {
    static ref POOL: AsyncOnce<Result<PgPool, Error>> = AsyncOnce::new(async {
        let uri = env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new().connect(&uri).await?;
        Ok(pool)
    });
}

pub async fn db() -> Result<PoolConnection<Postgres>, error::Error> {
    let pool = POOL.get().await;
    match pool {
        Err(e) => Err(e.clone()),
        Ok(pool) => {
            let cnx = pool.acquire().await?;
            Ok(cnx)
        }
    }
}

pub async fn init_db(cnx: &mut PgConnection) -> Result<(), error::Error> {
    sqlx::query(SCHEMA).execute(cnx).await?;
    Ok(())
}

pub async fn add(cnx: &mut PgConnection, item: &str) -> Result<(), error::Error> {
    sqlx::query("INSERT INTO Task(descr) VALUES ($1)")
        .bind(item)
        .execute(cnx)
        .await?;
    Ok(())
}
pub async fn remove(cnx: &mut PgConnection, id: i32) -> Result<(), error::Error> {
    sqlx::query("DELETE FROM Task WHERE id = $1")
        .bind(id)
        .execute(cnx)
        .await?;
    Ok(())
}
pub async fn list(cnx: &mut PgConnection) -> Result<Vec<(i32, String)>, error::Error> {
    let rows = sqlx::query("SELECT id, descr FROM Task")
        .fetch_all(cnx)
        .await?;
    let mut data = vec![];
    for row in rows {
        data.push((row.get("id"), row.get("descr")));
    }
    Ok(data)
}
