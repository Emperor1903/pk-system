use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Specialization {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub desc: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Doctor {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub short_intro: String,
    pub intro: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinic: Option<ObjectId>,
    pub specializations: Vec<String>,
    pub positions: Vec<String>,
    pub experiences: Vec<String>,
    pub awards: Vec<String>,
    pub educations: Vec<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Clinic {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,    
    pub name: String,
    pub desc: String,
    pub hospital: String,
    pub specializations: Vec<ObjectId>,
    pub doctor: Vec<ObjectId>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hospital {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,    
    pub name: String,
    pub desc: String,
    pub location: String,
    pub province: ObjectId,
    pub clinic: Vec<ObjectId>,
    pub specializations: Vec<ObjectId>,
    pub doctor: Vec<ObjectId>,
    pub telephone: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Province {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shift {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub doctor: Option<ObjectId>,
    pub hospital: Option<ObjectId>,
    pub start_time: u32,
    pub duration: u32
}
