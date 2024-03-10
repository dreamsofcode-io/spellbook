mod handler;
mod state;
use axum::routing::{delete, get, post, put, Router};
use std::error::Error;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let pg_url = "postgres://postgres::postrgres@localhost/test";
    let redis_url = "redis://localhost";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(pg_url).await?;

    tracing::info!("Running migrations. This can take a minute or so...");
    sqlx::migrate!().run(&pool).await?;
    tracing::info!("Migrations complete");

    let client = redis::Client::open(redis_url)?;

    let state = Arc::new(Mutex::new(state::StateInternal {
        database: pool,
        cache: client.get_connection()?,
    }));

    // build our application with a route
    let app = Router::new()
        .route("/spells", get(handler::list))
        .route("/spells/:id", get(handler::findbyid))
        .route("/spells", post(handler::create))
        .route("/spells", put(handler::update))
        .route("/spells", delete(handler::delete))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(
        "0.0.0.0:3000",
    ).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
