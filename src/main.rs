#[macro_use] extern crate diesel;
mod schema;
mod controls;
mod models;
mod db;
mod protos;
mod util;

use actix_web::web::Path;
use actix_web::{web, App, Responder, HttpServer, get, post, put, delete, Either};
use serde_json::json;
use controls::hero_service::HeroSevice;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db = db::DB::default();
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(HeroSevice::service(),
            )
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
