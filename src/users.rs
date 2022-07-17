use crate::helpers::created_or_err;
use argon2::{hash_encoded, Config};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, query, query_as};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
  pub username: String,
  pub name: String,
  pub email: String,
  pub bio: String,
}

pub async fn get_users(Extension(pool): Extension<Arc<PgPool>>) -> impl IntoResponse {
  let users = query_as!(User, "select username, name, email, bio from users")
    .fetch_all(&*pool)
    .await
    .unwrap();
  (StatusCode::OK, Json(users))
}

#[derive(Deserialize)]
pub struct CreateUser {
  pub username: String,
  pub password: String,
  pub name: String,
  pub email: String,
  pub bio: Option<String>,
}

pub async fn create_user(
  Extension(pool): Extension<Arc<PgPool>>,
  Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
  let CreateUser {
    username,
    password,
    name,
    email,
    bio,
  } = payload;
  let bio = if let Some(bio) = bio { bio } else { "".into() };
  let res: Result<(), sqlx::Error> = try {
    let salt = Uuid::new_v4().to_string().into_bytes();
    let password = password.into_bytes();
    let config = Config::default();
    let hashed_password = hash_encoded(&password, &salt, &config).unwrap();
    let mut transaction = pool.begin().await?;
    query!(
      "
            INSERT INTO users
            (username, password, name, email, bio)
            VALUES ($1, $2, $3, $4, $5)
          ",
      username,
      hashed_password,
      name,
      email,
      bio
    )
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
  };
  created_or_err("User", res)
}
