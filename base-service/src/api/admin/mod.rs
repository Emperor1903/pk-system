use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use actix_web::{web, HttpResponse};

use mongodb::bson::oid::ObjectId;
use actix_identity::Identity;

use crate::api::{do_response, form::{SearchQuery,RelateSearchQuery}};
use crate::app::admin;

pub async fn create
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    ( id: Identity, item: web::Json<T>) -> HttpResponse
{
    let data: T = item.into_inner();
    do_response(admin::create::<T>(&id, &data).unwrap()).await
}

pub async fn search
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (web::Query(query): web::Query<SearchQuery>, id: Identity) -> HttpResponse
{
    do_response(admin::search::<T>(&id, query).unwrap()).await
}

pub async fn update
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (item: web::Json<T>, id: Identity) -> HttpResponse
{
    let data: T = item.into_inner();
    do_response(admin::update::<T>(&id, &data).unwrap()).await
}

pub async fn delete
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (item: web::Json<ObjectId>, id: Identity) -> HttpResponse
{
    let data = item.into_inner();
    do_response(admin::delete::<T>(&id, data).unwrap()).await
}

pub async fn get
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>    
    (item: web::Json<ObjectId>, id: Identity) -> HttpResponse
{
    let data = item.into_inner();
    do_response(admin::get::<T>(&id, data).unwrap()).await
}

pub async fn relate
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (item: web::Json<RelateSearchQuery>, id: Identity) -> HttpResponse
{
    let query = item.into_inner();
    do_response(admin::relate::<T>(&id, query).unwrap()).await
}
