use sqlx::MySqlPool;
use sqlx::Pool;
use std::env;
use std::cell::RefCell;
use std::thread;
use redis::{AsyncCommands, Client};
use redis::aio::Connection;
use async_init::{ async_init };
use acm::AcmClient;
use futures_util::{
    stream::{Stream, StreamExt},
};
use tokio::runtime::Runtime;

#[async_init]
pub async fn get_db_pool() -> Result<MySqlPool, sqlx::Error> {
    println!("thread id is {:?}, create mysql pool", thread::current().id());
    let db_url = env::var("DATABASE_URL").expect("`DATABASE_URL` must be set to run this app");
    let pool:MySqlPool = Pool::new(&db_url).await?;
    Ok(pool)
}

#[async_init]
async fn get_redis_client() -> Result<Client, redis::RedisError> {
    redis::Client::open("redis://127.0.0.1/")
}

pub async fn get_redis() -> Result<Connection, redis::RedisError> {
    println!("thread id is {:?}", thread::current().id());
    let client = get_redis_client().await?;
    Ok(client.get_async_connection().await?)
}


pub fn init_acm() {
    let tenant =  env::var("tenant").expect("acm tenant must be set"); 
    let access_key = env::var("access_key").expect("acm tenant must be set"); 
    let access_secret = env::var("access_secret").expect("access_secret must be set"); 
    let endpoint = env::var("endpoint").unwrap_or("acm.aliyun.com".to_string());

    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mut client = AcmClient::new(tenant, access_key, access_secret, endpoint);
        let result = client.get_config("acm_test", "DEFAULT_GROUP").await.unwrap();
        println!("result is {:?}", result);
    
        let mut stream = client.subscribe("acm_test", "DEFAULT_GROUP");
    
        tokio::spawn(async move {
            let mut stream = client.subscribe("acm_test", "DEFAULT_GROUP");
            while let Some(message) = stream.next().await {
                println!("config change1: {:?}", message);
            }
         });
    
        while let Some(message) = stream.next().await {
            println!("config change2: {:?}", message);
        } 
    });
}