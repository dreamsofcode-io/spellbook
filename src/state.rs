pub struct StateInternal {
    pub database: sqlx::postgres::PgPool,
    pub cache: redis::Connection,
}

pub type AppState = std::sync::Arc<tokio::sync::Mutex<StateInternal>>;
