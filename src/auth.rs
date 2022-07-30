use argon2::verify_encoded;
use async_session::{Session, SessionStore};
use async_sqlx_session::PostgresSessionStore;
use axum::{
  extract::Extension,
  headers::Cookie,
  http::{header, StatusCode},
  response::IntoResponse,
  Json, TypedHeader,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, query_as};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
  pub username: Option<String>,
  pub email: Option<String>,
  pub password: String,
}

struct UserCheck {
  id: Uuid,
  password: String,
}

pub async fn login(
  header: Option<TypedHeader<Cookie>>,
  Extension(store): Extension<Arc<PostgresSessionStore>>,
  Extension(pool): Extension<Arc<PgPool>>,
  Json(payload): Json<LoginUser>,
) -> impl IntoResponse {
  let LoginUser {
    username,
    email,
    password,
  } = payload;
  if username.is_none() && email.is_none() {
    (
      StatusCode::BAD_REQUEST,
      "Either username or email along with password is required!",
    )
      .into_response()
  } else {
    let res = if let Some(username) = username {
      query_as!(
        UserCheck,
        "select id, password from users where username = $1",
        username
      )
      .fetch_one(&*pool)
      .await
      .unwrap()
    } else {
      query_as!(
        UserCheck,
        "select id, password from users where email = $1",
        email
      )
      .fetch_one(&*pool)
      .await
      .unwrap()
    };
    if verify_encoded(&res.password, &password.into_bytes()).unwrap() {
      let mut message = String::new();
      if let Some(TypedHeader(cookie)) = header {
        let cookie_value = cookie.get("session").unwrap();
        if let Some(session) = store
          .as_ref()
          .load_session(cookie_value.to_owned())
          .await
          .unwrap()
        {
          store.destroy_session(session).await.unwrap();
          message.push_str("\nExisting session destroyed!")
        }
      }
      let mut session = Session::new();
      let session_id = Uuid::new_v4();
      session.insert("session_id", session_id).unwrap();
      session.insert("user_id", res.id).unwrap();
      let cookie = store
        .as_ref()
        .store_session(session)
        .await
        .unwrap()
        .unwrap();
      let cookie = format!("session={}", cookie);
      (
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        format!("Authenticated!{}", message),
      )
        .into_response()
    } else {
      (StatusCode::UNAUTHORIZED, "Authenticated failed!").into_response()
    }
  }
}

pub async fn logout(
  TypedHeader(cookie): TypedHeader<Cookie>,
  Extension(store): Extension<Arc<PostgresSessionStore>>,
) -> impl IntoResponse {
  let cookie_value = cookie.get("session").unwrap();
  if let Some(session) = store
    .as_ref()
    .load_session(cookie_value.to_owned())
    .await
    .unwrap()
  {
    store.destroy_session(session).await.unwrap();
    (StatusCode::OK, "Logged out!")
  } else {
    (
      StatusCode::OK,
      "You aren't logged in. Your session might have expired.",
    )
  }
}
