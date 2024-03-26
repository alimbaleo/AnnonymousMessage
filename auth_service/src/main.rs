#![allow(unused)]
pub use self::error::{Error, Result};

use std::net::SocketAddr;
use axum::extract::{Path, Query};
use axum::response::{Html, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Router};
use axum::response::IntoResponse;
use model::ModelController;
use serde::Deserialize;
use tower_http::services::ServeDir;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
mod error;
mod web;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    
    let mc = ModelController::new().await?;
    let routes_apis = web::routes_user::routes(mc.clone())
                                .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
                            
let routes_all:Router = Router::new()
                                .merge(web::routes_login::routes())
                                .layer(middleware::map_response(main_response_mapper))
                                .layer(CookieManagerLayer::new())
                                .nest("/api", routes_apis)
                                .fallback_service(routes_default());

let listner = tokio::net::TcpListener::bind("127.0.0.1:5500").await.unwrap();
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