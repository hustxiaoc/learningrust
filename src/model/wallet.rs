use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use sqlx::FromRow;
use sqlx::mysql::MySqlQueryAs;
use sqlx::{query_as};
use crate::model::{BaseModel, utxo::Utxo, QueryValue};
use crate::db;


#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Wallet {
    pub id: u64,
    pub uid: i64,
    pub name: String,
    pub r#type: u8,
    pub token: i8,
    pub share: i8,
    pub threshold: i8,
    pub status: u8,
}

// impl Wallet {
    
// }

#[async_trait]
impl BaseModel for Wallet {
    fn get_table_name() -> &'static str {
        &"wallet"
    }   
}