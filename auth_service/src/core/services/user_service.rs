use core::ascii;
use std::ptr::null;

//use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use sea_orm::*;
use crate::core::entities::{user, user::Entity as User};
use crate::core::services::password_service;

pub struct UserService;
impl UserService{
   
    pub async fn create_user(
        db: &DbConn,
        user_name: &str,
        password: &str,
        first_name: &str,
        last_name: &str
    ) -> Result<user::ActiveModel, DbErr> {
        
        //hash password
        let password: String = password_service::hash_password(&password.to_string());
        //check if the username exists
        let user_name_exists = UserService::user_name_exists(db, user_name).await;
        if user_name_exists {
            return Err(DbErr::Custom("The specified username already exists".to_owned()));
        }
        //generate url
        user::ActiveModel {
            username: Set(user_name.to_string()),
            password: Set(password),
            first_name: Set(first_name.to_string()),
            last_name: Set(last_name.to_string()),
            profile_url: Set(user_name.to_string()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn validate_user(db: &DbConn, user_name: &String, password: &String) -> user::Model{

        let user = User::find().filter(user::Column::Username.eq(user_name)).one(db)
                            .await
                            .expect("could not find user")
                            .unwrap_or_else(|| panic!("could not find user"));
                        
        let is_pass_valid = password_service::verify_password(&user.password, password);

        if !is_pass_valid {
            panic!("The password is invalid");
        }

        return user;
    }
    pub async fn get_user_by_id(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }

    pub async fn get_user_by_profile_url(db: &DbConn, url: &str) -> Result<Option<user::Model>, DbErr> {
        User::find().filter(user::Column::ProfileUrl.eq(url)).one(db).await
    }

    async fn user_name_exists(db:&DbConn, user_name: &str) -> bool{
        let user = User::find().filter(user::Column::Username.eq(user_name)).count(db).await.unwrap_or(0);
        return  user > 0;
    }
}