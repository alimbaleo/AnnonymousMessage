use crate::core::entities::user;
use crate::core::entities::user::ActiveModel;
use crate::core::services::user_service::UserService;
use axum::extract::State;
use axum::Router;
use axum::extract::Path;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;
use serde::Deserialize;
use axum::routing::post;
use axum::routing::get;
use axum::Json;
use crate::web;
use crate::{Error, Result};


pub fn routes(db_connection: DatabaseConnection) -> Router{
    Router::new()
    .route("/register", post(create_user))
    .route("/user/:user_id", get(get_user))
    .route("/profile/:url", get(get_user_by_profile_url))
    .with_state(db_connection)
}    
async fn create_user(State(db): State<DatabaseConnection>, Json(input): Json<CreateUserRequest>) -> Result<Json<UserDto>>{

    println!("->> {:<12} - create_user", "HANDLER");
    let result = UserService::create_user(&db, &input.user_name, &input.password, &input.first_name,&input.last_name).await.unwrap();

    let user = UserDto{
        first_name : result.first_name.unwrap(),
        id : result.id.unwrap(),
        profile_url : result.profile_url.unwrap(),
        last_name:result.last_name.unwrap(),
        user_name : result.username.unwrap()
    };
return Ok(Json(user));
}


async fn get_user(State(db): State<DatabaseConnection>, Path(user_id): Path<i32>) -> Result<Json<UserDto>>{
   
    println!("->> {:<12} - get_user", "HANDLER");
    let result:user::Model  = UserService::get_user_by_id(&db, user_id)
                            .await
                            .expect("could not find user")
                            .unwrap_or_else(|| panic!("could not find user with id {user_id}"));
                        
    let user = UserDto{
        first_name : result.first_name,
        id : result.id,
        profile_url : result.profile_url,
        last_name:result.last_name,
        user_name : result.username
    };
    return Ok(Json(user));
    }
    
async fn get_user_by_profile_url(State(db): State<DatabaseConnection>, Path(url): Path<String>) -> Result<Json<UserSummaryDto>>{
   
    println!("->> {:<12} - get_user", "HANDLER");
    let result:user::Model  = UserService::get_user_by_profile_url(&db, &url)
                            .await
                            .expect("could not find user")
                            .unwrap_or_else(|| panic!("could not find user with url {url}"));
                        
    let user = UserSummaryDto {
        id : result.id,
        user_name : result.username
    };
    return Ok(Json(user));
    }
#[derive(Deserialize, Serialize)]
pub struct CreateUserRequest{
    pub user_name: String,
    pub password: String,
    pub first_name: String,
    pub last_name:String
}
#[derive(Deserialize, Serialize)]
pub struct UserDto{
    pub user_name: String,
    pub first_name: String,
    pub last_name:String,
    pub profile_url:String,
    pub id:i32
}
#[derive(Deserialize, Serialize)]
pub struct UserSummaryDto{
    pub user_name: String,
    pub id:i32
}