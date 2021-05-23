use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Specialization {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,
    pub name: String,
    pub desc: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Doctor {
    #[serde(rename = "_id", skip_serializing)]    
    pub id: Option<ObjectId>,
    pub name: String,
    pub short_intro: String,
    pub intro: String,
    pub clinic: Option<ObjectId>,
    pub specializations: Vec<ObjectId>,
    pub positions: Vec<String>,
    pub experiences: Vec<String>,
    pub awards: Vec<String>,
    pub educations: Vec<String>,
    pub email: String,
    pub phone_num: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Clinic {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,    
    pub name: String,
    pub desc: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hospital: Option<ObjectId>,
    pub specializations: Vec<ObjectId>,
    pub time_desc: String,
    //pub doctors: Vec<ObjectId>,
    pub phone_num: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hospital {
    #[serde(rename = "_id", skip_serializing)]    
    pub id: Option<ObjectId>,    
    pub name: String,
    pub desc: String,
    pub address: String,
    pub province: ObjectId,
    //pub clinics: Vec<ObjectId>,
    //pub specializations: Vec<ObjectId>,
    //pub doctors: Vec<ObjectId>,
    pub phone_num: String,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Province {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shift {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,
    pub doctor: Option<ObjectId>,
    pub clinic: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hospital: Option<ObjectId>,
    pub client_number: u32,
    pub start_time: u64, // timestamp in second
    pub end_time: u64, // timestamp in second    
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookingTicket {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,    
    pub shift: ObjectId,
    pub desc_symptoms: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ledger {
    #[serde(rename = "_id", skip_serializing)]    
    pub id: Option<ObjectId>,
    pub name: String,
    pub date_of_birth: u64, // timestamp in second
    pub contact: ObjectId,
    pub gender_is_male: bool,
    #[serde(skip_serializing)]
    pub email_verified: Option<bool>,    
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing)]
    pub username: String,
    pub password_hash: String,
    pub role: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contact {
    #[serde(rename = "_id")]
    pub email: String,
    pub phone_num: String,
    pub address: String
}
