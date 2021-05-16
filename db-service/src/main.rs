mod api;
mod db;
mod models;
mod config;
mod utils;

use actix_web::{web, App, HttpServer, middleware};
use dotenv;
use models::*;

use crate::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();
    let config = Config::from_env().unwrap();
    let host_url = format!("{}:{}", config.server.host, config.server.port);
    
    println!("Starting server at http://{}", host_url);
    
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
        // Doctor
            .service(web::resource("api/doctor/_new").route(web::post().to(api::create::<Doctor>)))
            .service(web::resource("api/doctor/_get").route(web::post().to(api::get::<Doctor>)))
            .service(web::resource("api/doctor/_update").route(web::post().to(api::update::<Doctor>)))
            .service(web::resource("api/doctor/_search").route(web::post().to(api::search::<Doctor>)))
            .service(web::resource("api/doctor/_delete").route(web::post().to(api::delete::<Doctor>)))
            .service(web::resource("api/doctor/_relate").route(web::post().to(api::relate::<Doctor>)))
        // Hospital
            .service(web::resource("api/hospital/_new").route(web::post().to(api::create::<Hospital>)))
            .service(web::resource("api/hospital/_get").route(web::post().to(api::get::<Hospital>)))
            .service(web::resource("api/hospital/_update").route(web::post().to(api::update::<Hospital>)))
            .service(web::resource("api/hospital/_search").route(web::post().to(api::search::<Hospital>)))
            .service(web::resource("api/hospital/_delete").route(web::post().to(api::delete::<Hospital>)))
            .service(web::resource("api/hospital/_relate").route(web::post().to(api::relate::<Hospital>)))
        // Specialization
            .service(web::resource("api/specialization/_new").route(web::post().to(api::create::<Specialization>)))
            .service(web::resource("api/specialization/_get").route(web::post().to(api::get::<Specialization>)))
            .service(web::resource("api/specialization/_update").route(web::post().to(api::update::<Specialization>)))
            .service(web::resource("api/specialization/_search").route(web::post().to(api::search::<Specialization>)))
            .service(web::resource("api/specialization/_delete").route(web::post().to(api::delete::<Specialization>)))
            .service(web::resource("api/specialization/_relate").route(web::post().to(api::relate::<Specialization>)))
        // Clinic
            .service(web::resource("api/clinic/_new").route(web::post().to(api::create::<Clinic>)))
            .service(web::resource("api/clinic/_get").route(web::post().to(api::get::<Clinic>)))
            .service(web::resource("api/clinic/_update").route(web::post().to(api::update::<Clinic>)))
            .service(web::resource("api/clinic/_search").route(web::post().to(api::search::<Clinic>)))
            .service(web::resource("api/clinic/_delete").route(web::post().to(api::delete::<Clinic>)))
            .service(web::resource("api/clinic/_relate").route(web::post().to(api::relate::<Clinic>)))
        // Province
            .service(web::resource("api/province/_new").route(web::post().to(api::create::<Province>)))
            .service(web::resource("api/province/_get").route(web::post().to(api::get::<Province>)))
            .service(web::resource("api/province/_update").route(web::post().to(api::update::<Province>)))
            .service(web::resource("api/province/_search").route(web::post().to(api::search::<Province>)))
            .service(web::resource("api/province/_delete").route(web::post().to(api::delete::<Province>)))
            .service(web::resource("api/province/_relate").route(web::post().to(api::relate::<Province>)))
        // Shift
            .service(web::resource("api/shift/_new").route(web::post().to(api::create::<Shift>)))
            .service(web::resource("api/shift/_get").route(web::post().to(api::get::<Shift>)))
            .service(web::resource("api/shift/_update").route(web::post().to(api::update::<Shift>)))
            .service(web::resource("api/shift/_search").route(web::post().to(api::search::<Shift>)))
            .service(web::resource("api/shift/_delete").route(web::post().to(api::delete::<Shift>)))
            .service(web::resource("api/shift/_relate").route(web::post().to(api::relate::<Shift>)))
        // Book
            .service(web::resource("api/book/_new").route(web::post().to(api::create::<Book>)))
            .service(web::resource("api/book/_get").route(web::post().to(api::get::<Book>)))
            .service(web::resource("api/book/_update").route(web::post().to(api::update::<Book>)))
            .service(web::resource("api/book/_search").route(web::post().to(api::search::<Book>)))
            .service(web::resource("api/book/_delete").route(web::post().to(api::delete::<Book>)))
            .service(web::resource("api/book/_relate").route(web::post().to(api::relate::<Book>)))            
            
            
    }).bind(host_url)?
        .run()
        .await
}
