use crate::state::AppState;
use std::error::Error;

#[derive(serde::Deserialize)]
pub struct UpdateBody {
    pub damage: i32
}

static QUERY: &str = "
UPDATE spell SET damage = $1 WHERE id = $2
";

pub async fn update(state: AppState, id: i64, body: UpdateBody) -> Result<u64, Box<dyn Error>> {
    let db = &state.lock().await.database;
    let res = sqlx::query(QUERY).bind(body.damage).bind(id).execute(db).await?;
    Ok(res.rows_affected())
}
