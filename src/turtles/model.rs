use crate::db;
use crate::error_handler::AppError;
use crate::schema::ninja_turtles;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
#[diesel(table_name = ninja_turtles)]
pub struct Turtle {
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub weapon: String,
    #[diesel(sql_type = Timestamp)]
    pub created_on: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = ninja_turtles)]
pub struct NewTurtle {
    pub name: String,
    pub email: String,
    pub weapon: String,
}

impl Turtle {
    pub fn find_all() -> Result<Vec<Self>, AppError> {
        let mut conn = db::connection()?;
        let turtles = ninja_turtles::table.load::<Turtle>(&mut conn)?;
        Ok(turtles)
    }

    pub fn find(id: i32) -> Result<Self, AppError> {
        let mut conn = db::connection()?;
        let turtle = ninja_turtles::table
            .filter(ninja_turtles::user_id.eq(id))
            .first(&mut conn)?;
        Ok(turtle)
    }

    pub fn create(turtle: NewTurtle) -> Result<Self, AppError> {
        let mut conn = db::connection()?;
        let turtle = diesel::insert_into(ninja_turtles::table)
            .values(turtle)
            .get_result(&mut conn)?;
        Ok(turtle)
    }

    pub fn update(id: i32, turtle: NewTurtle) -> Result<Self, AppError> {
        let mut conn = db::connection()?;
        let employee = diesel::update(ninja_turtles::table)
            .filter(ninja_turtles::user_id.eq(id))
            .set(turtle)
            .get_result(&mut conn)?;
        Ok(employee)
    }

    pub fn delete(id: i32) -> Result<usize, AppError> {
        let mut conn = db::connection()?;
        let res = diesel::delete(ninja_turtles::table.filter(ninja_turtles::user_id.eq(id)))
            .execute(&mut conn)?;
        Ok(res)
    }
}
