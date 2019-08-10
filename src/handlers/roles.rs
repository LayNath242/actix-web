use actix_web::{HttpResponse, Result, web, HttpRequest};
use crate::models::role::{RoleList, NewRole, Role};
use crate::db_connection::PgPool;
use crate::handlers::pg_pool_handler;
// use crate::handlers::LoggedUser;




pub fn index(_user: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(RoleList::list(&pg_pool)))
}


pub fn create(_user: HttpRequest, new_role: web::Json<NewRole>, pool: web::Data<PgPool>) ->
 Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    new_role
        .create(&pg_pool)
        .map(|roles| HttpResponse::Ok().json(roles))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}

pub fn show(_user: HttpRequest, id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Role::find(&id, &pg_pool)
        .map(|roles| HttpResponse::Ok().json(roles))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}


pub fn update(_user: HttpRequest, id: web::Path<i32>,  new_role: web::Json<NewRole>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Role::update(&id, &new_role, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}

pub fn destroy(_user: HttpRequest, id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;
    Role::destroy(&id, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(e)
        })
}



