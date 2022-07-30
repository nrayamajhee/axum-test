use async_sqlx_session::PostgresSessionStore;
use axum::{http::StatusCode, response::IntoResponse};
use sqlx::error::Error;
use std::sync::Arc;
use std::time::Duration;

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

pub fn clean_up_intermittently(store: Arc<PostgresSessionStore>, period: Duration) {
  tokio::spawn(async move {
    loop {
      tokio::time::sleep(period).await;
      if let Err(error) = store.cleanup().await {
        tracing::error!("cleanup error: {}", error);
      }
    }
  });
}
