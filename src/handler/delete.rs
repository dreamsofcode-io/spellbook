use crate::state::AppState;
use std::error::Error;

static QUERY: &str = "
DELETE FROM spell WHERE id = $1
";

pub async fn delete_spell(
    state: AppState, id: i64,
) -> Result<u64, Box<dyn Error>> {
    tracing::info!("deleting spell: {}", id);

    let s = state.lock().await;

    let res = sqlx::query(QUERY)
        .bind(id)
        .execute(&s.database).await?;

    Ok(res.rows_affected())
}
