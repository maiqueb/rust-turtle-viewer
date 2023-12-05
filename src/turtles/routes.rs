use crate::error_handler::AppError;
use crate::turtles::{NewTurtle, Turtle};
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, put, web, HttpResponse};

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

#[post("/turtles")]
async fn create(turtle: web::Json<NewTurtle>) -> Result<HttpResponse, AppError> {
    let employee = Turtle::create(turtle.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[put("/turtles/{id}")]
async fn update(
    id: web::Path<i32>,
    turtle: web::Json<NewTurtle>,
) -> Result<HttpResponse, AppError> {
    let employee = Turtle::update(id.into_inner(), turtle.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[delete("/turtles/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    Turtle::delete(id.into_inner())?;
    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
