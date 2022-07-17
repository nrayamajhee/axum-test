use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, query, query_as};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize)]
struct User {
    id: Uuid,
    username: String,
    name: String,
    email: String,
    bio: String,
    verified: bool,
}

pub async fn get_users(Extension(pool): Extension<Arc<PgPool>>) -> impl IntoResponse {
    let users = query_as!(User, "select * from users")
        .fetch_all(&*pool)
        .await
        .unwrap();
    (StatusCode::OK, Json(users))
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    name: String,
    email: String,
    bio: Option<String>,
}

pub async fn create_user(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let CreateUser {
        username,
        name,
        email,
        bio,
    } = payload;
    let bio = if let Some(bio) = bio { bio } else { "".into() };
    let res: Result<(), sqlx::Error> = try {
        let mut transaction = pool.begin().await?;
        query!(
            "
            INSERT INTO users
            (username, name, email, bio)
            VALUES ($1, $2, $3, $4)
          ",
            username,
            name,
            email,
            bio
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
        (StatusCode::CREATED, "User created!".into())
    }
}
