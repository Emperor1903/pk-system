use serde::Deserialize;
use mongodb::bson::oid::ObjectId;

#[derive(Deserialize, Debug, Clone)]
pub struct UserForm {
    pub username: String,
    pub password: String,
    pub clinic: Option<ObjectId>,
}

#[derive(Deserialize)]
pub struct HospitalByProvince {
    pub province: ObjectId,
}

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    pub keyword: Option<String>,
    pub ids: Option<Vec<ObjectId>>,
    pub fields: Option<Vec<String>>,
    pub start: Option<i64>,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateForm<T, U> {
    pub id: U,
    pub data: T,
}

pub struct StaffSearchQuery {
    pub keyword: Option<String>,
    pub clinic: ObjectId,
    pub start: Option<i64>,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,    
}
