use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use mongodb::bson::oid::ObjectId;
use actix_identity::Identity;

use crate::db;
use crate::models::*;
use crate::time_utils::*;
use crate::app::auth;
use crate::api::form::*;


pub fn search
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (id: &Identity, query: SearchQuery)
     -> Option<Result<mongodb::bson::Document, mongodb::error::Error>>
{
    if auth::check_role(id, 1) {
        Some(db::search::<T>(query))
    } else {
        None
    }
}

pub fn update_doctor
    (id: &Identity, data: &UpdateForm<Doctor, ObjectId>)
     -> Option<Result<ObjectId, mongodb::error::Error>>
{
    if auth::check_staff_authority(id, data.data.clinic.clone()) {
        Some(db::update::<Doctor, ObjectId>(data.id.clone(), &data.data))
    } else {
        None
    }
}

pub fn update_clinic
    (id: &Identity, data: &UpdateForm<Clinic, ObjectId>)
     -> Option<Result<ObjectId, mongodb::error::Error>>
{
    let oid = data.data.id.clone().unwrap();
    if auth::check_staff_authority(id, oid) {
        Some(db::update::<Clinic, ObjectId>(data.id.clone(), &data.data))
    } else {
        None
    }
}

pub fn create_doctor
    (id: &Identity, data: &Doctor) -> Option<Result<bson::Bson, mongodb::error::Error>>
{
    if auth::check_staff_authority(id, data.clinic.clone()) {
        Some(db::create::<Doctor>(data))
    }
    else {
        None
    }
}

pub fn create_schedule
    (id: &Identity, data: &Schedule)
     -> Option<Result<mongodb::bson::Bson, mongodb::error::Error>>
{
    let doctor = get_doctor(id, data.doctor.clone()).unwrap();
    match doctor {
        Some(_) => {
            Some(db::create::<Schedule>(data))
        }
        None => None
    }
}

fn get_doctor
    (id: &Identity, oid: ObjectId)
    -> Result<Option<Doctor>, mongodb::error::Error>
{
    let doctor = db::get::<Doctor, ObjectId>(oid)?;
    if auth::check_staff_authority(id, doctor.clinic.clone()) {
        Ok(Some(doctor))
    } else {
        Ok(None)
    }
}

pub fn get
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (id: &Identity, oid: ObjectId) -> Option<Result<T, mongodb::error::Error>>
{
    if auth::check_role(id, 1) {
        Some(db::get::<T, ObjectId>(oid))
    } else {
        None
    }
}

pub fn delete_doctor
    (id: &Identity, oid: ObjectId)
     -> Option<Result<ObjectId, mongodb::error::Error>>
{
    let doctor = get_doctor(id, oid.clone()).unwrap();
    match doctor {
        Some(_) => Some(db::delete::<Doctor, ObjectId>(oid)),
        None => None
    }
}

pub fn delete_schedule
    (id: &Identity, oid: ObjectId)
     -> Option<Result<ObjectId, mongodb::error::Error>>
{
    let schedule = db::get::<Schedule, ObjectId>(oid.clone()).unwrap();
    let doctor = get_doctor(id, schedule.doctor.clone()).unwrap();
    match doctor {
        Some(_) => Some(db::delete::<Doctor, ObjectId>(oid.clone())),
        None => None
    }
}

pub fn delete_shift
    (id: &Identity, oid: ObjectId)
     -> Option<Result<ObjectId, mongodb::error::Error>>
{
    let shift = db::get::<Shift, ObjectId>(oid.clone()).unwrap();
    let doctor = get_doctor(id, shift.doctor.clone()).unwrap();
    match doctor {
        Some(_) => Some(db::delete::<Doctor, ObjectId>(oid.clone())),
        None => None
    }
}

pub fn create_shifts
    (id: &Identity) -> Result<Option<i32>, mongodb::error::Error>
{
    if auth::check_role(id, 1) {
        let user = auth::get_identity(id).unwrap()?;
        let shift_hour: [i32; 2] = [8, 13];
        let schedules = db::get_all::<Schedule>()?;
        for s in schedules {
            if s.clinic == user.clinic.clone().unwrap() {
                let timestamp = get_timestamp_now();
                let nw_timestamp = get_timestamp_next_week(timestamp);            
                let shift_day_timestamp = get_timestamp_dow(nw_timestamp, s.shift_day);
                let start_time = shift_day_timestamp + 3600 * shift_hour[s.shift_num as usize];
                let shift = Shift {
                    id: None,
                    doctor: s.doctor,
                    clinic: s.clinic,
                    client_number: 0,
                    start_time: start_time,
                    end_time: (start_time + 4 * 3600),
                };
                db::create::<Shift>(&shift)?;
            }
        }
        Ok(Some(0))
    } else {
        Ok(None)
    }
}
    
