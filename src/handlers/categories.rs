use actix_web::{HttpResponse, Result, web, HttpRequest};
use crate::handlers::pg_pool_handler;
use crate::db_connection::PgPool;
use crate::models::category::{Categorieslist, NewCategory, Category};


pub fn index(_user: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(Categorieslist::list(&pg_pool)))
}

pub fn create(_user: HttpRequest, new_category: web::Json<NewCategory>, pool: web::Data<PgPool>)->Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;

    new_category
        .create(&pg_pool)
        .map(|category| HttpResponse::Ok().json(category))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}


pub fn show(_user: HttpRequest, id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse>{
    let pg_pool = pg_pool_handler(pool)?;
    Category::find(&id, &pg_pool)
        .map(|category| HttpResponse::Ok().json(category))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })

}


pub fn destroy(_user: HttpRequest, id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Category::destroy(&id, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}


pub fn update(_user: HttpRequest, 
            id: web::Path<i32>, 
            new_category: web::Json<NewCategory>, 
            pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Category::update(&id, &new_category, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}