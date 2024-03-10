use crate::handler::Spell;
use crate::state::AppState;
use std::error::Error;

static QUERY: &str = "
DELETE FROM spell WHERE id = $1
";

pub async fn list_spells(
    state: AppState, id: i32,
) -> Result<(), Box<dyn Error>> {
    let db = &state.lock().await.database;
    sqlx::query(QUERY).bind(id).execute(db).await?;
    Ok(())
}
