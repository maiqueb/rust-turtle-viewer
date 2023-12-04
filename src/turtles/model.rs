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
}
