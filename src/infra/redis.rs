use secrecy::{ExposeSecret, SecretString};
use tower_sessions_redis_store::fred::prelude::{Pool as RedisPool, *};

pub async fn establish_connection(url: SecretString) -> RedisPool {
    let cfg = Config::from_url(url.expose_secret()).expect("Failed to create redis config");
    let pool = RedisPool::new(cfg, None, None, None, 6).expect("Failed to create redis pool");

    pool.connect();
    pool.wait_for_connect()
        .await
        .expect("Failed to connect to redis");

    pool
}
