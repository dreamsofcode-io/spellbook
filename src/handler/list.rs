use crate::handler::Spell;
use crate::state::AppState;
use std::error::Error;

#[derive(serde::Deserialize)]
struct Filter {
    name: String,
}

static QUERY: &str = "
SELECT id, name, damage
FROM spell
WHERE name = $1
ORDER BY level DESC
LIMIT 20
";

pub async fn list_spells(
    state: AppState, filter: Filter,
) -> Result<Vec<Spell>, Box<dyn Error>> {
    let db = &state.lock().await.database;
    Ok(sqlx::query_as(QUERY).bind(filter.name).fetch_all(db).await?)
}
