#[macro_use] extern crate diesel;

use serde::{Deserialize};
use actix_web::{web, App, Responder, HttpServer, get, post, delete, Either};
use diesel::RunQueryDsl;
use std::ops::Deref;
use diesel::prelude::*;

mod db;
mod schema;
mod hero;


#[derive(Deserialize, Debug, Clone)]
struct Info {
    username : String
}

#[derive(Deserialize, Debug)]
struct PageRequest {
    start : Option<i32>,
}

#[get("/{username}")]
async fn hello(user : web::Path<Info> ) -> impl Responder {
    format!("Hello world {}", user.username)
}

#[post("/")]
async fn create(hero: web::Json<hero::HeroInput>, db: db::DB) -> impl Responder {
    println!("input is {:?}", hero);
    let created : hero::Hero = diesel::insert_into(schema::users::table)
        .values(hero.into_inner())
        .get_result(db.get().deref()).expect("Error creagint");
    println!("output is {:?}", created);
    web::Json(created)
}

#[get("/{id}")]
async fn get(update_id : web::Path<i32> , db : db::DB) -> impl Responder {
    use schema::users::dsl::*;
    let r = users.filter(id.eq(update_id.into_inner()))
        .first::<hero::Hero>(&*db.get());
    match r {
        Ok(v) => {
            return Either::A(web::Json(v))
        },
        Err(e) => {
            println!("error get {:?}", e);
        }
    }
    Either::B(web::HttpResponse::NotFound().await)
}

#[post("/{id}")]
async fn update(update_id : web::Path<i32> , hero : web::Json<hero::HeroInput>, db : db::DB) -> impl Responder {
    use schema::users::dsl::*;
    println!("input is {:?}", hero);
    let updated : hero::Hero = diesel::update(users.filter(id.eq(update_id.into_inner())))
        .set(hero.into_inner())
        .get_result(db.get().deref()).expect("Error creagint");
    println!("output is {:?}", updated);
    web::Json(updated)
}

#[delete("/{id}")]
async fn delete(update_id : web::Path<i32>, db : db::DB) -> impl Responder {
    use schema::users::dsl::*;
    let count = diesel::delete(users.filter(id.eq(update_id.into_inner())))
        .execute(&*db.get()).unwrap();
    println!("delete count {}", count);
    "".with_header("content-type", "json")
}

#[get("/")]
async fn gets(req : web::Query<PageRequest> ,db : db::DB) -> impl Responder {
    use schema::users::dsl::*;
    use crate::db::pagination::Paginate;
    let query = users.order(id.asc());
    let query = query.paginate(req.start.unwrap_or(0).into());
    let all_users : Vec<_> = query.load::<(hero::Hero, i64)>(&*db.get()).unwrap();
    web::Json(all_users.iter().map(|v| v.0.clone()).collect::<Vec<_>>())
}





#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(db::DB::default())
            .service(
                web::scope("/hello")
                    .service(hello)
            )
            .service(
                web::scope("/api")
                    .service(create)
                    .service(update)
                    .service(get)
                    .service(gets)
                    .service(delete)
            )
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
