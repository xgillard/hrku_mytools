//! Ici je mets le trucs relatifs à la creation de la base de données

use std::env;

use sqlx::{postgres::PgPoolOptions, PgPool, Row};

use crate::error;

static SCHEMA: &str = include_str!("schema.sql");

pub async fn db() -> Result<PgPool, error::Error> {
    let uri = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(&uri).await?;
    Ok(pool)
}
pub async fn init_db(pool: &PgPool) -> Result<(), error::Error> {
    sqlx::query(SCHEMA).execute(pool).await?;
    Ok(())
}

pub async fn add(pool: &PgPool, item: &str) -> Result<(), error::Error> {
    sqlx::query("INSERT INTO Task(descr) VALUES (?)")
        .bind(item)
        .execute(pool)
        .await?;
    Ok(())
}
pub async fn remove(pool: &PgPool, id: i64) -> Result<(), error::Error> {
    sqlx::query("DELETE FROM Task WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
pub async fn list(pool: &PgPool) -> Result<Vec<(i64, String)>, error::Error> {
    let rows = sqlx::query("SELECT (id, descr) FROM Task")
        .fetch_all(pool)
        .await?;
    let mut data = vec![];
    for row in rows {
        data.push((row.get(1), row.get(2)));
    }
    Ok(data)
}
