use axum::extract::FromRequestParts;
use axum::http::Request;
use axum::body::Body;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth(cookies:Cookies, req: Request<Body>, next: Next) -> Result<Response>
{

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
   
    let (user_id, exp, sign) = auth_token.ok_or(Error::AuthFailed).and_then(parse_token)?;
  
    Ok(next.run(req).await)
}

fn parse_token(token: String) -> Result<(u64, String, String)>{
    let (whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(Error::InvalidToken)?;

    let user_id:u64 = user_id.parse().map_err(|_| Error::InvalidToken)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
