use axum::{routing::get, Router};

use crate::users::{create_user, get_users};
use crate::posts::{get_posts, create_post};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(ok))
        .route("/users", get(get_users).post(create_user))
        .route("/posts", get(get_posts).post(create_post))
}

async fn ok() -> &'static str {
    "OK"
}
