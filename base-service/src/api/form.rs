use serde::Deserialize;
use mongodb::bson::oid::ObjectId;

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    pub id: Option<ObjectId>,
    pub keyword: Option<String>,
    pub start: Option<u32>,
    pub limit: Option<i32>
}

#[derive(Deserialize)]
pub struct UserForm {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct HospitalByProvince {
    pub province: ObjectId,
}
