use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use secrecy::{ExposeSecret, SecretString};

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection(url: SecretString) -> DbPool {
    let manager = ConnectionManager::<MysqlConnection>::new(url.expose_secret());

    Pool::builder()
        .build(manager)
        .expect("Failed to create db pool")
}
