use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use serde::{Serialize, Deserialize};
use sqlx::{postgres::PgPool, query_as, query};
use std::{sync::Arc, str::FromStr};
use uuid::Uuid;

#[derive(Serialize)]
struct Post {
  id: Uuid,
  author_id: Uuid,
  title: String,
  slug: String,
  tags: Vec<String>,
  body: String
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
  title: String,
  slug: String,
  tags: Vec<String>,
  body: String
}

pub async fn create_post(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(payload): Json<CreatePost>,
) -> impl IntoResponse {
    let CreatePost {
      title,
      slug,
      tags,
      body
    } = payload;
    let id = Uuid::from_str("07daa49e-a64c-4b20-96fa-07a2d56346b5").unwrap();
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
    if let Err(err) = res {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            match err {
                sqlx::error::Error::Database(derr) => format!("Database Error {}", derr.message()),
                _ => format!("{:?}", err),
            },
        )
    } else {
        (StatusCode::CREATED, "Post created!".into())
    }
}
