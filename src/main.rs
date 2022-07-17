#![feature(try_blocks)]
use axum::extract::Extension;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

mod routes;
mod users;
mod posts;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let db_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    let pool = Arc::new(pool);

    let app = routes::routes().layer(Extension(pool));

    let port: anyhow::Result<u16> = try { env::var("PORT")?.parse()? };
    let addr = SocketAddr::from(([127, 0, 0, 1], port.unwrap_or(8080)));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
