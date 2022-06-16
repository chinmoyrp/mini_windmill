use anyhow::Result;
use bson::doc;
use serde::{Serialize, Deserialize};
use tokio::time::{sleep, Duration, Instant};
use mongodb::change_stream::event::OperationType;

use crate::db;
use crate::job::{self, Job};

#[derive(Debug, Serialize, Deserialize)]
pub struct Worker {
    id: u32,
    status: String,
    last_updated: String,
}

async fn update_worker_status(id: u32, status: String) {
    let coll = db::get_collection::<Worker>("workers").await;
    let filter = doc!{ "id" : id };
    let res = coll.find_one(filter.clone(), None).await.unwrap();
    let doc = Worker{ id, status, last_updated: db::get_current_datetime() };
    if res.is_none() {
        coll.insert_one(doc, None).await.unwrap();
    } else {
        coll.replace_one(filter, doc, None).await.unwrap();
    }
}

async fn watch_for_jobs(id: u32) {
    let coll = db::get_collection::<Job>("jobs").await;
    let mut watch = coll.watch(None, None).await.unwrap();
    let mut now = Instant::now();
    let eight_seconds = Duration::from_secs(8);
    
    sleep(Duration::from_millis(1)).await;

    while watch.is_alive() {
        if now.elapsed() >= eight_seconds {
            update_worker_status(id, String::from("Idle")).await;
            now = Instant::now();
        }

        if let Some(event) = watch.next_if_any().await.unwrap() {
            if event.operation_type != OperationType::Insert {
                continue;
            }

            let doc = event.full_document;
            if doc.is_none() {
                continue;
            }
            let doc = doc.unwrap();

            // To get the job..all threads would update the worker id and 
            // after a 10ms delay query it. Whoever updated last gets the 
            // job.
            let query = doc!{ "id": doc.id };
            let update = doc!{ "$set": {"worker_id": id}};
            coll.update_one(query.clone(), update, None).await.unwrap();

            sleep(Duration::from_millis(10)).await;

            let doc = coll.find_one(query, None).await.unwrap();

            if doc.is_none() {
                continue;
            }

            let job = doc.unwrap();
            
            if job.worker_id == id {
                update_worker_status(id, String::from("Busy")).await;
                job::run(job).await;
                update_worker_status(id, String::from("Idle")).await;
            }
        }
    }
}

pub async fn start_workers(num: u32) -> Result<()>{
    for n in 1..=num {
        println!("Starting worker {}...", n);
        tokio::spawn(async move {
            watch_for_jobs(n).await;
        });
    }

    Ok(())
}