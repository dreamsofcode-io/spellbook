use fred::clients::RedisPool;

pub struct StateInternal {
    pub database: sqlx::postgres::PgPool,
    pub cache: RedisPool
}

pub type AppState = std::sync::Arc<tokio::sync::Mutex<StateInternal>>;
