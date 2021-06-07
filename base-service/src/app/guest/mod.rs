use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

use crate::models::*;
use crate::config::CONFIG;
use crate::db;

pub struct ClientForm {
    pub name: String,
    pub date_of_birth: i64,
    pub gender_is_male: bool,
    pub email: String,
    pub phone_num: String,
    pub address: String,
}
    
pub async fn create_client
    (client: &ClientForm)
    -> Result<mongodb::bson::Bson, mongodb::error::Error>
{
    let info = ClientInfo {
        id: None,
        name: client.name.clone(),
        email: client.email.clone(),
        phone_num: client.phone_num.clone(),
        address: client.address.clone(),
        date_of_birth: client.date_of_birth.clone(),
        gender_is_male: client.gender_is_male,
        email_verified: false,
    };

    db::create::<ClientInfo>(&info)
}

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
