use tracing_subscriber::EnvFilter;
use anyhow::{Result};
use tokio::net::TcpListener;
use axum::{response::Html, routing::get, Router};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // For early local development.
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();
        
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    
    // build our application with a route
    let app = Router::new().route("/", get(handler));


    // run it
    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}


