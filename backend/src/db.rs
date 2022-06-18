use mongodb::{
    Client, Collection, Database,
    bson::doc
};
use serde::{Serialize, Deserialize};
use chrono::Local;
use anyhow::Result;

const MONGODB_URI: &str = "mongodb://172.19.0.3:27017";
const DB_NAME: &str = "mini_windmill";

pub fn get_current_datetime() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub async fn get_client() -> Result<Client> {
    let client = Client::with_uri_str(MONGODB_URI).await?;
    Ok(client)
}

pub async fn get_db(name: &str) -> Result<Database> {
    let client = get_client().await?;
    let db = client.database(name);
    Ok(db)
}

pub async fn get_mini_wmill_db() -> Result<Database> {
    get_db(DB_NAME).await
}

pub async fn get_col() -> Result<String> {
    let mut v = vec![];
    for col in get_db("mini_windmill").await?.list_collection_names(None).await? {
        v.push(col);
    }

     Ok(v[0].clone())
}

pub async fn get_collection<'a, T: Deserialize<'a>>(collection: &str) -> Collection<T> {
    let db = get_mini_wmill_db().await.unwrap();
    db.collection::<T>(collection)    
}

#[derive(Debug, Serialize, Deserialize)]
struct Log {
    entries: Vec<String>
}

pub async fn log(msg: &str) {
    let datetime = get_current_datetime();
    let entry = format!("{}: {}", datetime, msg);

    let coll = get_collection::<Log>("logs").await;
    let log = coll.find_one(None, None).await.unwrap();

    if log.is_none() {
        let log = Log{ entries: vec![entry] };
        if let Err(e) = coll.insert_one(log, None).await {
            eprintln!("Error inserting log: {:?}", e);
        }
    } else {
        let mut log = log.unwrap();
        let query = doc! { "entries": &log.entries };
        log.entries.push(entry);
        let update = doc! { "$set": { "entries": &log.entries }};

        if let Err(res) = coll.update_one(query, update, None).await {
            eprintln!("Error updating log: {:?}", res);
        }
    }   
}