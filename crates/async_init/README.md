# async_init
init object asynchronously without Arc<Mutex>


```rust
#[async_init]
pub async fn get_db_pool() -> Result<MySqlPool, sqlx::Error> {
    println!("thread id is {:?}, create mysql pool", thread::current().id());
    let db_url = env::var("DATABASE_URL").expect("`DATABASE_URL` must be set to run this app");
    let pool:MySqlPool = Pool::new(&db_url).await?;
    Ok(pool)
}
```