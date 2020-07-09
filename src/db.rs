pub mod pagination;

use std::ops::Deref;

use r2d2;
use r2d2_diesel::ConnectionManager;

use diesel::pg::PgConnection;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::{PayloadStream, Payload};
use futures;
use futures::{FutureExt};
use futures::future::{LocalBoxFuture};
use std::time::Duration;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
static DATABASE_URL: Option<&'static str> = option_env!("DATABASE_URL");
static DEFAULT_URL : &'static str = "postgres://postgres:password@127.0.0.1/heroes";

pub fn connect() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL.unwrap_or(DEFAULT_URL));
    r2d2::Pool::builder().max_size(5).build(manager).expect("Failed to create pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct Connection(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

// For the convenience of using an &Connection as an &SqliteConnection.
impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct DB(pub Pool);

impl DB {
    pub fn get(&self) -> Connection {
        Connection(self.0.get().unwrap())
    }

    pub fn get_timeout(&self, timeout : Duration) -> Connection {
        Connection(self.0.get_timeout(timeout).unwrap())
    }

    pub fn try_get(&self) -> Option<Connection> {
        self.0.try_get().map(|v| Connection(v))
    }
}

impl Default for DB {
    fn default() -> Self {
        DB(connect())
    }
}


impl FromRequest for DB {
    type Error = ();
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;
    type Config = DB;

    fn from_request(req: &HttpRequest, _: &mut Payload<PayloadStream>) -> Self::Future {
        let config= req.app_data::<Self::Config>().unwrap();
        futures::future::ok(config.clone()).boxed_local()
    }
}

