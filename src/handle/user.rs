use crate::service;
use serde::{Serialize, Deserialize};
use serde_json::json;
use serde_json::value::Value;
use tide::http::StatusCode;
use tide::{Request, Server, Error};

#[derive(Debug, Deserialize, Serialize)]
struct UserQuery {
    id: Option<u64>,
    name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct VerifySesssionParams {
    t: u64,
    sig: String,
    token: String,
}


pub async fn query(params: Option<String>) -> Result<Value, Error> {
    let params = params.ok_or(Error::from_str(StatusCode::BadRequest, "params is required"))?;
    let query: UserQuery = serde_json::from_str(&params).map_err(|err| {
        println!("err is {:?}", err);
        err
    })?;
    
    let id = query.id.as_ref().ok_or(Error::from_str(StatusCode::BadRequest, "id is required"))?;

    let user = service::user::query_user_by_id(*id).await.map_err(|err| {
        println!("query_user_by_id err is {:?}", err);
        Error::from_str(StatusCode::InternalServerError, format!("{}", err))
    })?;

    Ok(json!(user))
}

pub async fn verify_session(params: Option<String>) -> Result<Value, Error> {
    let params = params.ok_or(Error::from_str(StatusCode::BadRequest, "params is required"))?;
    let params: VerifySesssionParams = serde_json::from_str(&params).map_err(|err| {
        println!("err is {:?}", err);
        err
    })?;

    let info: Vec<&str> = params.token.split(":").collect();
    if info.len() != 2 {
        return Err(Error::from_str(StatusCode::BadRequest, "Invalid token".to_owned()));
    }

    let uid: u64 = info[0].parse().unwrap();

    Ok(json!({

    }))
}
