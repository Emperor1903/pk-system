mod api;
mod db;
mod models;
mod config;
mod utils;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer, middleware};
use dotenv;
use models::*;

use crate::config::Config;

const PRIVATE_KEY: [u8; 32] = [4, 141, 82, 28, 211, 109, 76, 44, 193, 135, 179, 33, 9, 225, 7, 244, 110, 230, 70, 170, 153, 2, 152, 155, 154, 69, 126, 55, 39, 197, 237, 225];

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();
    let config = Config::from_env().unwrap();
    let host_url = format!("{}:{}", config.server.host, config.server.port);

    
    println!("Starting server at http://{}", host_url);

    HttpServer::new(|| {
        App::new()
            .wrap(
                IdentityService::new(
                    CookieIdentityPolicy::new(&PRIVATE_KEY)
                        .name("auth")
                        .secure(false),
                ))
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
        // Doctor
            .service(web::resource("api/doctor/_new").route(web::post().to(api::admin::create::<Doctor>)))
            .service(web::resource("api/doctor/_get").route(web::post().to(api::admin::get::<Doctor>)))
            .service(web::resource("api/doctor/_update").route(web::post().to(api::admin::update::<Doctor>)))
            .service(web::resource("api/doctor/_delete").route(web::post().to(api::admin::delete::<Doctor>)))
            .service(web::resource("api/doctor/_relate").route(web::post().to(api::admin::relate::<Doctor>)))
        // Hospital
            .service(web::resource("api/hospital/_new").route(web::post().to(api::admin::create::<Hospital>)))
            .service(web::resource("api/hospital/_update").route(web::post().to(api::admin::update::<Hospital>)))
            .service(web::resource("api/hospital/_delete").route(web::post().to(api::admin::delete::<Hospital>)))
        // Specialization
            .service(web::resource("api/specialization/_new").route(web::post().to(api::admin::create::<Specialization>)))
            .service(web::resource("api/specialization/_get").route(web::post().to(api::admin::get::<Specialization>)))
            .service(web::resource("api/specialization/_update").route(web::post().to(api::admin::update::<Specialization>)))
            .service(web::resource("api/specialization/_search").route(web::post().to(api::admin::search::<Specialization>)))
            .service(web::resource("api/specialization/_delete").route(web::post().to(api::admin::delete::<Specialization>)))
            .service(web::resource("api/specialization/_relate").route(web::post().to(api::admin::relate::<Specialization>)))
        // Clinic
            .service(web::resource("api/clinic/_new").route(web::post().to(api::admin::create::<Clinic>)))
            .service(web::resource("api/clinic/_get").route(web::post().to(api::admin::get::<Clinic>)))
            .service(web::resource("api/clinic/_update").route(web::post().to(api::admin::update::<Clinic>)))
            .service(web::resource("api/clinic/_search").route(web::post().to(api::admin::search::<Clinic>)))
            .service(web::resource("api/clinic/_delete").route(web::post().to(api::admin::delete::<Clinic>)))
            .service(web::resource("api/clinic/_relate").route(web::post().to(api::admin::relate::<Clinic>)))
        // Province
            .service(web::resource("api/province/_new").route(web::post().to(api::admin::create::<Province>)))
            .service(web::resource("api/province/_get").route(web::post().to(api::admin::get::<Province>)))
            .service(web::resource("api/province/_update").route(web::post().to(api::admin::update::<Province>)))
            .service(web::resource("api/province/_search").route(web::post().to(api::admin::search::<Province>)))
            .service(web::resource("api/province/_delete").route(web::post().to(api::admin::delete::<Province>)))
            .service(web::resource("api/province/_relate").route(web::post().to(api::admin::relate::<Province>)))
        // Shift
            .service(web::resource("api/shift/_new").route(web::post().to(api::admin::create::<Shift>)))
            .service(web::resource("api/shift/_get").route(web::post().to(api::admin::get::<Shift>)))
            .service(web::resource("api/shift/_update").route(web::post().to(api::admin::update::<Shift>)))
            .service(web::resource("api/shift/_search").route(web::post().to(api::admin::search::<Shift>)))
            .service(web::resource("api/shift/_delete").route(web::post().to(api::admin::delete::<Shift>)))
            .service(web::resource("api/shift/_relate").route(web::post().to(api::admin::relate::<Shift>)))
        // BookingTicket
            .service(web::resource("api/book/_new").route(web::post().to(api::admin::create::<BookingTicket>)))
            .service(web::resource("api/book/_get").route(web::post().to(api::admin::get::<BookingTicket>)))
            .service(web::resource("api/book/_update").route(web::post().to(api::admin::update::<BookingTicket>)))
            .service(web::resource("api/book/_search").route(web::post().to(api::admin::search::<BookingTicket>)))
            .service(web::resource("api/book/_delete").route(web::post().to(api::admin::delete::<BookingTicket>)))
            .service(web::resource("api/book/_relate").route(web::post().to(api::admin::relate::<BookingTicket>)))
        // Authentication
            .service(web::resource("api/auth/_new").route(web::post().to(api::auth::create_user)))
            .service(web::resource("api/auth/_login").route(web::post().to(api::auth::login)))
            .service(web::resource("api/auth/_logout").route(web::post().to(api::auth::logout)))
        // Public API
            .service(web::resource("api/hospital/_get").route(web::post().to(api::guest::get::<Hospital>)))                        
            .service(web::resource("api/doctor/_search").route(web::post().to(api::guest::search::<Doctor>)))            
            .service(web::resource("api/hospital/_relate").route(web::post().to(api::guest::relate::<Hospital>)))

            
    }).bind(host_url)?
        .run()
        .await
}
