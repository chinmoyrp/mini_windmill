use tokio::join;
use bson::doc;
use std::fs::File;
use std::process::{Command, Stdio};
use serde::{Serialize, Deserialize};
use std::io::{Write, BufRead, BufReader};

use crate::db;

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    pub hash: String,
    pub name: String,
    pub code: String,    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    job_id: String,
    step: Step,
    output: Vec<String>,
}

// pub async fn publish_step_result(job_id: String, step: Step, output: String) {
//     let coll = db::get_collection::<Output>(&job_id).await;
//     let res = coll.find_one(None, None).await.unwrap();
//     if res.is_none() {
//         let out = Output{ job_id, step, output:vec![output] };
//         coll.insert_one(out, None).await.unwrap();
//     } else {
//         let mut res = res.unwrap();
//         res.output.push(output);
//         coll.find_one_and_replace(doc!{ "job_id" : &job_id }, res, None).await.unwrap();
//     }
// }

// pub async fn publish_flow_result(job_id: String, code: String, output: String) {
//     let coll = db::get_collection::<Output>(&job_id).await;
//     let res = coll.find_one(doc!{ "job_id" : &job_id }, None).await.unwrap();
//     if res.is_none() {
//         let out = Output{ job_id, code, output:vec![output] };
//         coll.insert_one(out, None).await.unwrap();
//     } else {
//         let mut res = res.unwrap();
//         res.output.push(output);
//         coll.find_one_and_replace(doc!{ "job_id" : &job_id }, res, None).await.unwrap();
//     }
// }

pub async fn execute(step: Step) {
    let filepath = format!("/tmp/{}.go", step.hash);

    let mut file = File::create(&filepath).unwrap();
    file.write_all(step.code.as_bytes()).unwrap();

    let mut child = Command::new("/usr/local/go/bin/go")
                    .args(["run", &filepath])
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("go failed to execute!");

    
    let stderr = child.stderr.as_mut().unwrap();

    
    let reader_err = BufReader::new(stderr);

    join!(
        async {
            let stdout = child.stdout.as_mut().unwrap();
            let reader_out = BufReader::new(stdout);

        }
    );
}