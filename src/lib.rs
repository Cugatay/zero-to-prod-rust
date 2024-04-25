use axum::{
    http::StatusCode,
    response::Result as AxumResult,
    routing::{get, post},
    serve::Serve,
    Router,
};
use serde::Deserialize;

pub fn run(listener: tokio::net::TcpListener) -> Result<Serve<Router, Router>, std::io::Error> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    println!("listening on {}", listener.local_addr().unwrap());

    // let test = axum::serve(listener, app);
    // let test = test.await;
    // let test = test.unwrap();

    let server = axum::serve(listener, app);

    Ok(server)
}

async fn health_check() -> AxumResult<StatusCode> {
    Ok(StatusCode::OK)
    // Err(ErrorResponse::from(StatusCode::OK))
}

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: axum::Form<FormData>) -> AxumResult<StatusCode> {
    Ok(StatusCode::OK)
}
