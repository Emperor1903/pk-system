use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use mongodb::bson::oid::ObjectId;

use crate::api::{do_response, form::*};
use crate::app::admin;
use crate::api::form::UserForm;

pub async fn create
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    ( id: Identity, item: web::Json<T>) -> HttpResponse
{
    let data: T = item.into_inner();
    do_response(admin::create::<T>(&id, &data).unwrap()).await
}

pub async fn search
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (id: Identity, query: web::Json<SearchQuery>) -> HttpResponse
{
    let data = query.into_inner();
    do_response(admin::search::<T>(&id, data).unwrap()).await
}

pub async fn update
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (item: web::Json<UpdateForm<T, ObjectId>>, id: Identity) -> HttpResponse
{
    let data: UpdateForm<T, ObjectId> = item.into_inner();
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

pub async fn create_shifts
    (id: Identity) -> HttpResponse
{
    do_response(admin::create_shifts(&id)).await
}

pub async fn create_admin_user
    (id: Identity, user: web::Json<UserForm>) -> HttpResponse
{
    do_response(admin::create_admin_user(&id, &user).unwrap()).await
}

pub async fn create_staff_user
    (id: Identity, user: web::Json<UserForm>) -> HttpResponse
{
    let t = admin::create_staff_user(&id, &user).unwrap();
    do_response(t).await
}

pub async fn search_admin
    (id: Identity, query: web::Json<SearchQuery>) -> HttpResponse
{
    let data = query.into_inner();
    do_response(admin::search_admin(&id, data).unwrap()).await
}

pub async fn search_staff
    (id: Identity, query: web::Json<SearchQuery>) -> HttpResponse
{
    let data = query.into_inner();
    do_response(admin::search_staff(&id, data).unwrap()).await
}

pub async fn delete_user
    (id: Identity, query: web::Json<String>) -> HttpResponse
{
    let data = query.into_inner();
    do_response(admin::delete_user(&id, data).unwrap()).await    
}
