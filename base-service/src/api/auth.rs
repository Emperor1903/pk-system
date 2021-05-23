use actix_web::{web, HttpResponse};
use actix_identity::Identity;

use crate::api::do_response;
use crate::api::form::UserForm;
use crate::app::auth;

pub async fn
    create_admin_user(id: Identity, user: web::Form<UserForm>) -> HttpResponse
{
    do_response(auth::create_admin_user(&id, &user).unwrap()).await
}

pub async fn
    create_staff_user(id: Identity, user: web::Form<UserForm>) -> HttpResponse
{
    do_response(auth::create_staff_user(&id, &user).unwrap()).await
}

pub async fn login
    (id: Identity, form: web::Form<UserForm>) -> HttpResponse
{
    let data = form.into_inner();
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
