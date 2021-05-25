mod api;
mod db;
mod models;
mod config;
mod utils;
mod app;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer, middleware, cookie};
use actix_cors::Cors;
use dotenv;
use models::*;

use crate::config::Config;

const PRIVATE_KEY: [u8; 32] = [4, 141, 82, 28, 211, 109, 76, 44, 193, 135, 179, 33, 9, 225, 7, 244, 110, 230, 70, 170, 153, 2, 152, 155, 154, 69, 126, 55, 39, 197, 237, 225];

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    dotenv::from_filename(".env").ok();
    let config = Config::from_env().unwrap();
    let host_url = format!("{}:{}", config.server.host, config.server.port);

    
    println!("Starting server at http://{}", host_url);

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::permissive()
            )
            .wrap(
                IdentityService::new(
                    CookieIdentityPolicy::new(&PRIVATE_KEY)
                        .name("session-token")
                        .domain("localhost")
                        .secure(true)
                        .same_site(cookie::SameSite::None)
                        .http_only(true),
                ))
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
        // ***************Admin API
        // Authentication
            .service(web::resource("admin/_new").route(web::post().to(api::auth::create_admin_user)))
            .service(web::resource("admin/_new_staff").route(web::post().to(api::auth::create_staff_user)))                        
        // Doctor
            .service(web::resource("admin/doctor/_get").route(web::post().to(api::admin::get::<Doctor>)))
            .service(web::resource("admin/doctor/_relate").route(web::post().to(api::admin::relate::<Doctor>)))
            .service(web::resource("admin/doctor/_search").route(web::post().to(api::admin::search::<Doctor>)))            
            .service(web::resource("admin/doctor/_new").route(web::post().to(api::admin::create::<Doctor>)))
            .service(web::resource("admin/doctor/_update").route(web::post().to(api::admin::update::<Doctor>)))
            .service(web::resource("admin/doctor/_delete").route(web::post().to(api::admin::delete::<Doctor>)))
        // Hospital
            .service(web::resource("admin/hospital/_get").route(web::post().to(api::admin::get::<Hospital>)))
            .service(web::resource("admin/hospital/_relate").route(web::post().to(api::admin::relate::<Hospital>)))
            .service(web::resource("admin/hospital/_search").route(web::post().to(api::admin::search::<Hospital>)))            
            .service(web::resource("admin/hospital/_new").route(web::post().to(api::admin::create::<Hospital>)))
            .service(web::resource("admin/hospital/_update").route(web::post().to(api::admin::update::<Hospital>)))
            .service(web::resource("admin/hospital/_delete").route(web::post().to(api::admin::delete::<Hospital>)))
        // Specialization
            .service(web::resource("admin/specialization/_get").route(web::post().to(api::admin::get::<Specialization>)))
            .service(web::resource("admin/specialization/_relate").route(web::post().to(api::admin::relate::<Specialization>)))
            .service(web::resource("admin/specialization/_search").route(web::post().to(api::admin::search::<Specialization>)))
            .service(web::resource("admin/specialization/_new").route(web::post().to(api::admin::create::<Specialization>)))
            .service(web::resource("admin/specialization/_update").route(web::post().to(api::admin::update::<Specialization>)))
            .service(web::resource("admin/specialization/_delete").route(web::post().to(api::admin::delete::<Specialization>)))
        // Clinic
            .service(web::resource("admin/clinic/_get").route(web::post().to(api::admin::get::<Clinic>)))
            .service(web::resource("admin/clinic/_relate").route(web::post().to(api::admin::relate::<Clinic>)))
            .service(web::resource("admin/clinic/_search").route(web::post().to(api::admin::search::<Clinic>)))            
            .service(web::resource("admin/clinic/_new").route(web::post().to(api::admin::create::<Clinic>)))
            .service(web::resource("admin/clinic/_update").route(web::post().to(api::admin::update::<Clinic>)))
            .service(web::resource("admin/clinic/_delete").route(web::post().to(api::admin::delete::<Clinic>)))
        // Province
            .service(web::resource("admin/province/_get").route(web::post().to(api::admin::get::<Province>)))
            .service(web::resource("admin/province/_relate").route(web::post().to(api::admin::relate::<Province>)))
            .service(web::resource("admin/province/_search").route(web::post().to(api::admin::search::<Province>)))            
            .service(web::resource("admin/province/_new").route(web::post().to(api::admin::create::<Province>)))
            .service(web::resource("admin/province/_update").route(web::post().to(api::admin::update::<Province>)))
            .service(web::resource("admin/province/_delete").route(web::post().to(api::admin::delete::<Province>)))
        // Shift
            .service(web::resource("admin/shift/_get").route(web::post().to(api::admin::get::<Shift>)))
            .service(web::resource("admin/shift/_relate").route(web::post().to(api::admin::relate::<Shift>)))
            .service(web::resource("admin/shift/_search").route(web::post().to(api::admin::search::<Shift>)))            
            .service(web::resource("admin/shift/_new").route(web::post().to(api::admin::create::<Shift>)))
            .service(web::resource("admin/shift/_update").route(web::post().to(api::admin::update::<Shift>)))
            .service(web::resource("admin/shift/_delete").route(web::post().to(api::admin::delete::<Shift>)))
        // ***************Public API
        // AUthentcation
            .service(web::resource("api/auth/_me").route(web::post().to(api::auth::get_indentity)))            
            .service(web::resource("api/auth/_login").route(web::post().to(api::auth::login)))
            .service(web::resource("api/auth/_logout").route(web::post().to(api::auth::logout)))
        // Doctor
            .service(web::resource("api/doctor/_get").route(web::post().to(api::guest::get::<Doctor>)))
            .service(web::resource("api/doctor/_relate").route(web::post().to(api::guest::relate::<Doctor>)))
        // Specialization
            .service(web::resource("api/specialization/_get").route(web::post().to(api::guest::get::<Specialization>)))
            .service(web::resource("api/specialization/_search").route(web::post().to(api::guest::search::<Specialization>)))
            .service(web::resource("api/specialization/_relate").route(web::post().to(api::guest::relate::<Specialization>)))
        // Clinic
            .service(web::resource("api/clinic/_get").route(web::post().to(api::guest::get::<Clinic>)))
            .service(web::resource("api/clinic/_search").route(web::post().to(api::guest::search::<Clinic>)))
            .service(web::resource("api/clinic/_relate").route(web::post().to(api::guest::relate::<Clinic>)))            
        // Hospital
            .service(web::resource("api/hospital/_get").route(web::post().to(api::guest::get::<Hospital>)))                        
            .service(web::resource("api/hospital/_search").route(web::post().to(api::guest::search::<Hospital>)))            
            .service(web::resource("api/hospital/_relate").route(web::post().to(api::guest::relate::<Hospital>)))
        // Province
            .service(web::resource("api/province/_get").route(web::post().to(api::guest::get::<Province>)))            
            .service(web::resource("api/province/_search").route(web::post().to(api::guest::search::<Province>)))
            .service(web::resource("api/province/_relate").route(web::post().to(api::guest::relate::<Province>)))
        // Shift
            .service(web::resource("api/shift/_get").route(web::post().to(api::guest::get::<Shift>)))
            .service(web::resource("api/shift/_search").route(web::post().to(api::guest::search::<Shift>)))
            .service(web::resource("api/shift/_relate").route(web::post().to(api::guest::relate::<Shift>)))
        // BookingTicket
            .service(web::resource("api/book/_new").route(web::post().to(api::guest::new_booking_ticket)))            
            .service(web::resource("api/book/_get").route(web::post().to(api::guest::get::<BookingTicket>)))
            .service(web::resource("api/book/_search").route(web::post().to(api::guest::search::<BookingTicket>)))
            .service(web::resource("api/book/_relate").route(web::post().to(api::guest::relate::<BookingTicket>)))
            

            
    }).bind(host_url)?
        .run()
        .await
}
