use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use actix_web::{web, HttpResponse};

use mongodb::bson::oid::ObjectId;
use actix_identity::Identity;

use crate::db;
use crate::api::{do_auth_response, form::SearchQuery};

pub async fn create
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (item: web::Json<T>, id: Identity) -> HttpResponse
{
    let data: T = item.into_inner();
    do_auth_response(db::create::<T>(&data), id).await
}

pub async fn search
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (web::Query(query): web::Query<SearchQuery>, id: Identity) -> HttpResponse
{
    do_auth_response(db::search::<T>(query), id).await
}

pub async fn update
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (item: web::Json<T>, id: Identity) -> HttpResponse
{
    let data: T = item.into_inner();
    do_auth_response(db::update::<T>(&data), id).await
}

pub async fn delete
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (item: web::Json<ObjectId>, id: Identity) -> HttpResponse
{
    let data = item.into_inner();
    do_auth_response(db::delete::<T, ObjectId>(data), id).await
}

pub async fn get
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>    
    (item: web::Json<ObjectId>, id: Identity) -> HttpResponse
{
    let data = item.into_inner();
    do_auth_response(db::get::<T, ObjectId>(data), id).await
}

pub async fn relate
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (item: web::Json<SearchQuery>, id: Identity) -> HttpResponse
{
    let query = item.into_inner();
    println!("{:?}", query);
    do_auth_response(db::search_relate::<T>(query), id).await
}

