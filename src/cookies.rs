use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use axum::http::StatusCode;
use tower_cookies::{Cookie as TowerCookie, Cookies as TowerCookies};
use uuid::Uuid;

const COOKIE_NAME: &str = "stacky_sides_cookie";

pub struct Cookies {
    pub session_id: Uuid,
}

#[async_trait]
impl<B> FromRequest<B> for Cookies
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let extensions = req.extensions().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Can't extract cookies: extensions has been taken by another extractor",
        ))?;

        let cookies = extensions.get::<TowerCookies>().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Can't extract cookies. Is `CookieManagerLayer` enabled?",
        ))?;

        let session_id = cookies
            .get(COOKIE_NAME)
            .and_then(|cookie| cookie.value().parse().ok())
            .unwrap_or(Uuid::new_v4());

        cookies.add(TowerCookie::new(COOKIE_NAME, session_id.to_string()));

        Ok(Cookies { session_id })
    }
}
