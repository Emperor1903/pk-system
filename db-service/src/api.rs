use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use actix_web::{web, http, HttpResponse};
use bson::oid::ObjectId;
use crate::db;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub keyword: Option<String>,
    pub start: Option<u32>,
    pub limit: Option<i32>
}

pub async fn do_response<T:'static + std::marker::Send + Serialize>
    (db_rs: Result<T, mongodb::error::Error>) -> HttpResponse
{
    let rs = web::block(move || db_rs).await;
    match rs {
        Ok(_result) => HttpResponse::Ok().json(_result),
        _ => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn create
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (item: web::Json<T>) -> HttpResponse
{
    let data: T = item.into_inner();
    do_response(db::create::<T>(&data)).await
}

pub async fn search
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (web::Query(query): web::Query<SearchQuery>) -> HttpResponse
{
    do_response(db::search::<T>(query)).await
}

pub async fn update
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (item: web::Json<T>) -> HttpResponse
{
    let data: T = item.into_inner();
    do_response(db::update::<T>(&data)).await
}

pub async fn delete
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (item: web::Json<ObjectId>) -> HttpResponse
{
    let data = item.into_inner();
    do_response(db::delete::<T>(&data)).await
}

pub async fn get
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>    
    (item: web::Json<ObjectId>) -> HttpResponse
{
    let data = item.into_inner();
    do_response(db::get::<T>(&data)).await
}
