use actix_web::{web, App, Responder, HttpServer, get, post, put, delete, HttpResponse};

use serde_json::json;
mod hero;
use hero::Hero;
use actix_web::web::Path;
use std::sync::Arc;
use diesel::{RunQueryDsl, ExpressionMethods};
use diesel::pg::PgConnection;

#[macro_use] extern crate diesel;

mod db;
mod schema;
use schema::heroes;
use self::diesel::prelude::*;

#[get("/")]
async fn read() -> impl Responder {
    web::Json(vec!["hero", "hero"])
}

#[put("/{id}")]
async fn update(path: Path<i32>, hero: web::Json<Hero>) -> impl Responder {
    println!("path is {}", path.into_inner());
    hero
}

#[delete("/{id}")]
async fn delete(path : Path<i32>) -> impl Responder{
    println!("path is {}", path.into_inner());
    web::Json(json!({"status": "ok"}))
}

#[post("/")]
async fn create(hero: web::Json<Hero>, conn : db::Connection) -> impl Responder {
    let conn: &PgConnection = &conn;
    diesel::insert_into(heroes::table)
        .values(&hero.into_inner())
        .execute(conn)
        .expect("Error creating new hero");

    let hero = heroes::table.order(heroes::id.desc()).first::<Hero>(conn).unwrap();
    web::Json(hero)
}


#[get("/{user}/{name}")]
async fn hello(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Welcome {},  name {}!", info.1, info.0)
}


struct AppState {
    app_name: String,
    pool: Arc<db::Pool>,
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db = db::DBConfig::default();
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(
                web::scope("/hello")
                    .service(hello)
                    .service(read)
                    .service(create)
                    .service(delete)
                    .service(update)
            )
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
