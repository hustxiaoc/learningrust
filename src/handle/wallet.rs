use crate::service;
use crate::model::{utxo::Utxo};
use serde::{Serialize, Deserialize};
use serde_json::json;
use serde_json::value::Value;
use tide::http::StatusCode;
use tide::{Request, Server, Error};

#[derive(Debug, Deserialize, Serialize)]
struct WalletQuery {
    wid: Option<u64>,
}


pub async fn query_by_id(params: Option<String>) -> Result<Value, Error> {
    let params = params.ok_or(Error::from_str(StatusCode::BadRequest, "method is required"))?;
    let query: WalletQuery = serde_json::from_str(&params).map_err(|err| {
        println!("err is {:?}", err);
        err
    })?;
    let id = query.wid.as_ref().ok_or(Error::from_str(StatusCode::BadRequest, "wid is required"))?;

    let wallet = service::wallet::query_by_id(*id).await.map_err(|err| {
        println!("wallet query_by_id err is {:?}", err);
        Error::from_str(StatusCode::InternalServerError, format!("{}", err))
    })?;

    let utxos = Utxo::query(*id).await.map_err(|err| {
        println!("wallet query utxo err is {:?}", err);
        Error::from_str(StatusCode::InternalServerError, format!("{}", err))
    })?;
    println!("utxos = {:?}", utxos);

    Ok(json!({
        "wallet": wallet,
        "utxos": utxos,
    }))
}