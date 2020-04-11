use std::ops::Deref;

use r2d2;
use r2d2_diesel::ConnectionManager;

use diesel::pg::PgConnection;
use actix_web::{FromRequest, HttpRequest, Error};
use actix_web::dev::{PayloadStream, Payload};
use futures;
use futures::{FutureExt};
use futures::future::{LocalBoxFuture, err};
use futures::io::ErrorKind;
use std::io;
use std::time::Duration;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
static DATABASE_URL: Option<&'static str> = option_env!("DATABASE_URL");
static DEFAULT_URL : &'static str = "postgres://woo:jade@home/heros";

pub fn connect() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL.unwrap_or(DEFAULT_URL));
    r2d2::Pool::builder().build(manager).expect("Failed to create pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct Connection(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

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


/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl FromRequest for Connection {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Error>>;
    type Config = DB;

    fn from_request(req: &HttpRequest, _: &mut Payload<PayloadStream>) -> Self::Future {
        let pool = req
            .app_data::<Self::Config>()
            .unwrap();

        match pool.0.try_get() {
            Some(c) => {
                let conn = Connection(c);
                futures::future::ok(conn).boxed_local()
            },
            None => err(io::Error::from(ErrorKind::NotFound).into()).boxed_local()
        }
    }
}

// For the convenience of using an &Connection as an &SqliteConnection.
impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

