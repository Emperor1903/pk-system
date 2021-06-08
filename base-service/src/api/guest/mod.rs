use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use actix_web::{web, HttpResponse};
use mongodb::bson::oid::ObjectId;

use crate::models::*;
use crate::db;
use crate::app::*;
use crate::api::{do_response, form::*};


pub async fn search
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (query: web::Json<SearchQuery>) -> HttpResponse
{
    let data = query.into_inner();
    do_response(guest::search::<T>(data)).await
}

pub async fn get
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>    
    (item: web::Json<ObjectId>) -> HttpResponse
{
    let data = item.into_inner();
    do_response(guest::get::<T>(data)).await
}
