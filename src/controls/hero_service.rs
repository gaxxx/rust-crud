use actix_web::web::Path;
use actix_web::{web, App, Responder, HttpServer, get, post, put, delete, Either, Scope};
use serde_json::json;
use crate::db;
use crate::schema::heroes;
use crate::models::hero::Hero;
use actix_web::dev::HttpServiceFactory;
use std::ops::Deref;
use diesel::prelude::*;


#[get("/")]
async fn read() -> impl Responder {
    web::Json(vec!["hero", "hero"])
}

#[put("/{id}")]
async fn update(path: Path<i32>, hero: web::Json<Hero>, conn: db::Connection) -> impl Responder {
    let hero = hero.into_inner();
    if HeroSevice::update(path.into_inner(), hero.clone(), &conn) {
        Either::A(web::Json(hero))
    } else {
        Either::B(web::Json(json!({"status": "error"})))
    }
}

#[delete("/{id}")]
async fn delete(path : Path<i32>, db: db::DB) -> impl Responder{
    HeroSevice::delete(path.into_inner(), &db.get());
    web::Json(json!({"status": "ok"}))
}

#[post("/")]
async fn create(hero: web::Json<Hero>, db: db::DB) -> impl Responder {
    let hero = HeroSevice::create(hero.into_inner(), &db.get());
    web::Json(hero)
}


#[get("/{user}/{name}")]
async fn hello(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Welcome {},  name {}!", info.1, info.0)
}

pub struct HeroSevice;

impl HeroSevice {
    pub fn service() -> Scope {
        web::scope("/hello")
            .service(hello)
            .service(read)
            .service(create)
            .service(delete)
            .service(update)

    }
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