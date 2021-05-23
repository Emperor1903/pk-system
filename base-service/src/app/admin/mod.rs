use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use mongodb::bson::oid::ObjectId;
use actix_identity::Identity;

use crate::db;
use crate::app::auth;
use crate::api::form::{SearchQuery,RelateSearchQuery};

pub fn create
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (id: &Identity, data: &T) -> Option<Result<mongodb::bson::Bson, mongodb::error::Error>>
{
    if auth::check_role(id, 0) {
        Some(db::create::<T>(data))
    } else {
        None
    }
}

pub fn search
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (id: &Identity, query: SearchQuery) -> Option<Result<Vec<T>, mongodb::error::Error>>
{
    if auth::check_role(id, 0) {
        Some(db::search::<T>(query))
    } else {
        None
    }
}

pub fn update
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (id: &Identity, data: &T) -> Option<Result<T, mongodb::error::Error>>    
{
    if auth::check_role(id, 0) {
        Some(db::update::<T>(data))
    } else {
        None
    }
}

pub fn delete
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (id: &Identity, oid: ObjectId) -> Option<Result<ObjectId, mongodb::error::Error>>
{
    if auth::check_role(id, 0) {
        Some(db::delete::<T, ObjectId>(oid))
    } else {
        None
    }
}

pub fn get
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>    
    (id: &Identity, oid: ObjectId) -> Option<Result<T, mongodb::error::Error>>
{
    if auth::check_role(id, 0) {
        Some(db::get::<T, ObjectId>(oid))
    } else {
        None
    }
}

pub fn relate
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (id: &Identity, query: RelateSearchQuery) -> Option<Result<Vec<mongodb::bson::Document>, mongodb::error::Error>>
{
    if auth::check_role(id, 0) {
        Some(db::search_relate::<T>(query))
    } else {
        None
    }
}
