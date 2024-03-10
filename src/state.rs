#![allow(dead_code)]
use crate::handler::Spell;
use fred::interfaces::KeysInterface;
use fred::{clients::RedisPool, prelude::*};
use serde_json::Value;
use sqlx::postgres::PgPool;
use std::error::Error;

pub struct StateInternal {
    pub database: sqlx::postgres::PgPool,
    pub cache: Cache
}

impl StateInternal {
    pub fn new(db: PgPool, redis: RedisPool) -> Self {
        StateInternal{
            database: db,
            cache: Cache{ internal: redis },
        }
    }
}

pub struct Cache {
    internal: RedisPool,
}

impl Cache {
    fn key_for_id(id: i64) -> String {
        format!("spell:{}", id)
    }

    pub async fn get(&mut self, id: i64) -> Result<Option<Spell>, Box<dyn Error>> {
        let value: Option<Value> = self.internal.get(Self::key_for_id(id)).await?;

        let spell = match value {
            Some(x) => match serde_json::from_value(x) {
                Ok(x) => Some(x),
                Err(_) => None,
            },
            None => None,
        };
        Ok(spell)
    }

    pub async fn set(
        &mut self,
        id: i64,
        spell: &Spell,
        expiration: Option<Expiration>,
        set_opts: Option<SetOptions>,
        get: bool
    ) -> Result<(), Box<dyn Error>> {
        let value: Value = serde_json::to_value(spell)?;
        let key = Self::key_for_id(id);
        self.internal.set(key, value.to_string(), expiration, set_opts, get).await?;
        Ok(())
    }
}

pub type AppState = std::sync::Arc<tokio::sync::Mutex<StateInternal>>;
