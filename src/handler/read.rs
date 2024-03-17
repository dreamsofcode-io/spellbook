#![allow(unused_imports)]
use crate::handler::Spell;
use crate::state::AppState;
use fred::prelude::*;
use std::error::Error;

static QUERY: &str = "
SELECT id, name, damage, created_at, updated_at
FROM spell
WHERE id = $1
";

pub async fn find_by_id(state: AppState, id: i64) -> Result<Option<Spell>, Box<dyn Error>> {
    let res: Option<Spell> = sqlx::query_as(QUERY)
        .bind(id)
        .fetch_optional(&state.database)
        .await?;

    tracing::info!("returning database version");
    Ok(res)
}
