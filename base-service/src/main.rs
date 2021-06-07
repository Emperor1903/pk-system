mod api;
mod db;
mod models;
mod config;
mod utils;
mod app;
mod time_utils;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer, middleware, cookie};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use actix_cors::Cors;
use dotenv;
use models::*;

use crate::config::CONFIG;

const PRIVATE_KEY: [u8; 32] = [4, 141, 82, 28, 211, 109, 76, 44, 193, 135, 179, 33, 9, 225, 7, 244, 110, 230, 70, 170, 153, 2, 152, 155, 154, 69, 126, 55, 39, 197, 237, 225];

fn load_ssl() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    builder
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
        
    dotenv::from_filename(".env").ok();
    
    let host_url = format!("{}:{}", CONFIG.server.host, CONFIG.server.port);
    
    println!("Starting server at https://{}", host_url);

    let domain: String = CONFIG.server.domain.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
            )
            .wrap(
                IdentityService::new(
                    CookieIdentityPolicy::new(&PRIVATE_KEY)
                        .name("session-token")
                        .domain(domain.as_str())
                        .same_site(cookie::SameSite::None)
                        .http_only(false)
                        .secure(true),
                ))
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
        // ***************Admin API
        // Authentication
            .service(web::resource("admin/admin/_new").route(web::post().to(api::admin::create_admin_user)))
            .service(web::resource("admin/staff/_new").route(web::post().to(api::admin::create_staff_user)))
        // User
            .service(web::resource("admin/user/_search").route(web::post().to(api::admin::search::<User>)))
        // Doctor
            .service(web::resource("admin/doctor/_get").route(web::post().to(api::admin::get::<Doctor>)))
            .service(web::resource("admin/doctor/_search").route(web::post().to(api::admin::search::<Doctor>)))
            .service(web::resource("admin/doctor/_new").route(web::post().to(api::admin::create::<Doctor>)))
            .service(web::resource("admin/doctor/_update").route(web::post().to(api::admin::update::<Doctor>)))
            .service(web::resource("admin/doctor/_delete").route(web::delete().to(api::admin::delete::<Doctor>)))
        // Hospital
            .service(web::resource("admin/hospital/_get").route(web::post().to(api::admin::get::<Hospital>)))
            .service(web::resource("admin/hospital/_search").route(web::post().to(api::admin::search::<Hospital>)))            
            .service(web::resource("admin/hospital/_new").route(web::post().to(api::admin::create::<Hospital>)))
            .service(web::resource("admin/hospital/_update").route(web::post().to(api::admin::update::<Hospital>)))
            .service(web::resource("admin/hospital/_delete").route(web::delete().to(api::admin::delete::<Hospital>)))
        // Specialization
            .service(web::resource("admin/specialization/_get").route(web::post().to(api::admin::get::<Specialization>)))
            .service(web::resource("admin/specialization/_search").route(web::post().to(api::admin::search::<Specialization>)))
            .service(web::resource("admin/specialization/_new").route(web::post().to(api::admin::create::<Specialization>)))
            .service(web::resource("admin/specialization/_update").route(web::post().to(api::admin::update::<Specialization>)))
            .service(web::resource("admin/specialization/_delete").route(web::delete().to(api::admin::delete::<Specialization>)))
        // Clinic
            .service(web::resource("admin/clinic/_get").route(web::post().to(api::admin::get::<Clinic>)))
            .service(web::resource("admin/clinic/_search").route(web::post().to(api::admin::search::<Clinic>)))            
            .service(web::resource("admin/clinic/_new").route(web::post().to(api::admin::create::<Clinic>)))
            .service(web::resource("admin/clinic/_update").route(web::post().to(api::admin::update::<Clinic>)))
            .service(web::resource("admin/clinic/_delete").route(web::delete().to(api::admin::delete::<Clinic>)))
        // Province
            .service(web::resource("admin/province/_get").route(web::post().to(api::admin::get::<Province>)))
            .service(web::resource("admin/province/_search").route(web::post().to(api::admin::search::<Province>)))            
            .service(web::resource("admin/province/_new").route(web::post().to(api::admin::create::<Province>)))
            .service(web::resource("admin/province/_update").route(web::post().to(api::admin::update::<Province>)))
            .service(web::resource("admin/province/_delete").route(web::delete().to(api::admin::delete::<Province>)))
        // Shift
            .service(web::resource("admin/shift/_get").route(web::post().to(api::admin::get::<Shift>)))
            .service(web::resource("admin/shift/_search").route(web::post().to(api::admin::search::<Shift>)))            
            .service(web::resource("admin/shift/_new").route(web::post().to(api::admin::create::<Shift>)))
            .service(web::resource("admin/shift/_update").route(web::post().to(api::admin::update::<Shift>)))
            .service(web::resource("admin/shift/_delete").route(web::delete().to(api::admin::delete::<Shift>)))
            .service(web::resource("admin/shift/_new_week").route(web::post().to(api::admin::create_shifts)))
        // Schedule
            .service(web::resource("admin/schedule/_get").route(web::post().to(api::admin::get::<Schedule>)))
            .service(web::resource("admin/schedule/_search").route(web::post().to(api::admin::search::<Schedule>)))            
            .service(web::resource("admin/schedule/_new").route(web::post().to(api::admin::create::<Schedule>)))
            .service(web::resource("admin/schedule/_update").route(web::post().to(api::admin::update::<Schedule>)))
            .service(web::resource("admin/schedule/_delete").route(web::delete().to(api::admin::delete::<Schedule>)))
        // ***************Public API
        // AUthentcation
            .service(web::resource("api/auth/_me").route(web::post().to(api::auth::get_indentity)))
            .service(web::resource("api/auth/_login").route(web::post().to(api::auth::login)))
            .service(web::resource("api/auth/_logout").route(web::post().to(api::auth::logout)))
    }).bind_openssl(host_url, load_ssl())?
        .run()
        .await
}
