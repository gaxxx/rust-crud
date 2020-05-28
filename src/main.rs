#[macro_use] extern crate diesel;

use serde::{Deserialize};
use actix_web::{web, App, Responder, HttpServer, get, post};
use diesel::RunQueryDsl;
use std::ops::Deref;

mod db;
mod schema;
mod hero;



#[derive(Deserialize, Debug, Clone)]
struct Info {
    username : String
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
            )
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
