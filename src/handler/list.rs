use crate::handler::Spell;
use crate::state::AppState;
use std::error::Error;

static QUERY: &str = "
SELECT id, name, damage, created_at, updated_at
FROM spell
";

pub async fn list_spells(state: AppState) -> Result<Vec<Spell>, Box<dyn Error>> {
    let db = &state.lock().await.database;
    Ok(sqlx::query_as(QUERY).fetch_all(db).await?)
}
