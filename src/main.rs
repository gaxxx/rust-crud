use serde::{Deserialize};
use actix_web::{web, App, Responder, HttpServer, get};


#[derive(Deserialize, Debug, Clone)]
struct Info {
    username : String
}

#[get("/{username}")]
async fn hello(user : web::Path<Info> ) -> impl Responder {
    format!("Hello world {}", user.username)
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/hello")
                    .service(hello)
            )
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
