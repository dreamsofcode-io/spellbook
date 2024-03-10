use crate::handler::Spell;
use crate::state::AppState;
use std::error::Error;

#[derive(serde::Deserialize)]
pub struct CreateSpellBody {
    pub name: String,
    pub damage: i32,
}

static QUERY: &str = "
INSERT INTO spell
(name, damage)
VALUES ($1, $2)
RETURNING (id, name, damage, created_at, updated_at)
";

pub async fn create(state: AppState, spell: CreateSpellBody) -> Result<Spell, Box<dyn Error>> {
    let db = &state.lock().await.database;

    let spell = sqlx::query_as(QUERY)
        .bind(&spell.name)
        .bind(&spell.damage)
        .fetch_one(db).await?;

    Ok(spell)
}
