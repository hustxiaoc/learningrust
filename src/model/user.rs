use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use sqlx::FromRow;
use sqlx::mysql::MySqlQueryAs;
use sqlx::{query_as};
use crate::model::{BaseModel, QueryValue};
use crate::db;
use std::collections::HashMap;

type UserResult = Result<Option<User>, Box<dyn std::error::Error>> ;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: u64,
    pub email: Option<String>,
}

impl User {
    pub async fn query_by_email(email: &str) -> UserResult {
        let mut filter = HashMap::new();
        filter.insert("email", QueryValue::string(email.to_owned()));
        Self::fetch_one(Some(filter)).await
    }
}


#[async_trait]
impl BaseModel for User {
    fn get_table_name() -> &'static str {
        &"user"
    }   
}

