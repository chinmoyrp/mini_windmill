use mongodb::{
    bson::{doc, Document},
    Client, Collection, Database
};
use anyhow::Result;

const DB_URI: &str = "mongodb://172.19.0.2:27017";

pub async fn get_client() -> Result<Client> {
    let client = Client::with_uri_str(DB_URI).await?;
    Ok(client)
}

pub async fn get_db(name: &str) -> Result<Database> {
    let client = get_client().await?;
    let db = client.database(name);
    Ok(db)
}

pub async fn get_col() -> Result<String> {
    let mut v = vec![];
    for col in get_db("mini_windmill").await?.list_collection_names(None).await? {
        v.push(col);
    }

     Ok(v[0].clone())
}
