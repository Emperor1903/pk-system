use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use mongodb::bson::oid::ObjectId;

use crate::models::*;
use crate::api::{do_response, form::*};
use crate::app::staff;
use crate::api::form::UserForm;

pub async fn create_doctor
    (id: Identity, item: web::Json<Doctor>) -> HttpResponse
{
    let data = item.into_inner();
    do_response(staff::create_doctor(&id, &data).unwrap()).await
}

pub async fn create_schedule
    ( id: Identity, item: web::Json<Schedule>) -> HttpResponse
{
    let data = item.into_inner();
    do_response(staff::create_schedule(&id, &data).unwrap()).await
}

pub async fn update_clinic
    (id: Identity, item: web::Json<UpdateForm<Clinic, ObjectId>>) -> HttpResponse
{
    let data = item.into_inner();
    do_response(staff::update_clinic(&id, &data).unwrap()).await
}

pub async fn update_doctor
    (id: Identity, item: web::Json<UpdateForm<Doctor, ObjectId>>) -> HttpResponse
{
    let data = item.into_inner();
    do_response(staff::update_doctor(&id, &data).unwrap()).await
}

pub async fn delete_doctor
    (item: web::Json<ObjectId>, id: Identity) -> HttpResponse
{
    let data = item.into_inner();
    do_response(staff::delete_doctor(&id, data).unwrap()).await
}

pub async fn delete_shift
    (item: web::Json<ObjectId>, id: Identity) -> HttpResponse
{
    let data = item.into_inner();
    do_response(staff::delete_shift(&id, data).unwrap()).await
}

pub async fn delete_schedule
    (item: web::Json<ObjectId>, id: Identity) -> HttpResponse
{
    let data = item.into_inner();
    do_response(staff::delete_schedule(&id, data).unwrap()).await
}

pub async fn search
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (id: Identity, query: web::Json<SearchQuery>) -> HttpResponse
{
    let data = query.into_inner();
    do_response(staff::search::<T>(&id, data).unwrap()).await
}

pub async fn get
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>    
    (item: web::Json<ObjectId>, id: Identity) -> HttpResponse
{
    let data = item.into_inner();
    do_response(staff::get::<T>(&id, data).unwrap()).await
}

pub async fn create_shifts
    (id: Identity) -> HttpResponse
{
    do_response(staff::create_shifts(&id)).await
}
