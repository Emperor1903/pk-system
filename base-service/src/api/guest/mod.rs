use actix_web::{web, HttpResponse};

use crate::models::*;
use crate::db;
use crate::api::{do_response, form::SearchQuery};

pub async fn get_hospital_by_province
    (web::Query(mut query): web::Query<SearchQuery>) -> HttpResponse
{
    query.keyword = Some("province".to_string());
    do_response(db::search_relate::<Hospital>(query)).await
}
