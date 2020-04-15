use actix_web::web::Path;
use actix_web::{web, App, Responder, HttpServer, get, post, put, delete, Either, Scope, HttpResponse};
use serde_json::json;
use crate::db;
use crate::schema::heroes;
use crate::util::PageRequest;

use crate::models::hero::Hero;
use actix_web::dev::HttpServiceFactory;
use std::ops::Deref;
use diesel::prelude::*;
use crate::protos::hero::Hero as PHero;
use std::alloc::handle_alloc_error;
use actix_web::http::StatusCode;
use crate::db::pagination::{Paginate, DEFAULT_PER_PAGE};



#[get("/")]
async fn gets(info : web::Query<PageRequest>, conn : db::Connection) -> impl Responder {
    let page = info.into_inner();
    web::Json(HeroSevice::gets(page, &conn))
}

#[get("/{id}")]
async fn get(path : Path<i32>, conn : db::Connection) -> impl Responder {
    match HeroSevice::get(path.into_inner(), &conn) {
        Some(v) => {
            Either::A(web::Json(v))
        },
        None => {
            Either::B(HttpResponse::new(StatusCode::NOT_FOUND))
        }
    }
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


#[get("/{hometown}/{name}")]
async fn find(info: web::Path<(String, String)>, conn: db::Connection) -> impl Responder {
    format!("Welcome {} from {}!", info.1, info.0);
    web::Json(HeroSevice::find(info.0.clone(), info.1.clone(), &conn))
}

pub struct HeroSevice;

impl HeroSevice {
    pub fn service() -> Scope {
        web::scope("/hello")
            .service(gets)
            .service(find)
            .service(get)
            .service(create)
            .service(delete)
            .service(update)

    }

    pub fn find(hometown : String, name : String, conn : &db::Connection) -> Option<Hero> {
        heroes::table.filter(heroes::dsl::name.eq(name))
            .filter(heroes::dsl::hometown.eq(hometown)).first(conn.deref()).ok()
    }

    pub fn create(hero: Hero, connection: &db::Connection) -> Hero {
        diesel::insert_into(heroes::table)
            .values(&hero)
            .execute(connection.deref())
            .expect("Error creating new hero");

        heroes::table.order(heroes::id.desc()).first(connection.deref()).unwrap()
    }

    pub fn get(id : i32, conn: &db::Connection) -> Option<Hero> {
        heroes::table.find(id).first::<Hero>(conn.deref()).ok()
    }

    pub fn gets(page : PageRequest, conn: &db::Connection) -> Vec<Hero> {
        // heroes::table.order(heroes::id.asc()).load::<Hero>(conn.deref()).unwrap()
        let query = heroes::table.order(heroes::id.asc()).paginate(page.start.unwrap_or(0) as i64).count(page.count.unwrap_or(DEFAULT_PER_PAGE as u32) as i64);
        let (heros, total) =
            query.load_and_count::<Hero>(&conn).unwrap();
        println!("total count {}",total);
        heros
    }

    pub fn update(id: i32, hero: Hero, connection: &db::Connection) -> bool {
        let updated_row = diesel::update(heroes::table.find(id)).set(&hero).execute(connection.deref());
        updated_row.is_ok() && updated_row.unwrap() > 0
    }

    pub fn delete(id: i32, connection: &db::Connection) -> bool {
        diesel::delete(heroes::table.find(id)).execute(connection.deref()).is_ok()
    }
}