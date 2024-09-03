use tracing_subscriber::EnvFilter;
use anyhow::{Result};
use tokio::net::TcpListener;
use axum::{response::Html, routing::get, Router};


// #[tokio::main]
// async fn main() -> Result<()> {
//     tracing_subscriber::fmt()
//         // For early local development.
//         .with_target(false)
//         .with_env_filter(EnvFilter::from_default_env())
//         .pretty()
//         .init();
//     log::debug!("Hello, world!");
//     let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

//     Ok(())
// }


#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}


