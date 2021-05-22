use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use crate::api::form::{SearchQuery, RelateSearchQuery};
use mongodb::sync::{Client, Database};
use lazy_static::lazy_static;
use mongodb::bson::{oid::ObjectId, Bson};

use crate::config::Config;

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
    (data: &T) -> Result<Bson, mongodb::error::Error>
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
    <T:'static + Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone,
     U: Clone>
    (id: U) -> Result<U, mongodb::error::Error> where Bson: From<U>
{
    query::delete::<T, U>(id)
}

pub fn get
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone,
     U: Clone> 
    (id: U) ->
    Result<T, mongodb::error::Error> where Bson: From<U>
{
    query::get::<T, U>(id)
}

pub fn search_relate
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (query: RelateSearchQuery) -> Result<Vec<mongodb::bson::Document>, mongodb::error::Error>
{
    let ids = query.ids.unwrap();
    let fields = query.fields.unwrap();
    query::search_relate::<T>(ids, fields, query.start, query.limit, query.start_time, query.end_time)
}
