use bson::doc;
use std::fs::File;
use std::process::{Command};
use serde::{Serialize, Deserialize};
use std::io::Write;

use crate::db;

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    pub hash: String,
    pub name: String,
    pub code: String,    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub output: Vec<String>,
}

pub async fn execute(job_id: &str, step: Step) {
    let filepath = format!("/tmp/{}.go", step.hash);

    let mut file = File::create(&filepath).unwrap();
    file.write_all(step.code.as_bytes()).unwrap();

    let output = Command::new("/usr/local/go/bin/go")
                    .args(["run", &filepath])
                    .output()
                    .expect("go failed to execute!");
    
    // if child.stderr.is_some() {
    // let stderr = child.stderr.as_mut().unwrap();
    // let reader_err = BufReader::new(stderr);
    // for err in reader_err.lines() {
    //     println!("{:?}....o]eut", err);
    //     let coll = db::get_collection::<Output>(job_id).await;
    //     let mut res = coll.find_one(None, None).await.unwrap();
    //     let out = res.as_mut().unwrap();
    //     out.output.push(err.unwrap());
    //     coll.find_one_and_replace(doc! { "output" : &out.output }, out, None).await.unwrap();
    // }}

    let coll = db::get_collection::<Output>(job_id).await;
    let mut res = coll.find_one(None, None).await.unwrap();
    let res = res.as_mut().unwrap();
    let filter = doc! { "output" : &res.output };
    let mut out_text = String::from_utf8_lossy(&output.stdout).to_string();
    out_text.push_str(&String::from_utf8_lossy(&output.stderr).to_string());
    res.output.push(out_text);
    coll.find_one_and_replace(filter, res, None).await.unwrap();
}