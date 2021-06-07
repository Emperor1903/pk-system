use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Specialization {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub desc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_keyword: Option<String>,    
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Doctor {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]    
    pub id: Option<ObjectId>,
    pub name: String,
    pub short_intro: String,
    pub intro: String,
    pub clinic: ObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]    
    pub specialization: Option<ObjectId>,
    pub position: String,
    pub email: String,
    pub phone_num: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_keyword: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Clinic {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,    
    pub name: String,
    pub desc: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hospital: Option<ObjectId>,
    pub time_desc: String,
    pub phone_num: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_keyword: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hospital {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]        
    pub id: Option<ObjectId>,    
    pub name: String,
    pub desc: String,
    pub address: String,
    pub province: ObjectId,
    pub phone_num: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_keyword: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Province {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_keyword: Option<String>,    
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub doctor: Option<ObjectId>,
    pub shift_num: i32,
    pub shift_day: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shift {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub doctor: Option<ObjectId>,
    pub client_number: i32,
    pub start_time: i32, // timestamp in second
    pub end_time: i32, // timestamp in second    
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookingTicket {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,    
    pub shift: ObjectId,
    pub client: ObjectId,
    pub desc_symptoms: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientInfo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]    
    pub id: Option<ObjectId>,
    pub name: String,
    pub date_of_birth: i64, // timestamp in second
    pub gender_is_male: bool,
    pub email_verified: bool,
    pub email: String,
    pub phone_num: String,
    pub address: String,    
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub username: String,
    pub password_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    #[serde(rename = "_id")]    
    pub username: String,
    pub role: i32,
    #[serde(skip_serializing_if = "Option::is_none")]        
    pub clinic: Option<ObjectId>
}

