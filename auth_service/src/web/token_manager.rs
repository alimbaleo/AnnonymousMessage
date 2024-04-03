
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Serialize, Deserialize};
use crate::Error as AppError;
use std::env;
use jsonwebtoken::errors::Error;

#[derive(Deserialize, Serialize)]
pub struct Claims{
    pub user_name: String,
    pub exp:u64,
    pub first_name: String,
    pub last_name: String,
    pub id:i32
}

/* 
#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = AppError;

    async fn from_request(req: &mut FromRequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AppError::InvalidToken)?;
            dotenvy::dotenv().ok();
           
        Ok(data.claims)
    }
}*/
pub struct Keys{
    pub encoding:EncodingKey,
    pub decoding:DecodingKey,
}

impl  Keys {
    pub fn new(secret: &[u8]) -> Self{
        Self{
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret)
        }
    }
}

pub struct TokenManager{}

impl TokenManager{
    pub fn generate_token(claim: &Claims) -> Result<String, Error>{
        dotenvy::dotenv().ok();
    let encoding_secret = env::var("ENCODING_KEY").expect("ENCODING_KEY is not set in .env file");
    let token = encode(&Header::default(), &claim, &Keys::new(encoding_secret.as_bytes()).encoding);

    return token;
    }

    pub fn validate_token(token: String) -> Result<TokenData<Claims>, AppError>{
        dotenvy::dotenv().ok();

    let decoding_secret = env::var("DECODING_KEY").expect("DECODING_KEY is not set in .env file");
    let data = decode::<Claims>(&token, &Keys::new(decoding_secret.as_bytes()).decoding, &Validation::default())
        .map_err(|_| AppError::InvalidToken);
    return data;
    }
}