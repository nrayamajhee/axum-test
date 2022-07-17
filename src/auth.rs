use argon2::verify_encoded;
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, query_as};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
  pub username: Option<String>,
  pub email: Option<String>,
  pub password: String,
}

struct UserCheck {
  password: String,
}

pub async fn login(
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
        "select password from users where username = $1",
        username
      )
      .fetch_one(&*pool)
      .await
      .unwrap()
    } else {
      query_as!(
        UserCheck,
        "select password from users where email = $1",
        email
      )
      .fetch_one(&*pool)
      .await
      .unwrap()
    };
    let res = if verify_encoded(&res.password, &password.into_bytes()).unwrap() {
      "Authenticated!"
    } else {
      "Authentication failed!"
    };
    (StatusCode::OK, res).into_response()
  }
}
