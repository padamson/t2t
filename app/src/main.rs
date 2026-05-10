use axum::{Router, routing::get};
use tracing::info;

// CALLOUT: tokio-main `#[tokio::main]` desugars `main` into a synchronous wrapper that boots a Tokio runtime and blocks on the async body. Axum needs Tokio because every request handler is `async fn`.
#[tokio::main]
async fn main() {
    init_tracing();

    // CALLOUT: port-env Read the port from the `PORT` environment variable, falling back to 3000 (the cargo-leptos default we'll use in Ch 4). PORT is the convention every cloud platform (AWS App Runner, Fly.io, Heroku) sets for the container's exposed port.
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);
    let addr = format!("0.0.0.0:{port}");

    // CALLOUT: router-route `Router::new().route(path, handler)` is Axum's declarative routing. `get(...)` wraps a handler so Axum knows it answers HTTP GET. We add more routes by chaining `.route(...)` calls.
    let app = Router::new().route("/health", get(health));

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind {addr}: {e}"));

    info!(%addr, "scimantic-server starting");
    axum::serve(listener, app)
        .await
        .expect("server crashed");
}

// CALLOUT: instrument `#[tracing::instrument]` wraps the function body in a span named `health`. Every `info!`/`warn!`/`error!` log inside the function carries that span's context, so log aggregators can group all messages from one request together.
#[tracing::instrument]
async fn health() -> &'static str {
    info!("health check");
    "ok"
}

// CALLOUT: tracing-init Two layers: an `EnvFilter` (reads `RUST_LOG`, defaults to `info`) and a `fmt` layer (human-readable output for development; we'll switch to JSON for production in Ch 12). The `registry` is what ties layers together.
fn init_tracing() {
    use tracing_subscriber::{EnvFilter, fmt, prelude::*};

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();
}
