extern crate dotenv;

use dotenv::dotenv;
use sqlx::{mysql::MySqlPool, MySql, Pool};
use std::env;

pub async fn get_pool() -> Result<Pool<MySql>, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();
    MySqlPool::connect(&database_url).await
}

pub async fn get_hi_msg() -> Result<String, sqlx::Error> {
    let pool = get_pool().await?;
    let msgs = sqlx::query!(r#"SELECT new_user FROM msgs"#).fetch_all(&pool).await?;

    Ok(msgs[0].new_user.clone().unwrap())
}

pub async fn set_hi_msg(text: String) -> Result<(), sqlx::Error> {
    let pool = get_pool().await?;
    let _ = sqlx::query!(r#"UPDATE msgs SET new_user = ?"#, &text)
        .execute(&pool)
        .await?;

    Ok(())
}

pub async fn get_help_msg() -> Result<String, sqlx::Error> {
    let pool = get_pool().await?;
    let msgs = sqlx::query!(r#"SELECT help FROM msgs"#).fetch_all(&pool).await?;

    Ok(msgs[0].help.clone().unwrap())
}

pub async fn set_help_msg(text: String) -> Result<(), sqlx::Error> {
    let pool = get_pool().await?;
    let _ = sqlx::query!(r#"UPDATE msgs SET help = ? "#, &text)
        .execute(&pool)
        .await?;

    Ok(())
}
