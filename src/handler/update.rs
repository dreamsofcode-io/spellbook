use crate::handler::Spell;
use crate::state::AppState;
use std::error::Error;

#[derive(serde::Deserialize)]
struct UpdateBody {
    damage: i32
}

static QUERY: &str = "
UPDATE spell SET damage = $1 WHERE id = $2
";

pub async fn update(state: AppState, id: i32, body: UpdateBody) -> Result<(), Box<dyn Error>> {
    let db = &state.lock().await.database;
    sqlx::query(QUERY).bind(body.damage).bind(id).execute(db).await?;
    Ok(())
}
