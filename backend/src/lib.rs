pub mod db;
pub mod step;
pub mod flow;
pub mod handler;
pub mod worker;
pub mod job;

use axum::{
    Router, 
    routing::{get, post}, 
    handler::Handler,
    response::IntoResponse, 
    http::{StatusCode,Uri}};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use anyhow::Result;

pub async fn run_server(addr: SocketAddr) -> Result<()> {

    worker::start_workers(3).await?;

    let api_routes = Router::new()
                            .route("/steps", get(handler::get_available_steps).post(handler::add_step))
                            .route("/steps/:hash", get(handler::get_step).post(fallback))
                            .route("/flow", get(handler::get_current_flow).post(handler::add_to_flow))
                            .route("/flow/remove", get(fallback).post(handler::remove_from_flow))
                            .route("/jobs", get(handler::get_jobs).post(handler::add_job))
                            .route("/jobs/:hash", get(handler::get_job).post(fallback))
                            .route("/workers", get(handler::get_workers))
                            .route("/logs", get(handler::get_logs));

    let routes = Router::new()
                    .nest("/api", api_routes)
                    .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
                    .fallback(fallback.into_service());

    axum::Server::bind(&addr).serve(routes.into_make_service()).await?;

    Ok(())
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("Undefined route: {}\n", uri))
}