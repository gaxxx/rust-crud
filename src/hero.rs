use serde::{Serialize,Deserialize};
use diesel;
use diesel::prelude::*;
use crate::schema::heroes;
use crate::db;
use std::ops::Deref;

#[table_name = "heroes"]
#[derive(Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

impl Hero {
    pub fn create(hero: Hero, connection: &db::Connection) -> Hero {
        diesel::insert_into(heroes::table)
            .values(&hero)
            .execute(connection.deref())
            .expect("Error creating new hero");

        heroes::table.order(heroes::id.desc()).first(connection.deref()).unwrap()
    }

    pub fn read(connection: &db::Connection) -> Vec<Hero> {
        heroes::table.order(heroes::id.asc()).load::<Hero>(connection.deref()).unwrap()
    }

    pub fn update(id: i32, hero: Hero, connection: &db::Connection) -> bool {
        let updated_row = diesel::update(heroes::table.find(id)).set(&hero).execute(connection.deref());
        updated_row.is_ok() && updated_row.unwrap() > 0
    }

    pub fn delete(id: i32, connection: &db::Connection) -> bool {
        diesel::delete(heroes::table.find(id)).execute(connection.deref()).is_ok()
    }
}
