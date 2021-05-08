mod api;
mod db;
mod utils;
mod models;

use actix_web::{web, App, HttpServer, middleware};
use dotenv;
use models::*;

use crate::utils::{config::Config};

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
        // Specialization
            .service(web::resource("api/specialization/_new").route(web::post().to(api::create::<Specialization>)))
            .service(web::resource("api/specialization/_get").route(web::post().to(api::get::<Specialization>)))
            .service(web::resource("api/specialization/_update").route(web::post().to(api::update::<Specialization>)))
            .service(web::resource("api/specialization/_search").route(web::post().to(api::search::<Specialization>)))
            .service(web::resource("api/specialization/_delete").route(web::post().to(api::delete::<Specialization>)))
        // Clinic
            .service(web::resource("api/clinic/_new").route(web::post().to(api::create::<Clinic>)))
            .service(web::resource("api/clinic/_get").route(web::post().to(api::get::<Clinic>)))
            .service(web::resource("api/clinic/_update").route(web::post().to(api::update::<Clinic>)))
            .service(web::resource("api/clinic/_search").route(web::post().to(api::search::<Clinic>)))
            .service(web::resource("api/clinic/_delete").route(web::post().to(api::delete::<Clinic>)))
        // Province
            .service(web::resource("api/province/_new").route(web::post().to(api::create::<Province>)))
            .service(web::resource("api/province/_get").route(web::post().to(api::get::<Province>)))
            .service(web::resource("api/province/_update").route(web::post().to(api::update::<Province>)))
            .service(web::resource("api/province/_search").route(web::post().to(api::search::<Province>)))
            .service(web::resource("api/province/_delete").route(web::post().to(api::delete::<Province>)))
            
            
    }).bind(host_url)?
        .run()
        .await
}
