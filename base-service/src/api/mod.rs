use serde::Serialize;
use actix_web::{web, http, HttpResponse};

pub async fn do_response<T:'static + std::marker::Send + Serialize>
    (db_rs: Result<T, mongodb::error::Error>) -> HttpResponse
{
    let rs = web::block(move || db_rs).await;
    match rs {
        Ok(_result) => HttpResponse::Ok().json(_result),
        _ => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub mod auth;
pub mod admin;
pub mod guest;
pub mod form;
