use axum::{Json, response::IntoResponse, extract::Path};
use bson::{doc, Document};
use futures::TryStreamExt;

use crate::step::Step;
use crate::flow::Flow;
use crate::worker::Worker;
use crate::job::{Job, JobStatus};
use crate::db::{self, JsonResult};

macro_rules! json_result {
    ($a:expr) => { serde_json::to_string(&$a).unwrap() }
}

pub async fn get_available_steps() -> impl IntoResponse {
    let cursor = db::get_collection::<Step>("steps").await.find(None, None).await.unwrap();
    let v: Vec<_> = cursor.try_collect().await.unwrap_or_default();
    json_result!(v)
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

pub async fn get_step(Path(hash): Path<String>) -> impl IntoResponse {
    let coll = db::get_collection::<Step>("steps").await;
    let res = coll.find_one(doc!{ "hash": hash }, None).await.unwrap();
    if let Some(step) = res {
        json_result!(step)
    } else {
        json_result!("No step with matching hash found")
    }
}

pub async fn add_step(Json(mut step): Json<Step>) -> impl IntoResponse {
    step.hash = format!("{:x}", md5::compute(format!("{}{}", step.name, step.code).as_bytes()));
    let coll = db::get_collection::<Step>("steps").await;
    let existing = coll.find_one(doc!{ "hash": &step.hash }, None).await.unwrap();
    if existing.is_none() {
        if let Err(res) = coll.insert_one(&step, None).await {
            eprintln!("Error adding a step: {:?}", res);
            return JsonResult::from(vec!["error"]);
        }
        db::log(format!("Added a step: {} ({})", step.name, step.hash).as_str()).await;
    }
    JsonResult::from(vec![])
}

pub async fn add_to_flow(Json(hash): Json<String>) -> impl IntoResponse {
    let steps = db::get_collection::<Step>("steps").await;
    let step = steps.find_one(doc!{ "hash": &hash }, None).await.unwrap();

    if step.is_none() {
        return JsonResult::from(vec!["No step with this hash exist"]);
    }

    let coll = db::get_collection::<Flow>("flow").await;
    let flow = coll.find_one(None, None).await.unwrap();
    let step = step.unwrap();
    if flow.is_none() {
        let flow = Flow{ steps: vec![step.hash.clone()] };
        if let Err(e) = coll.insert_one(flow, None).await {
            eprintln!("Error: {:?}", e);
            return JsonResult::from(vec!["Error inserting flow"]);
        }
    } else {
        let mut flow = flow.unwrap();
        let query = doc!{ "steps": &flow.steps };
        flow.steps.push(hash.to_string());
        let update = doc!{"$set": { "steps": &flow.steps }};
        if let Err(e) = coll.update_one(query, update, None).await {
            eprintln!("Error: {:?}", e);
            return JsonResult::from(vec!["Error adding the step to flow"]);
        }

    }

    db::log(format!("Added to flow: {} ({})", step.name, step.hash).as_str()).await;

    JsonResult::from(vec!["success"])
}

pub async fn remove_from_flow(Json(index): Json<String>) -> impl IntoResponse {
    let coll = db::get_collection::<Flow>("flow").await;
    let flow = coll.find_one(None, None).await.unwrap();

    if flow.is_none() {
        return JsonResult::from(vec!["No flow exist"]);
    }

    let mut flow = flow.unwrap();
    let index = match index.parse::<usize>() {
        Ok(i) if i < flow.steps.len() => i,
        _ => return JsonResult::from(vec!["Invalid index"])
    };

    let query = doc!{ "steps": &flow.steps };
    flow.steps.remove(index);
    let update = doc!{"$set": { "steps": &flow.steps }};
    if let Err(e) = coll.update_one(query, update, None).await {
        eprintln!("Error: {:?}", e);
        return JsonResult::from(vec!["Error updating flow"]);
    }

    JsonResult::from(vec!["success"])
}

pub async fn add_job(Json(mut job): Json<Job>) -> impl IntoResponse {
    job.status = JobStatus::Created(db::get_current_datetime());
    let job_str = serde_json::to_string(&job).unwrap();
    job.id = format!("{:x}", md5::compute(job_str.as_bytes()));
    let coll = db::get_collection::<Job>("jobs").await;

    if let Err(res) = coll.insert_one(&job, None).await {
        eprintln!("Error adding job to queue: {:?}", res);
        return JsonResult::from(vec!["error"]);
    }
    db::log(format!("Added job to queue: ({})", job_str).as_str()).await;

    JsonResult::from(vec!["success"])
}

