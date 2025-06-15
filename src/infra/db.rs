use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use secrecy::{ExposeSecret, SecretString};

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection(url: SecretString) -> DbPool {
    let manager = ConnectionManager::<MysqlConnection>::new(url.expose_secret());

    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create db pool");

    let mut conn = pool.get().expect("Failed to get connection");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    pool
}
