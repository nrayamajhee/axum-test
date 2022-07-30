use crate::helpers::created_or_err;
use async_session::{Session, SessionStore};
use async_sqlx_session::PostgresSessionStore;
use axum::{
  extract::Extension, headers::Cookie, http::StatusCode, response::IntoResponse, Json, TypedHeader,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, query, query_as};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Post {
  pub author_username: String,
  pub author_name: String,
  pub author_email: String,
  pub title: String,
  pub slug: String,
  pub tags: Vec<String>,
  pub body: String,
}

pub async fn get_posts(Extension(pool): Extension<Arc<PgPool>>) -> impl IntoResponse {
  let posts = query_as!(Post, "
    select username as author_username, name as author_name, email as author_email, title, slug, tags, body
    from posts p join users u on p.author_id = u.id
  ")
    .fetch_all(&*pool)
    .await
    .unwrap();
  (StatusCode::OK, Json(posts))
}

#[derive(Deserialize)]
pub struct CreatePost {
  pub title: String,
  pub slug: String,
  pub tags: Vec<String>,
  pub body: String,
}

pub async fn create_post(
  Extension(store): Extension<Arc<PostgresSessionStore>>,
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
  let cookie = cookie.get("session").unwrap();
  if let Some(session) = store.load_session(cookie.to_owned()).await.unwrap() {
    let id: Uuid = session.get("user_id").unwrap();
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
    created_or_err("Post", res).into_response()
  } else {
    (
      StatusCode::OK,
      "You aren't logged in. Please login to create a post!",
    )
      .into_response()
  }
}
