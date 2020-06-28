use serde::{Serialize,Deserialize};
use crate::schema::users;


#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
pub struct Hero {
    pub id: i32,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

#[table_name = "users"]
#[derive(Clone, Debug, Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
pub struct HeroInput {
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}
