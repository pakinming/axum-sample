use anyhow::Result;
use axum::{response::Html, routing::get, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "khunpk=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", port)).await?;

    tracing::info!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler() -> Html<&'static str> {
    tracing::info!("request handler");
    Html("<h1>Hello, World!</h1>")
}
