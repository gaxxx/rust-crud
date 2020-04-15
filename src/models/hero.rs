use serde::{Serialize,Deserialize};
use diesel;
use crate::schema::heroes;

#[table_name = "heroes"]
#[derive(Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

