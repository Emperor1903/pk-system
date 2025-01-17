use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use crate::api::form::{SearchQuery};
use mongodb::sync::{Client, Database};
use lazy_static::lazy_static;
use mongodb::bson::Bson;

use crate::config::CONFIG;

lazy_static! {
    pub static ref DB: Database = get_mongodb_db();
}

fn get_mongodb_db() -> Database {
    let client = Client::with_uri_str(&*CONFIG.mongodb.uri).expect("failed to init mongo client");    
    client.database("pk-db")
}

pub mod query;

pub fn create
    <T:'static + Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (data: &T) -> Result<Bson, mongodb::error::Error>
{
    query::create(data)
}

pub fn update
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone,
     U: Clone>
    (id: U, data: &T)
     -> Result<U, mongodb::error::Error> where Bson: From<U>
{
    query::update::<T, U>(id, data)
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

pub fn search
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (query: SearchQuery) -> Result<mongodb::bson::Document, mongodb::error::Error>
{
    query::search::<T>(query.keyword,
                       query.ids,
                       query.fields,
                       query.start,
                       query.limit,                       
                       query.start_time,
                       query.end_time)
}


pub fn get_client_info
    (email: &String) -> Result<mongodb::bson::Document, mongodb::error::Error>
{
    query::get_client_info(email.clone())
}
    
pub fn get_all
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    () -> Result<Vec<T>, mongodb::error::Error>
{
    query::get_all::<T>()
}

pub fn search_admin
    (query: SearchQuery) -> Result<mongodb::bson::Document, mongodb::error::Error>
{
    let _keyword = query.keyword;
    let _ids = query.ids;
    let _fields = query.fields;
    let _start = query.start;
    let _limit = query.limit;
    let _start_time = query.start_time;
    let _end_time = query.end_time;
    
    query::search_admin(_keyword, _start, _limit)
}

pub fn search_staff
    (query: SearchQuery) -> Result<mongodb::bson::Document, mongodb::error::Error>
{
    let _keyword = query.keyword;
    let _ids = query.ids;
    let _fields = query.fields;
    let _start = query.start;
    let _limit = query.limit;
    let _start_time = query.start_time;
    let _end_time = query.end_time;
    
    query::search_staff(_keyword, _ids, _start, _limit)
}
