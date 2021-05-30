use serde::Deserialize;
use mongodb::bson::oid::ObjectId;

#[derive(Deserialize, Debug)]
pub struct UserForm {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct HospitalByProvince {
    pub province: ObjectId,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub keyword: Option<String>,
    pub ids: Option<Vec<ObjectId>>,
    pub fields: Option<Vec<String>>,
    pub start: Option<u64>,
    pub limit: Option<i64>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>
}


#[derive(Deserialize)]
pub struct UpdateForm<T, U> {
    pub id: U,
    pub data: T
}
