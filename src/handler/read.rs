use crate::handler::Spell;
use crate::state::AppState;
use std::error::Error;

static QUERY: &str = "
SELECT id, name, damage
FROM spell
WHERE id = $1
";

pub async fn list_spells(
    state: AppState, id: i32,
) -> Result<Vec<Spell>, Box<dyn Error>> {
    let db = &state.lock().await.database;
    Ok(sqlx::query_as(QUERY).bind(id).fetch_all(db).await?)
}
