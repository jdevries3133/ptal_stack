use anyhow::Result;
use chrono::prelude::*;
use sqlx::pool::PoolConnection;
use sqlx::postgres::Postgres;
use sqlx::{query, query_as};

use common::models::User;

use super::dog::get_photo;
use super::pw::HashedPw;

/// `identifier` can be the username OR email
pub async fn get_user(dbc: &mut PoolConnection<Postgres>, identifier: &str) -> Result<User> {
    Ok(query_as!(
        User,
        "SELECT id, username, email FROM users WHERE username = $1 OR email = $1",
        identifier
    )
    .fetch_one(dbc.as_mut())
    .await?)
}

/// Create a user, linked to a row in the `password` table
pub async fn register_user(
    dbc: &mut PoolConnection<Postgres>,
    username: &str,
    email: &str,
    password: &HashedPw,
) -> Result<User> {
    let new_user = query_as!(
        User,
        "INSERT INTO users (username, email) VALUES ($1, $2)
        RETURNING id, username, email",
        username,
        email
    )
    .fetch_one(dbc.as_mut())
    .await?;

    query!(
        "INSERT INTO password (salt, digest, user_id) VALUES ($1, $2, $3)",
        password.salt,
        password.digest,
        new_user.id
    )
    .execute(dbc.as_mut())
    .await?;

    Ok(new_user)
}

pub async fn reset_password(
    dbc: &mut PoolConnection<Postgres>,
    user_id: i32,
    new_password: &HashedPw,
) -> Result<()> {
    query!(
        "UPDATE password SET salt = $1, digest = $2
        WHERE user_id = $3",
        new_password.salt,
        new_password.digest,
        user_id
    )
    .execute(dbc.as_mut())
    .await?;

    Ok(())
}

struct Dog {
    href: String,
}

pub async fn upsert_dog_of_the_day(dbc: &mut PoolConnection<Postgres>) -> Result<String> {
    let today = Utc::now().naive_utc().date();
    let dog = sqlx::query_as!(Dog, r#"SELECT href FROM dogs WHERE day = $1"#, today)
        .fetch_optional(dbc.as_mut())
        .await?;

    if let Some(dog) = dog {
        Ok(dog.href)
    } else {
        let new_dog = get_photo().await?;
        sqlx::query!("INSERT INTO dogs (href) VALUES ($1)", new_dog)
            .execute(dbc.as_mut())
            .await?;

        Ok(new_dog)
    }
}
