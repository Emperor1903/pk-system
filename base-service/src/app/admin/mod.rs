use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use mongodb::bson::oid::ObjectId;
use actix_identity::Identity;

use crate::db;
use crate::models::*;
use crate::time_utils::*;
use crate::app::auth;
use crate::api::form::{SearchQuery, UpdateForm, UserForm};

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
    (id: &Identity, query: SearchQuery) -> Option<Result<mongodb::bson::Document, mongodb::error::Error>>
{
    if auth::check_role(id, 0) {
        Some(db::search::<T>(query))
    } else {
        None
    }
}

pub fn update
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (id: &Identity, data: &UpdateForm<T, ObjectId>) -> Option<Result<ObjectId, mongodb::error::Error>>    
{
    if auth::check_role(id, 0) {
        Some(db::update::<T, ObjectId>(data.id.clone(), &data.data))
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

pub fn create_shifts
    (id: &Identity) -> Result<Option<i32>, mongodb::error::Error>
{
    if auth::check_role(id, 0) {
        let shift_hour: [i32; 2] = [8, 13];
        let schedules = db::get_all::<Schedule>()?;
        for s in schedules {
            let timestamp = get_timestamp_now();
            let nw_timestamp = get_timestamp_next_week(timestamp);            
            let shift_day_timestamp = get_timestamp_dow(nw_timestamp, s.shift_day);
            let start_time = shift_day_timestamp + 3600 * shift_hour[s.shift_num as usize];
            let shift = Shift {
                id: None,
                doctor: s.doctor,
                client_number: 0,
                start_time: start_time,
                end_time: (start_time + 4 * 3600),
            };
            db::create::<Shift>(&shift)?;
        }
        Ok(Some(0))
    } else {
        Ok(None)
    }
}


pub fn create_admin_user
    (id: &Identity, form: &UserForm)
     -> Option<Result<bson::Bson, mongodb::error::Error>>
{
    if auth::check_role(id, 0) {
        auth::create_user(form, 0)
    } else {
        None
    }
}

pub fn create_staff_user
    (id: &Identity, form: &UserForm)
     -> Option<Result<bson::Bson, mongodb::error::Error>>
{
    if auth::check_role(id, 0) {
        auth::create_user(form, 1)
    } else {
        None
    }
}
