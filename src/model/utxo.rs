use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use sqlx::FromRow;
use sqlx::mysql::MySqlQueryAs;
use sqlx::{query_as};
use crate::model::{BaseModel, QueryValue};
use crate::db;
use std::collections::HashMap;

type UtxoListResult = Result<Vec<Utxo>, Box<dyn std::error::Error>>;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Utxo {
    pub id: String,
    pub wid: i64,
    pub txid: String,
    pub address: String,
    pub vout: i16,
    pub status: i8,
    pub value: i64,
}

impl Utxo {
    pub async fn query(wid: u64) -> UtxoListResult {
        let mut filter = HashMap::new();
        filter.insert("wid", QueryValue::u64(wid));
        filter.insert("status", QueryValue::arr(Box::new(vec![QueryValue::u8(1), QueryValue::u8(2)])));
        Self::fetch_all(filter).await
    }
}

#[async_trait]
impl BaseModel for Utxo {
    fn get_table_name() -> &'static str {
        &"utxo"
    }
}