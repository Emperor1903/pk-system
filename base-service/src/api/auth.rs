use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use sha2::{Sha256, Digest};


use crate::db;
use crate::models::User;
use crate::api::do_auth_response;
use crate::api::form::UserForm;



fn hash_password(password: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}


pub async fn create_user
    (form: web::Form<UserForm>, id: Identity) -> HttpResponse
{

    let user = User {
        username: form.username.clone(),
        password_hash: hash_password(&form.password),
    };
            
    do_auth_response(db::create::<User>(&user), id).await
}


pub async fn login
    (form: web::Form<UserForm>, id: Identity) -> HttpResponse
{
    let data = form.into_inner();
    match db::get::<User, String>(data.username) {
        Ok(user) => {
            if hash_password(&data.password) == user.password_hash {
                id.remember(user.username);
                HttpResponse::Ok().body("succeed to login")
            } else {
                HttpResponse::Ok().body("failed to login")                
            }
        }
        Err(_) => {
            HttpResponse::Ok().body("failed to login")
        }
    }
}


pub async fn logout
    (id: Identity) -> HttpResponse
{
    id.forget();
    HttpResponse::Ok().body("Ok")
}
