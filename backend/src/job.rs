use bson::{doc};
use serde::{Serialize, Deserialize};

use crate::step::{self, Step, Output};
use crate::flow::Flow;
use crate::db;

#[derive(Debug, Serialize, Deserialize)]
pub enum JobKind {
    Step(String),
    Flow(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JobStatus {
    Created(String),
    Running(String),
    Finished(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub kind: JobKind,
    pub status: JobStatus,
    pub worker_id: u32,
}

pub async fn get_step(hash: &str) -> Option<Step> {
    let coll = db::get_collection::<Step>("steps").await;
    coll.find_one(doc!{ "hash": hash}, None).await.unwrap()
}

pub async fn get_flow() -> Option<Flow> {
    let coll = db::get_collection::<Flow>("flow").await;
    coll.find_one(None, None).await.unwrap()
}

async fn update_job_status(id: &str, status: JobStatus) {
    let coll = db::get_collection::<Job>("jobs").await;
    let filter = doc!{ "id": id };
    let job = coll.find_one(filter.clone(), None).await.unwrap();
    if let Some(mut job) = job {
        job.status = status;
        coll.find_one_and_replace(filter, job, None).await.unwrap();
    }
}

async fn insert_output_stream_for_job(id: &str) {
    let coll = db::get_collection::<Output>(id).await;
    let res = coll.find_one(None, None).await.unwrap();
    if res.is_none() {
        let empty: Vec<String> = Vec::new();
        let out = Output{ output: empty };
        coll.insert_one(out, None).await.unwrap();
    }
}

pub async fn run(job: Job) {
    insert_output_stream_for_job(&job.id).await;
    update_job_status(&job.id, JobStatus::Running(db::get_current_datetime())).await;

    match job.kind {
        JobKind::Step(hash)  => {
            let step = get_step(&hash).await;
            if let Some(step) = step {
                db::log(&format!("Executing step: {}", step.hash)).await;
                step::execute(&job.id, step).await;
            }
        }

        JobKind::Flow(_) => { 
            let flow = get_flow().await;
            if let Some(Flow{ steps }) = flow {
                db::log("Executing flow").await;
                for hash in steps {
                    let step = get_step(&hash).await;
                    // let coll = db::get_collection::<Output>(&hash).await;
                    // let res = coll.find_one(None, None).await.unwrap();
                    // if let Some(mut out) = res {
                    //     let query = doc!{ "output": &out.output };
                    //     out.output.push(format!("Executing step: {}", step.hash));
                    //     coll.find_one_and_replace(query, out, None).await.unwrap();
                    // }
                    if let Some(step) = step {
                        step::execute(&job.id, step).await;
                    }
                }
            }
        }
    }
    update_job_status(&job.id, JobStatus::Finished(db::get_current_datetime())).await;
}