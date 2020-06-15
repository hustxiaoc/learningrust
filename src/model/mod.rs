use serde::{Serialize, Deserialize, de::DeserializeOwned};
use async_trait::async_trait;
use sqlx::MySqlPool;
use sqlx::Pool;
use sqlx::FromRow;
use sqlx::mysql::MySqlRow;
use serde_json::json;
use sqlx::mysql::MySqlQueryAs;
use sqlx::{query, query_as};
use redis::{Value, RedisResult, ToRedisArgs, RedisWrite, FromRedisValue};
use sql_builder::{SqlBuilder, quote};
use std::collections::HashMap;
use crate::db;

pub mod user;
pub mod wallet_party;
pub mod wallet;
pub mod utxo;

// struct QueryFilter((String, QueryValue));

#[derive(Debug, Clone)]
pub enum QueryValue {
    string(String),
    u8(u8),
    u64(u64),
    arr(Box<Vec<QueryValue>>),
}

#[async_trait]
pub trait BaseModel {
    // async fn query_by_id(id: u64) -> Result<Option<Self>, Box<dyn std::error::Error>> 
    // where Self: Sized + Send + for<'c> FromRow<'c, MySqlRow<'c>> 
    // {
    //     let pool = db::get_db_pool().await?;
    //     let table = Self::get_table_name();
    //     let sql = format!("select * from {} where id = ?", table);

    //     let result = query_as::<_, Self>(&sql)
    //         .bind(id)
    //         .fetch_optional(&pool).await?;

    //     Ok(result)
    // }

    fn get_table_name() -> &'static str {
        unimplemented!("get_table_name")
    }

    fn get_sql_value(v: &QueryValue) -> Option<String> {
        match v {
            QueryValue::string(s) => {
                Some(quote(s))
            },
            QueryValue::u8(v) => {                    
                Some(v.to_string())
            },
            QueryValue::u64(v) => {
                Some(v.to_string())
            },
            QueryValue::arr(arr) => {
                None
            },
        }
    }

    fn get_sql_select_builder(filter: Option<HashMap<&str, QueryValue>>) -> SqlBuilder {
        let mut builder = SqlBuilder::select_from(Self::get_table_name());
        match filter {
            Some(filter) => {
                for (k, v) in filter {
                    println!("key is {:?}, value is {:?}", k, v);
                    match Self::get_sql_value(&v) {
                        Some(s) => {
                            builder.and_where_eq(k, s);
                        },
                        None => {
                            match v {
                                QueryValue::arr(arr) => {
                                    let mut list = vec![];
                                    for val in *arr {
                                        list.push(Self::get_sql_value(&val).unwrap());
                                    }
                                    builder.and_where(format!("{} in ( {} )", k, list.join(",")));
                                },
                                _ => {
                                    unreachable!();
                                }
                            };
                        }
                    };
                }
                builder
            },
            None => {
                builder
            }
        }
    }

    async fn fetch_by_id(id: u64) -> Result<Option<Self>, Box<dyn std::error::Error>> 
    where Self: Sized + Send + for<'c> FromRow<'c, MySqlRow<'c>> 
    {
        let mut filter = HashMap::new();
        filter.insert("id", QueryValue::u64(id));
        Self::fetch_one(Some(filter)).await
    }

    async fn fetch_all(filter: HashMap<&str, QueryValue>) -> Result<Vec<Self>, Box<dyn std::error::Error>> 
    where Self: Sized + Send + for<'c> FromRow<'c, MySqlRow<'c>> 
    {
        let pool = db::get_db_pool().await?;
        let builder = Self::get_sql_select_builder(Some(filter));
        let sql = builder.sql().unwrap();
        println!("sql is {:?}", sql);

        let result = query_as::<_, Self>(&sql)
            .fetch_all(&pool).await?;

        Ok(result)
    }

    async fn fetch_one(filter: Option<HashMap<&str, QueryValue>>) -> Result<Option<Self>, Box<dyn std::error::Error>> 
    where Self: Sized + Send + for<'c> FromRow<'c, MySqlRow<'c>> 
    {
        let pool = db::get_db_pool().await?;
        let mut builder = Self::get_sql_select_builder(filter);
        let sql = builder.limit(1).sql().unwrap();
        println!("sql is {:?}", sql);

        let result = query_as::<_, Self>(&sql)
            .fetch_optional(&pool).await?;

        Ok(result)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Model<T:BaseModel>(pub T);

impl<T> ToRedisArgs for Model<T> where T:BaseModel + Serialize { 
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let s = json!(self.0).to_string();
        out.write_arg(s.as_bytes())
    }
}

impl<T> ToRedisArgs for &Model<T> where T:BaseModel + Serialize { 
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let s = json!(self.0).to_string();
        out.write_arg(s.as_bytes())
    }
}


impl<T> FromRedisValue for Model<T> where T: BaseModel + DeserializeOwned {
    fn from_redis_value(v: &Value) -> RedisResult<Model<T>> {
        match *v {
            Value::Data(ref bytes) => {
                let model: Model<T> = serde_json::from_slice(bytes).unwrap();
                Ok(model)
            },
            _ => {
                unreachable!()
            }
        }
    }
}
