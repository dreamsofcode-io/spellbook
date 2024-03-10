pub struct StateInternal {
    pub database: sqlx::postgres::PgPool,
    pub cache: redis::Client,
}

pub type AppState = std::sync::Arc<tokio::sync::Mutex<StateInternal>>;
