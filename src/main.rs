use axum::{
    routing::get,
    http::{
        StatusCode,
        Uri
    },
    response::{
        IntoResponse,
        Html
    },
    handler::Handler
};


#[tokio::main]
pub async fn main() {
    // Build our application by creating our router
    let app = axum::Router::new()
        .fallback(handler_404.into_service())
        .route("/",
            get(hello)
        )
        .route("/demo.html", 
            get(get_demo_html)
        )
        .route("/hello.html", 
            get(hello_html)
        );

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

// axum handler for "GET /demo.html" which responds with HTML text.
// The `Html` type sets an HTTP header content-type of `text/html`.
pub async fn get_demo_html() -> Html<&'static str> {
    "<h1>Hello, this is HTML Text!</h1>".into()
}

// axum handler that responds with typical HTML coming from a file.
// This uses the Rust macro `std::include_str` to include a UTF-8 file
// path, relative to `main.rs`, as a `&'static str` at compile time.
async fn hello_html() -> Html<&'static str> {
    include_str!("hello.html").into()
}
