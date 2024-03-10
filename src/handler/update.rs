#![allow(unused_imports)]
use crate::state::AppState;
use std::error::Error;
use crate::handler::Spell;
use fred::prelude::*;

#[derive(serde::Deserialize)]
pub struct UpdateBody {
    pub damage: i32
}

static QUERY: &str = "
UPDATE spell SET damage = $1, updated_at = now() WHERE id = $2
RETURNING id, name, damage, created_at, updated_at
";

pub async fn update(
    state: AppState, id: i64, body: UpdateBody
) -> Result<Option<Spell>, Box<dyn Error>> {
    tracing::info!("updating spell: {}", id);

    let s = state.lock().await;

    let res: Option<Spell> = sqlx::query_as(QUERY)
        .bind(body.damage)
        .bind(id)
        .fetch_optional(&s.database).await?;

    Ok(res)
}
