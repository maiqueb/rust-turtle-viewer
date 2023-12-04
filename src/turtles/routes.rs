use crate::error_handler::AppError;
use crate::turtles::Turtle;
use actix_web::{get, web, HttpResponse};

#[get("/turtles")]
async fn find_all() -> Result<HttpResponse, AppError> {
    let turtles = web::block(Turtle::find_all).await.unwrap();
    Ok(HttpResponse::Ok().json(turtles))
}

#[get("/turtles/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    let turtle = Turtle::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(turtle))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
}
