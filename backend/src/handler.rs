use axum::{Json, response::IntoResponse, extract::Path};
use bson::{doc, Document};
use futures::TryStreamExt;
use std::collections::HashMap;
use chrono::Utc;

use crate::step::{Step, Output};
use crate::flow::Flow;
use crate::worker::Worker;
use crate::job::{self, Job, JobStatus, JobKind};
use crate::db;

macro_rules! json_result {
    ($a:expr) => {
        {
            let mut map = HashMap::new();
            map.insert("result", $a);
            Json(map).into_response()
        }
    }
}

macro_rules! compute_hash {
    ($a:expr) => {{
        format!("{:x}", md5::compute(format!("{:?}{:?}", $a, Utc::now()).as_bytes()))
    }};
}

pub async fn get_available_steps() -> impl IntoResponse {
    let cursor = db::get_collection::<Step>("steps").await.find(None, None).await.unwrap();
    let v: Vec<_> = cursor.try_collect().await.unwrap_or_default();
    json_result!(v)
}

pub async fn get_step(Path(hash): Path<String>) -> impl IntoResponse {
    let res = job::get_step(&hash).await;
    match res {
        Some(step) => json_result!(step),
        None => json_result!("No step with matching hash found")
    }
}

pub async fn get_current_flow() -> impl IntoResponse {
    let cursor = db::get_collection::<Flow>("flow").await.find(None, None).await.unwrap();
    let v: Vec<_> = cursor.try_collect().await.unwrap_or_default();
    json_result!(v)
}

pub async fn get_logs() -> impl IntoResponse {
    let cursor = db::get_collection::<Document>("logs").await.find(None, None).await.unwrap();
    let v: Vec<_> = cursor.try_collect().await.unwrap_or_default();
    json_result!(v)
}

pub async fn get_workers() -> impl IntoResponse {
    let cursor = db::get_collection::<Worker>("workers").await.find(None, None).await.unwrap();
    let v: Vec<_> = cursor.try_collect().await.unwrap_or_default();
    json_result!(v)
}

pub async fn get_jobs() -> impl IntoResponse {
    let cursor = db::get_collection::<Job>("jobs").await.find(None, None).await.unwrap();
    let v: Vec<_> = cursor.try_collect().await.unwrap_or_default();
    json_result!(v)
}

pub async fn get_job(Path(id): Path<String>) -> impl IntoResponse {
    let coll = db::get_collection::<Output>(&id).await;
    let res = coll.find_one(None, None).await.unwrap();
    match res {
        Some(doc) => json_result!(doc),
        None => json_result!("No job with matching hash found")
    }
}

pub async fn add_step(Json(mut step): Json<Step>) -> impl IntoResponse {
    step.hash = compute_hash!(step);
    let coll = db::get_collection::<Step>("steps").await;
    let existing = coll.find_one(doc!{ "hash": &step.hash }, None).await.unwrap();
    if existing.is_none() {
        if let Err(res) = coll.insert_one(&step, None).await {
            eprintln!("Error adding a step: {:?}", res);
            return json_result!("error");
        }
        db::log(format!("Added a step: {} ({})", step.name, step.hash).as_str()).await;
    }
    json_result!(step.hash)
}

pub async fn add_to_flow(Json(hash): Json<String>) -> impl IntoResponse {
    let steps = db::get_collection::<Step>("steps").await;
    let step = steps.find_one(doc!{ "hash": &hash }, None).await.unwrap();

    if step.is_none() {
        return json_result!("No step with this hash exist");
    }

    let coll = db::get_collection::<Flow>("flow").await;
    let flow = coll.find_one(None, None).await.unwrap();
    let step = step.unwrap();
    if flow.is_none() {
        let flow = Flow{ steps: vec![step.hash.clone()] };
        if let Err(_e) = coll.insert_one(flow, None).await {
            return json_result!("Error inserting flow");
        }
    } else {
        let mut flow = flow.unwrap();
        let query = doc!{ "steps": &flow.steps };
        flow.steps.push(hash.to_string());
        let update = doc!{"$set": { "steps": &flow.steps }};
        if let Err(_e) = coll.update_one(query, update, None).await {
            return json_result!("Error adding the step to flow");
        }

    }

    db::log(&format!("Added to flow: {} ({})", step.name, step.hash)).await;

    json_result!(step)
}

pub async fn remove_from_flow(Json(index): Json<String>) -> impl IntoResponse {
    let coll = db::get_collection::<Flow>("flow").await;
    let flow = coll.find_one(None, None).await.unwrap();

    if flow.is_none() {
        return json_result!("No flow exist");
    }

    let mut flow = flow.unwrap();
    let index = match index.parse::<usize>() {
        Ok(i) if i < flow.steps.len() => i,
        _ => return json_result!("Invalid index")
    };

    let query = doc!{ "steps": &flow.steps };
    flow.steps.remove(index);
    let update = doc!{"$set": { "steps": &flow.steps }};
    if let Err(e) = coll.update_one(query, update, None).await {
        eprintln!("Error: {:?}", e);
        return json_result!("Error updating flow");
    }

    json_result!("success")
}

pub async fn add_job(Json(mut job): Json<Job>) -> impl IntoResponse {
    job.status = JobStatus::Created(db::get_current_datetime());
    job.id = compute_hash!(job);
    job.worker_id = 0;
    let coll = db::get_collection::<Job>("jobs").await;

    if let JobKind::Step(hash) = &job.kind {
        let step = job::get_step(hash).await;
        if step.is_none() {
            return json_result!("No step with matching hash found");
        }
    }

    if let Err(res) = coll.insert_one(&job, None).await {
        eprintln!("Error adding job to queue: {:?}", res);
        return json_result!("error");
    }
    db::log(&format!("Added a job to queue: id({})", job.id)).await;

    json_result!(job.id)
}

