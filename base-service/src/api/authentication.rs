use crate::db;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::models::User;
use super::do_response;
use sha2::{Sha256, Digest};

#[derive(Deserialize)]
pub struct UserCreateForm {
    username: String,
    password: String
}

#[derive(Deserialize)]
pub struct UserGetForm {
    username: String,
}

fn hash_password(password: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}


pub async fn create_user
    (form: web::Form<UserCreateForm>) -> HttpResponse
{
    let user = User {
        username: form.username.clone(),
        password_hash: hash_password(&form.password),
        is_admin: true
    };
    
    do_response(db::create::<User>(&user)).await
}


pub async fn get_user
    (item: web::Json<UserGetForm>) -> HttpResponse
{
    let data = item.into_inner();
    println!("{}", data.username)
    do_response(db::get::<User, String>(data.username)).await    
}
