use axum::{http::StatusCode, response::IntoResponse};
use sqlx::error::Error;

pub fn created_or_err(resource: &str, res: Result<(), sqlx::Error>) -> impl IntoResponse {
  if let Err(err) = res {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      if let Error::Database(derr) = err {
        format!("Database Error {}", derr.message())
      } else {
        format!("{:?}", err)
      },
    )
  } else {
    (StatusCode::CREATED, format!("{} created!", resource))
  }
}
