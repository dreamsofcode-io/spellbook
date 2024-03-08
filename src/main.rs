mod handler;
use axum::routing::{delete, get, post, Router};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/spells", get(handler::list_spells))
        .route("/spells/recent", get(handler::recent_spells))
        .route("/wizards", get(handler::list_wizards))
        .route("/wizards/:id", get(handler::wizard_for_id))
        .route("/spellbooks", post(handler::learn_spell))
        .route("/spellbooks", delete(handler::forget_spell));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Hello, world!");
    Ok(())
}
