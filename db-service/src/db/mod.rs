use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use crate::api::SearchQuery;
use mongodb::sync::{Client, Database};
use lazy_static::lazy_static;
use bson::oid::ObjectId;

use crate::utils::config::Config;


lazy_static! {
    pub static ref DB: Database = get_mongodb_db();
}

fn get_mongodb_db() -> Database {
    let config = Config::from_env().unwrap();
    let client = Client::with_uri_str(&*config.mongodb.uri).expect("failed to init mongo client");    
    client.database("pk-db")
}

pub mod query;

pub fn create
    <T:'static + Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (data: &T) -> Result<T, mongodb::error::Error>
{
    query::create(data)
}

pub fn search
    <T:'static + Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (query: SearchQuery) -> Result<Vec<T>, mongodb::error::Error>
{
    query::search(query.keyword, query.start, query.limit)
}

pub fn update
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (data: &T)
     -> Result<T, mongodb::error::Error>
{
    query::update(data)
}

pub fn delete
    <T:'static + Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (id: &ObjectId) -> Result<ObjectId, mongodb::error::Error>
{
    query::delete::<T>(id)
}

pub fn get
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (id: &ObjectId) ->
    Result<T, mongodb::error::Error>
{
    query::get::<T>(id)
}

pub fn search_relate
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (query: SearchQuery) -> Result<Vec<bson::Document>, mongodb::error::Error>
{
    let id = query.id.unwrap();
    let field = query.keyword.unwrap();
    query::search_relate::<T>(id, field, query.start, query.limit)
}
