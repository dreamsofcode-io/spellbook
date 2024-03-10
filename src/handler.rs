mod create;
mod delete;
mod list;
mod read;
mod update;

use axum::http::StatusCode;
use axum::{extract::{State, Path}, Json};
use serde::{Serialize,Deserialize};
use crate::state::AppState;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Spell {
    pub id: i64,
    pub name: String,
    pub damage: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<Spell>>, StatusCode> {
    match list::list_spells(state).await {
        Ok(x) => Ok(Json(x)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn read(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Spell>, StatusCode> {
    match read::find_by_id(state, id).await {
        Ok(x) => match x {
            Some(x) => Ok(Json(x)),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<create::CreateSpellBody>,
) -> Result<(StatusCode, Json<Spell>), StatusCode> {
    let res = create::create(state, body).await;
    match res {
        Ok(x) => Ok((StatusCode::CREATED, Json(x))),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<update::UpdateBody>,
) -> StatusCode {
    match update::update(state, id, body).await {
        Ok(x) => {
            match x {
                None => StatusCode::NOT_FOUND,
                _ => StatusCode::OK,
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> StatusCode {
    match delete::delete_spell(state, id).await {
        Ok(rows) => {
            match rows {
                0 => StatusCode::NOT_FOUND,
                _ => StatusCode::OK,
            }
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
