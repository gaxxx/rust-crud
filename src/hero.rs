use serde::{Serialize,Deserialize};
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::schema::heroes;

#[table_name = "heroes"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

impl Hero {

}