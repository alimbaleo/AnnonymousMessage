#![allow(unused)]

use crate::{Error, Result};
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;
use axum::Router;
use axum::routing::post;
use axum::Json;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

use crate::web;

pub fn routes() -> Router{
    Router::new().route("/api/login", post(api_login))
}

async fn api_login( cookies:Cookies, payload: Json<LoginRequest>) -> Result<Json<Value>>{
    println!("--> {:12} - api_login", "Handler");

    if (payload.username != "demo" || payload.password != "welcome"){
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
    let body = Json(json!({
"result": {
    "success": true
}
    }));

    return Ok(body);
}

#[derive(Debug, Deserialize)]
struct LoginRequest{
    username: String,
    password: String
}