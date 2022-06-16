use bson::doc;
use serde::{Serialize, Deserialize};

use crate::step::{self, Step};
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

async fn get_step(hash: &str) -> Option<Step> {
    let coll = db::get_collection::<Step>("steps").await;
    coll.find_one(doc!{ "hash": hash}, None).await.unwrap()
}

async fn get_flow() -> Option<Flow> {
    let coll = db::get_collection::<Flow>("flow").await;
    coll.find_one(None, None).await.unwrap()
}

pub async fn run(job: Job) {
    match job.kind {
        JobKind::Step(hash)  => {
            let step = get_step(&hash).await;
            if let Some(step) = step {
                
                //sleep(Duration::from_secs(30)).await;
                // let out = step::execute(step).await;
                // db::log(serde_json::to_string(&out).unwrap().as_str()).await;
            }
        }

        JobKind::Flow(_) => {
            let flow = get_flow().await;
            if let Some(Flow{ steps }) = flow {
                for hash in steps {
                    let step = get_step(&hash).await;
                    step::execute(step.unwrap()).await;
                }
            }
        }
    }
}