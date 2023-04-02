use actix_web::{error, get, post, web, HttpRequest, HttpResponse, Result};

use common::models::{LoginPayload, LoginResponse, Profile, RegisterPayload};

use super::auth::authenticate;
use super::db_ops::{register_user, upsert_dog_of_the_day};
use super::pw::hash_new;
use super::session::{serialize_session, Session};
use super::util::{err_and_log, get_authenticated_ctx, get_ctx, get_db};

struct RequestCountDemo {
    request_count: i32,
}
#[get("/")]
/// An endpoint that optionally identifies the user, talks to the database,
/// and sends a dynamic response.
///
/// Doing the `map_err` inside route handlers is quite cumbersome, so you'll
/// see other handlers dispatch to db_ops, where we'll be returning
/// anyhow::Result, leading to better ergonomics, but this route makes the
/// nuts and bolts of writing a request handler as clear as possible.
pub async fn index(
    req: HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> Result<HttpResponse> {
    let mut ctx = get_ctx(req, db).await?;

    if let Some(user) = ctx.user {
        // Notice that the user profile is stored in the session as a secure
        // session token, we don't need to hit the database for authentication
        // or to know basic details about the user.
        println!("Recieved request from {}", user.username);
        Ok(HttpResponse::Ok().body(format!("Hello {}", user.username)))
    } else {
        println!("Recieved request from anonymous user");
        sqlx::query!("INSERT INTO anonymous_req_count values (default)")
            .execute(ctx.dbc.as_mut())
            .await
            .map_err(|e| {
                println!("Err: {}", e);
                error::ErrorInternalServerError("")
            })?;
        let count = sqlx::query_as!(
            RequestCountDemo,
            "SELECT id AS request_count FROM anonymous_req_count ORDER BY id DESC LIMIT 1"
        )
        .fetch_one(ctx.dbc.as_mut())
        .await
        .map_err(|e| {
            println!("Err: {}", e);
            error::ErrorInternalServerError("")
        })?;
        let content = format!("This was anonymous request number {}", count.request_count);

        Ok(HttpResponse::Ok().body(content))
    }
}

#[get("/profile")]
pub async fn profile(
    req: HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> Result<HttpResponse> {
    let mut ctx = get_authenticated_ctx(req, db).await?;

    let dog = upsert_dog_of_the_day(&mut ctx.dbc)
        .await
        .map_err(|e| err_and_log(e))?;

    Ok(HttpResponse::Ok().json(Profile {
        user: ctx.user,
        dog_photo_of_the_day_href: dog,
    }))
}

#[post("/auth/login")]
pub async fn login(
    payload: web::Json<LoginPayload>,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> Result<HttpResponse> {
    let mut dbc = get_db(db).await?;
    let auth_result = authenticate(&mut dbc, &payload.identifier, &payload.password).await;

    if let Ok(session) = auth_result {
        let response_data = serde_json::to_string(&LoginResponse {
            session_token: serialize_session(&session)?,
        })?;

        Ok(HttpResponse::Ok().body(response_data))
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

#[post("/auth/register")]
pub async fn register(
    payload: web::Json<RegisterPayload>,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> Result<HttpResponse> {
    println!("{:?}", payload);
    let mut dbc = get_db(db).await?;
    let hashed_pw = hash_new(&payload.password);
    let user = register_user(&mut dbc, &payload.username, &payload.email, &hashed_pw)
        .await
        .map_err(|e| err_and_log(e))?;

    println!("{} has registered", payload.username);

    let session = Session { user };
    let response_body = serde_json::to_string(&LoginResponse {
        session_token: serialize_session(&session)?,
    })?;

    Ok(HttpResponse::Ok().body(response_body))
}
