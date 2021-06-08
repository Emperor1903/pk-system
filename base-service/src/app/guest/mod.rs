use std::fmt::Debug;
use serde::Serialize;
use serde::de::DeserializeOwned;
use mongodb::bson::oid::ObjectId;

use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

use crate::models::*;
use crate::config::CONFIG;
use crate::api::form::*;
use crate::db;

pub fn search
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug + Sync + std::marker::Send + Clone>
    (query: SearchQuery) -> Result<mongodb::bson::Document, mongodb::error::Error>
{
    db::search::<T>(query)
}

pub fn get
    <T:'static +  Serialize + DeserializeOwned + Unpin + Debug+ Sync + std::marker::Send + Clone>
    (oid: ObjectId) -> Result<T, mongodb::error::Error>
{
    db::get::<T, ObjectId>(oid)
}


// pub async fn create_client
//     (client: &ClientForm)
//     -> Result<mongodb::bson::Bson, mongodb::error::Error>
// {
//     let info = ClientInfo {
//         id: None,
//         name: client.name.clone(),
//         email: client.email.clone(),
//         phone_num: client.phone_num.clone(),
//         address: client.address.clone(),
//         date_of_birth: client.date_of_birth.clone(),
//         gender_is_male: client.gender_is_male,
//         email_verified: false,
//     };

//     db::create::<ClientInfo>(&info)
// }

// pub async fn send_verify_mail
//     (email_address: String)
//     -> Result<mongodb::bson::Bson, mongodb::error::Error>
// {
//     let info = db::get_client_info(email_address)?;

//     let smtp_address = CONFIG.email.server.clone();
//     let username = CONFIG.email.username.clone();
//     let password = CONFIG.email.password.clone();

// //    let verify_code = 

//     let email = EmailBuilder::new()
//         .to(email_address)
//         .from(username)
//         .subject("Lay lay lai so ghi")
//         .text("")
//         .build()
//         .unwrap()
//         .into();

//     let credentials = (username, password).into_credentials();
//     let mut client = SmtpClient::new_simple(smtp_address)
//         .unwrap()
//         .credentials(credentials)
//         .transport();        

//     let _result = client.send(email).unwrap();
// }
