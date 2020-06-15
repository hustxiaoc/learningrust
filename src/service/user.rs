use crate::db;
use redis::AsyncCommands;
use crate::model::{user::User, Model, BaseModel, wallet_party::WalletParty};

pub async fn query_user_by_id(uid: u64) -> Result<Option<User>, Box<dyn std::error::Error>> {
    let wallets = WalletParty::query_list(uid, None).await.unwrap();
        println!("wallets = {:?}", wallets);

    let mut redis = db::get_redis().await?;
    let key: String = format!("user_id_{}", uid);

    // ignore redis error
    if let Ok(model) = redis.get::<'_, &str, Option<Model<User>>>(&key).await {
        if model.is_some() {
            return Ok(model.map(|f| f.0));
        }
    }

    let user = User::fetch_by_id(uid).await?;

    match user {
        Some(u) => {
            // todo log redis error
            if let Err(err) = redis.set_ex::<&str, Model<User>, String>(&key, Model(u.clone()), 60 * 30).await {
                // println!("redis.set_ex err {:?}", err);
            }
            Ok(Some(u))
        },
        None => {
            Ok(None)
        }
    }
}


pub async fn query_wallet_party(uid: u64) {

}