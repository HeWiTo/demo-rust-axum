use axum::{
    routing::get,
    http::{
        StatusCode,
        Uri
    },
    response::IntoResponse,
    handler::Handler
};


#[tokio::main]
pub async fn main() {
    // Build our application by creating our router
    let app = axum::Router::new()
        .fallback(handler_404.into_service())
        .route("/",
            get(hello)
        );

    //let app = app.fallback(handler_404);

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

// Tokio signal handler that will wait for a user to press CTRL+C.
// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}

// axum handler for any request that fails to match the router routes.
// This implementation returns HTTP status code Not Found (404).
pub async fn handler_404(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

// axum handler for "GET /" which returns a string and causes axum to
// immediately respond with status code `200 OK` and with the string.
pub async fn hello() -> String {
    "Hello, World!".to_string()
}
