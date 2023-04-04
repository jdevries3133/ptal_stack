use actix_web::{error, web, HttpRequest, Result};
use sqlx::pool::PoolConnection;
use sqlx::postgres::Postgres;

use crate::session::deserialize_session;
use common::models::User;

pub struct RequestContext {
    pub dbc: PoolConnection<Postgres>,
    pub user: Option<User>,
}

pub struct AuthenticatedRequestContext {
    pub dbc: PoolConnection<Postgres>,
    pub user: User,
}

/// Useful for mapping errors in response handlers
pub fn err_and_log(e: anyhow::Error) -> error::Error {
    println!("{}", e);
    error::ErrorInternalServerError("Something went wrong")
}

pub async fn get_db(db: web::Data<sqlx::Pool<sqlx::Postgres>>) -> Result<PoolConnection<Postgres>> {
    db.acquire().await.map_err(|e| err_and_log(e.into()))
}

fn get_user(req: HttpRequest) -> Option<User> {
    let cookie = match req.headers().get("Authorization") {
        Some(v) => v
            .to_str()
            .unwrap_or("")
            .strip_prefix("Bearer ")
            .unwrap_or(""),
        None => return None,
    };

    match deserialize_session(cookie) {
        Ok(v) => Some(v.user),
        Err(e) => {
            eprint!("Could not deserialize session due to {}", e);
            None
        }
    }
}

/// For most request handlers, this utility can be used to acquire a database
/// connection and optionally determine the current user.
pub async fn get_ctx(
    req: HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> Result<RequestContext> {
    let dbc = get_db(db).await?;
    let user = get_user(req);

    Ok(RequestContext { dbc, user })
}

/// Like `unpack_request`, but this requires the user to be authenticated, and
/// will return a 400 error if they are not.
pub async fn get_authenticated_ctx(
    req: HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> Result<AuthenticatedRequestContext> {
    let context = get_ctx(req, db).await?;
    if let Some(user) = context.user {
        Ok(AuthenticatedRequestContext {
            user,
            dbc: context.dbc,
        })
    } else {
        Err(error::ErrorForbidden(
            "provide Authorization header with valid bearer token",
        ))
    }
}
