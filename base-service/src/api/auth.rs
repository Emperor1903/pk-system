use actix_web::{web, HttpResponse};
use actix_identity::Identity;

use crate::api::do_response;
use crate::api::form::UserForm;
use crate::app::auth;

pub async fn login
    (id: Identity, form: web::Form<UserForm>) -> HttpResponse
{
    let data = form.into_inner();
    println!("{:?}", data);
    match auth::login(&data, &id) {
        Some(_) => HttpResponse::Ok().body("succeed to login"),
        None => HttpResponse::BadRequest().body("failed to login")
    }
}

pub async fn logout
    (id: Identity) -> HttpResponse
{
    auth::logout(&id);
    HttpResponse::Ok().body("Ok")
}

pub async fn get_indentity
    (id: Identity) -> HttpResponse
{
    match auth::get_indentity(&id) {
        Some(user) => do_response(user).await,
        None => HttpResponse::Ok().body("")
    }
}
