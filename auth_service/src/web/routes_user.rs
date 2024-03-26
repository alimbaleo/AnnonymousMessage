use crate::model::{ModelController, User, CreateUserRequest};
use axum::extract::State;
use axum::Router;
use axum::extract::Path;
use serde_json::Value;
use serde_json::json;
use axum::routing::post;
use axum::routing::get;
use axum::Json;
use crate::web;
use crate::{Error, Result};


pub fn routes(mc: ModelController) -> Router{
    Router::new()
    .route("/register", post(create_user))
    .route("/user/:user_id", get(get_user))
    .with_state(mc)
}    
async fn create_user(State(mc): State<ModelController>, Json(input): Json<CreateUserRequest>) -> Result<Json<User>>{

    println!("->> {:<12} - create_user", "HANDLER");
    let user = mc.create_user(input).await?;

return Ok(Json(user));
}


async fn get_user(State(mc): State<ModelController>, Path(user_id): Path<u64>) -> Result<Json<User>>{
   
    println!("->> {:<12} - get_user", "HANDLER");
    let user = mc.get_user(user_id).await?;
    
    return Ok(Json(user));
    }
    