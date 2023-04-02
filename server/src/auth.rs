use anyhow::{bail, Result};
use sqlx::pool::PoolConnection;
use sqlx::postgres::Postgres;
use sqlx::query_as;

use super::db_ops::get_user;
use super::pw::{check, HashedPw};
use super::session::Session;

/// `identifier` can be a users username _or_ email
pub async fn authenticate(
    dbc: &mut PoolConnection<Postgres>,
    identifier: &str,
    password: &str,
) -> Result<Session> {
    let user = get_user(dbc, identifier).await?;
    let truth = query_as!(
        HashedPw,
        "SELECT salt, digest FROM password WHERE user_id = $1",
        user.id
    )
    .fetch_one(dbc.as_mut())
    .await?;

    if check(password, &truth).is_ok() {
        Ok(Session { user })
    } else {
        bail!("wrong password")
    }
}
