#[macro_use] extern crate diesel;
mod hero;
mod db;

use hero::Hero;
mod schema;
use actix_web::web::Path;
use actix_web::{web, App, Responder, HttpServer, get, post, put, delete, Either};
use serde_json::json;

#[get("/")]
async fn read() -> impl Responder {
    web::Json(vec!["hero", "hero"])
}

#[put("/{id}")]
async fn update(path: Path<i32>, hero: web::Json<Hero>, conn: db::Connection) -> impl Responder {
    let hero = hero.into_inner();
    if Hero::update(path.into_inner(), hero.clone(), &conn) {
        Either::A(web::Json(hero))
    } else {
        Either::B(web::Json(json!({"status": "error"})))
    }
}

#[delete("/{id}")]
async fn delete(path : Path<i32>, db: db::DB) -> impl Responder{
    Hero::delete(path.into_inner(), &db.get());
    web::Json(json!({"status": "ok"}))
}

#[post("/")]
async fn create(hero: web::Json<Hero>, db: db::DB) -> impl Responder {
    let hero = Hero::create(hero.into_inner(), &db.get());
    web::Json(hero)
}


#[get("/{user}/{name}")]
async fn hello(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Welcome {},  name {}!", info.1, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db = db::DB::default();
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
