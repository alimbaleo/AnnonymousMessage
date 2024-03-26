use axum::http::Request;
use axum::body::Body;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::{Cookie, Cookies};

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth(cookies:Cookies, req: Request<Body>, next: Next) -> Result<Response>
{

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
   
    auth_token.ok_or(Error::AuthFailed)?;
   
    Ok(next.run(req).await)
}