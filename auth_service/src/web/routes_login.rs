#![allow(unused)]

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::core::services::user_service::UserService;
use crate::web::token_manager::{Claims, TokenManager};
use crate::{Error, Result};
use axum::extract::State;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;
use axum::Router;
use axum::routing::post;
use axum::Json;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

use crate::web;

pub fn routes(db_connection: DatabaseConnection) -> Router{
    Router::new().route("/api/login", post(api_login)).with_state(db_connection)
}

async fn api_login(State(db): State<DatabaseConnection>, cookies:Cookies, payload: Json<LoginRequest>) -> Result<Json<Value>>{
    println!("--> {:12} - api_login", "Handler");

    //validate password
    let user = UserService::validate_user(&db, &payload.username, &payload.password).await;
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() + Duration::from_secs(28800).as_secs();
    //generate token
    let claim = Claims{
        exp: expiration,
        first_name: user.first_name,
        id:user.id,
        last_name : user.last_name,
        user_name: user.username
    };

    let token = TokenManager::generate_token(&claim).expect("An error occurred generating token");
    let body = Json(json!({
"result": {
    "token": &token,
    "exp":&expiration
}
    }));

    return Ok(body);
}

#[derive(Debug, Deserialize)]
struct LoginRequest{
    username: String,
    password: String
}
