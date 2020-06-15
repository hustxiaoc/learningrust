use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use sqlx::FromRow;
use sqlx::mysql::MySqlQueryAs;
use sqlx::{query_as};
use crate::model::BaseModel;
use crate::db;


#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct WalletParty {
    pub id: u64,
    pub wid: i64,
    pub r#type: u8,
    pub role: i8,
    pub uid: i64,
    pub status: u8,
   
}

impl WalletParty {
    pub async fn query(uid: u64, wallet_id: u64) -> Result<Option<Self>, Box<dyn std::error::Error>> {
        let pool = db::get_db_pool().await?;
        let result = query_as::<_, Self>("select * from wallet_party where uid = ? & wid = ? limit 1")
            .bind(uid)
            .bind(wallet_id)
            .fetch_optional(&pool).await?;

        Ok(result)
    }

    pub async fn query_list(uid: u64, wallet_type: Option<usize>) -> Result<Vec<WalletParty>, Box<dyn std::error::Error>> {
        let pool = db::get_db_pool().await?;
        let sql = match wallet_type {
            Some(t) => {
                format!("select * from wallet_party where uid = {} and type = {}", uid, t)
            },
            None => {
                format!("select * from wallet_party where uid = {}", uid)
            }
        };

        let result = query_as::<_, Self>(&sql)
            .fetch_all(&pool).await?;
        Ok(result)
    }
}

#[async_trait]
impl BaseModel for WalletParty {
    fn get_table_name() -> &'static str {
        &"wallet_party"
    }
    
}