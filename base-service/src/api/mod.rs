use serde::Serialize;
use actix_web::{web, http, HttpResponse};
use actix_identity::Identity;
use crate::db;

pub async fn do_response<T:'static + std::marker::Send + Serialize>
    (db_rs: Result<T, mongodb::error::Error>) -> HttpResponse
{
    let rs = web::block(move || db_rs).await;
    match rs {
        Ok(_result) => HttpResponse::Ok().json(_result),
        _ => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn do_auth_response<T:'static + std::marker::Send + Serialize>
    (db_rs: Result<T, mongodb::error::Error>, id: Identity) -> HttpResponse
{
    match id.identity() {
        Some(_) => {
            do_response(db_rs).await
        }
        None => HttpResponse::NonAuthoritativeInformation().body("Ok")
    }    
}

pub mod auth;
pub mod admin;
pub mod form;
