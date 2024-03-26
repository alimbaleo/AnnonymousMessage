use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct User{
    pub id: u64,
    pub user_name: String,
    pub password: String,
    pub first_name: String,
    pub last_name:String
}

#[derive(Deserialize)]
pub struct CreateUserRequest{
    pub user_name: String,
    pub password: String,
    pub first_name: String,
    pub last_name:String
}

#[derive(Clone)]
pub struct ModelController{
    users_store: Arc<Mutex<Vec<Option<User>>>>
}

impl ModelController{
    pub async fn new() -> Result<Self>{
        Ok(Self{
            users_store: Arc::default()
        })
    }
}

impl ModelController{
    pub async fn create_user(&self, user_info: CreateUserRequest) -> Result<User>{
        let mut store = self.users_store.lock().unwrap();

        let id = store.len() as u64;
        let user = User{
            id,
            user_name: user_info.user_name,
            password: user_info.password,
            first_name: user_info.first_name,
            last_name: user_info.last_name
        };

        store.push(Some(user.clone()));

        Ok(user)
    }

    pub async fn get_user(&self, id: u64) -> Result<User>{
        let mut store = self.users_store.lock().unwrap();

        let user = store.get_mut(id as usize).and_then(|t| t.take());

        user.ok_or(Error::NotFound)
    }
}