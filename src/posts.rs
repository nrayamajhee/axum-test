use crate::helpers::created_or_err;
use axum::{
  extract::Extension, headers::Cookie, http::StatusCode, response::IntoResponse, Json, TypedHeader,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, query, query_as};
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Post {
  pub id: Uuid,
  pub author_id: Uuid,
  pub title: String,
  pub slug: String,
  pub tags: Vec<String>,
  pub body: String,
}

pub async fn get_posts(Extension(pool): Extension<Arc<PgPool>>) -> impl IntoResponse {
  let users = query_as!(Post, "select * from posts")
    .fetch_all(&*pool)
    .await
    .unwrap();
  (StatusCode::OK, Json(users))
}

#[derive(Deserialize)]
pub struct CreatePost {
  pub title: String,
  pub slug: String,
  pub tags: Vec<String>,
  pub body: String,
}

pub async fn create_post(
  TypedHeader(cookie): TypedHeader<Cookie>,
  Extension(pool): Extension<Arc<PgPool>>,
  Json(payload): Json<CreatePost>,
) -> impl IntoResponse {
  let CreatePost {
    title,
    slug,
    tags,
    body,
  } = payload;
  let id = Uuid::from_str("07daa49e-a64c-4b20-96fa-07a2d56346b5").unwrap();
  tracing::debug!("ID {:?}", id);
  tracing::debug!("HEADER {:?}", cookie);
  println!("HEADER {:?}", cookie);
  let res: Result<(), sqlx::Error> = try {
    let mut transaction = pool.begin().await?;
    query!(
      "
            INSERT INTO posts
            (author_id, title, slug, tags, body)
            VALUES ($1, $2, $3, $4, $5)
          ",
      id,
      title,
      slug,
      &tags,
      body
    )
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
  };
  created_or_err("Post", res)
}
