use crate::error_handler::AppError;
use crate::turtles::{NewTurtle, Turtle};
use actix_web::http::StatusCode;
use actix_web::{guard, web, HttpResponse};

async fn find_all() -> Result<HttpResponse, AppError> {
    let turtles = web::block(Turtle::find_all).await.unwrap();
    Ok(HttpResponse::Ok().json(turtles))
}

async fn find(id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    let turtle = Turtle::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(turtle))
}

async fn create(turtle: web::Json<NewTurtle>) -> Result<HttpResponse, AppError> {
    let employee = Turtle::create(turtle.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

async fn update(
    id: web::Path<i32>,
    turtle: web::Json<NewTurtle>,
) -> Result<HttpResponse, AppError> {
    let employee = Turtle::update(id.into_inner(), turtle.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

async fn delete(id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    Turtle::delete(id.into_inner())?;
    Ok(HttpResponse::new(StatusCode::NO_CONTENT))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/turtles")
            .route(
                web::route()
                    .guard(guard::Header("content-type", "application/json"))
                    .guard(guard::Get())
                    .to(find_all),
            )
            .route(
                web::route()
                    .guard(guard::Header("content-type", "application/json"))
                    .guard(guard::Post())
                    .to(create),
            ),
    );

    config.service(
        web::resource("/turtles/{id}")
            .route(
                web::route()
                    .guard(guard::Header("content-type", "application/json"))
                    .guard(guard::Get())
                    .to(find),
            )
            .route(
                web::route()
                    .guard(guard::Header("content-type", "application/json"))
                    .guard(guard::Put())
                    .to(update),
            )
            .route(
                web::route()
                    .guard(guard::Header("content-type", "application/json"))
                    .guard(guard::Delete())
                    .to(delete),
            ),
    );
}
