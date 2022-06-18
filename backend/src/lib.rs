pub mod db;
pub mod step;
pub mod flow;
pub mod handler;
pub mod worker;
pub mod job;

use axum::{
    Router, 
    routing::get, 
    handler::Handler,
    response::IntoResponse, 
    http::{StatusCode, Uri, Method}};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;
use anyhow::Result;

pub async fn run_server(addr: SocketAddr) -> Result<()> {

    worker::start_workers(3).await?;

    let api_routes = Router::new()
                            .route("/steps", get(handler::get_available_steps).post(handler::add_step))
                            .route("/steps/:hash", get(handler::get_step).post(fallback))
                            .route("/steps/remove/:hash", get(handler::remove_step).post(fallback))
                            .route("/steps/update", get(fallback).post(handler::update_step))
                            .route("/flow", get(handler::get_current_flow).post(fallback))
                            .route("/flow/add/:hash", get(handler::add_to_flow).post(fallback))
                            .route("/flow/remove/:hash", get(handler::remove_from_flow).post(fallback))
                            .route("/jobs", get(handler::get_jobs).post(handler::add_job))
                            .route("/jobs/:hash", get(handler::get_job).post(fallback))
                            .route("/workers", get(handler::get_workers).post(fallback))
                            .route("/logs", get(handler::get_logs).post(fallback))
                            .route("/spec", get(handler::get_openapi_spec).post(fallback));

    let cors = CorsLayer::very_permissive();

    let routes = Router::new()
                    .nest("/api", api_routes)
                    .layer(ServiceBuilder::new().layer(cors))
                    .fallback(fallback.into_service());

    axum::Server::bind(&addr).serve(routes.into_make_service()).await?;

    Ok(())
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("Undefined route: {}\n", uri))
}