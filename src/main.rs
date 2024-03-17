mod handler;
mod state;
use axum::routing::{delete, get, post, put, Router};
use dotenv::dotenv;
use fred::prelude::*;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let pg_url = std::env::var("DATABASE_URL")?;
    let redis_url = match std::env::var("REDIS_URL")?.as_str() {
        "" => "redis://localhost:5432".to_string(),
        x => x.to_string(),
    };

    let dbpool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_url)
        .await?;

    sqlx::migrate!().run(&dbpool).await?;

    let pool_size = 8;
    let config = RedisConfig::from_url(&redis_url)?;

    let redis_pool = Builder::from_config(config)
        .with_performance_config(|config| {
            config.auto_pipeline = true;
        })
        .set_policy(ReconnectPolicy::new_exponential(0, 100, 30_000, 2))
        .build_pool(pool_size)
        .expect("Failed to create redis pool");

    if std::env::var("REDIS_URL")? != "" {
        redis_pool.init().await.expect("Failed to connect to redis");
        let _ = redis_pool.flushall::<i32>(false).await;
    }

    let state = Arc::new(state::StateInternal::new(dbpool, redis_pool));

    // build our application with a route
    let app = Router::new()
        .route("/spells", post(handler::create))
        .route("/spells", get(handler::list))
        .route("/spells/:id", get(handler::read))
        .route("/spells/:id", put(handler::update))
        .route("/spells/:id", delete(handler::delete))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
