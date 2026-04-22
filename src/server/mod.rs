use std::path::PathBuf;
use std::{env::args, net::SocketAddr};

use axum::{Router, routing::get};
#[cfg(debug_assertions)]
use axum::{
    body::Body,
    extract::Request,
    response::{IntoResponse, Response},
};
#[cfg(debug_assertions)]
use reqwest::StatusCode;

pub async fn start(app: Router) {
    let port = args().nth(1).unwrap_or("3000".to_string()).parse().unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind HTTP listener");

    eprintln!("Listening on http://{addr}");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("HTTP server exited unexpectedly");
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    eprintln!("Shutdown signal received, stopping server...");
}

fn api_routes() -> Router {
    Router::new().route("/health", get(|| async { "OK" }))
}

#[cfg(not(debug_assertions))]
pub fn app(asset_dir: PathBuf) -> Router {
    use tower_http::services::{ServeDir, ServeFile};

    let index_file = asset_dir.join("index.html");
    let spa = ServeDir::new(&asset_dir)
        .append_index_html_on_directories(true)
        .fallback(ServeFile::new(index_file));
    // SPA fallback: any unmatched path returns index.html so Vue Router handles it
    let spa_fallback = axum::routing::get_service(spa);

    Router::new()
        .nest("/api", api_routes())
        .fallback_service(spa_fallback)
}

#[cfg(debug_assertions)]
pub fn app(_asset_dir: PathBuf) -> Router {
    Router::new()
        .nest("/api", api_routes())
        .fallback(proxy_to_vite)
}

#[cfg(debug_assertions)]
async fn proxy_to_vite(req: Request) -> Response {
    use reqwest::Client;

    const VITE: &str = "http://127.0.0.1:5173";

    let client = Client::new();
    let uri = req.uri();

    let target = format!(
        "{VITE}{}",
        uri.path_and_query().map(|p| p.as_str()).unwrap_or("/")
    );

    let method = reqwest::Method::from_bytes(req.method().as_str().as_bytes()).unwrap();
    let headers = req.headers().clone();
    let body = req.into_body();
    let bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .unwrap_or_default();

    let mut rb = client.request(method, &target).body(bytes);
    for (k, v) in &headers {
        rb = rb.header(k, v);
    }

    match rb.send().await {
        Ok(resp) => {
            let status = axum::http::StatusCode::from_u16(resp.status().as_u16())
                .unwrap_or(StatusCode::BAD_GATEWAY);
            let headers = resp.headers().clone();
            let body = resp.bytes().await.unwrap_or_default();

            let mut response = Response::new(Body::from(body));
            *response.status_mut() = status;
            for (k, v) in &headers {
                response.headers_mut().insert(k, v.clone());
            }
            response
        }
        Err(e) => (StatusCode::BAD_GATEWAY, format!("Vite proxy error: {e}")).into_response(),
    }
}
