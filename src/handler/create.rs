use crate::handler::Spell;
use crate::state::AppState;
use std::error::Error;
use std::collections::Bound;

#[derive(serde::Deserialize)]
pub struct CreateSpell {
    pub name: String,
    pub damage: i32,
}

static QUERY: &str = "
INSERT INTO spell
(name, damage, mana, level)
VALUES
($1, $2, $3, $4)
RETURNING (id, name, damage)
";

pub async fn create(state: AppState, spell: CreateSpell) -> Result<Spell, Box<dyn Error>> {
    let db = &state.lock().await.database;

    let spell = sqlx::query_as(QUERY)
        .bind(&spell.name)
        .bind(&spell.damage)
        .fetch_one(db).await?;

    Ok(spell)
}
