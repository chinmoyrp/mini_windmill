pub mod db;

use axum::{
    Router, 
    routing::get, 
    handler::Handler,
    response::IntoResponse, 
    http::{StatusCode,Uri}};
use mongodb::Client;
use std::net::SocketAddr;
use anyhow::Result;

pub async fn run_server(addr: SocketAddr) -> Result<()> {

    let api_routes = Router::new().route("/steps", get(steps));
    let routes = Router::new()
                    .nest("/api", api_routes)
                    .fallback(fallback.into_service());

    axum::Server::bind(&addr).serve(routes.into_make_service()).await?;

    Ok(())
}

async fn steps() -> impl IntoResponse {
    let res = db::get_col().await;
    if let Ok(s) = res {
        return s;
    }
    String::from("Nothin")
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("Undefined route: {}", uri))
}