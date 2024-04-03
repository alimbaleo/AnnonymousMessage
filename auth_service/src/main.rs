#![allow(unused)]
pub use self::error::{Error, Result};

use std::net::SocketAddr;
use axum::extract::{Path, Query};
use axum::response::{Html, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Router};
use axum::response::IntoResponse;
use sea_orm::Database;
use core::services::user_service::UserService;
use serde::Deserialize;
use tower_http::services::ServeDir;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use std::{env, string};
mod error;
mod web;
mod core;
extern crate argon2;


#[tokio::main]
async fn main() -> Result<()> {
    
   dotenvy::dotenv().ok();
   let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
   let port = env::var("PORT").expect("Server port not set");
   let host = env::var("HOST").expect("Server host not set");
   let server_url = format!("{host}:{port}");
   println!("->> {:<12} - is - {db_url:?} ", "DB_URL");
   println!("->> {:<12} - is - {server_url:?} ", "SERVER_URL");
   let conn = Database::connect(db_url)
   .await
   .expect("Database connection failed");
    let routes_apis = web::routes_user::routes(conn.clone())
                                .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

let routes_all:Router = Router::new()
                                .merge(web::routes_login::routes(conn.clone()))
                                .layer(middleware::map_response(main_response_mapper))                                
                                .nest("/api", routes_apis)
                                .layer(CookieManagerLayer::new())
                                .fallback_service(routes_default());

let listner = tokio::net::TcpListener::bind(&server_url).await.unwrap();
axum::serve(listner, routes_all).await.unwrap();

return Ok(());
}
async fn main_response_mapper(res: Response) -> Response{

    println!("->> {:<12} - main_response_mapper - {res:?} ", "RES_MAPPER");

    println!();

    res

}

fn routes_default() -> Router{
   // todo!("Implement default response for not found");
Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, Deserialize)]
struct  HelloParams{
name: Option<String>
}