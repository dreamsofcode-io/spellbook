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
use tracing::{event,Level};
use std::error::Error;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Spell {
    pub id: i64,
    pub name: String,
    pub damage: (i64, i64),
    pub mana: i32,
}

/*
fn handle_result<T>(res: Result<T, Box<dyn Error>>) -> Result<Json<T>, StatusCode> {
    match res {
        Ok(x) => Ok(Json::<T>::new()),
        Err(e) => {
            tracing::error!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
*/
pub async fn list(State(state): State<AppState>) -> StatusCode {
    StatusCode::OK
}

pub async fn findbyid(State(state): State<AppState>) -> StatusCode {
    StatusCode::OK
}

pub async fn create(State(state): State<AppState>) -> StatusCode {
    StatusCode::OK
}

pub async fn update(State(state): State<AppState>) -> StatusCode {
    StatusCode::OK
}

pub async fn delete(State(state): State<AppState>) -> StatusCode {
    StatusCode::OK
}
