use axum::{
  routing::{get, post},
  Router,
};

use crate::{
  auth::login,
  posts::{create_post, get_posts},
  users::{create_user, get_users},
};

pub fn routes() -> Router {
  Router::new()
    .route("/", get(ok))
    .route("/users", get(get_users).post(create_user))
    .route("/posts", get(get_posts).post(create_post))
    .route("/login", post(login))
}

async fn ok() -> &'static str {
  "OK"
}
