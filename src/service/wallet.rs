use crate::db;
use redis::AsyncCommands;
use crate::model::{
    user::User, 
    Model, BaseModel, 
    wallet_party::WalletParty,
    wallet::Wallet,
    utxo::Utxo,
};

pub async fn query_by_id(id: u64)  -> Result<Option<Wallet>, Box<dyn std::error::Error>> {
    let utxos = Utxo::query(id).await?;
    println!("utxos = {:?}", utxos);
    Wallet::fetch_by_id(id).await
}