use sqlx::MySqlPool;
use sqlx::Pool;
use std::env;
use tide::{Request, Server, Error};
use serde::{Serialize, Deserialize};
use tide::http::StatusCode;
use tokio::net::{TcpListener, TcpStream};
use futures_util::StreamExt;
use std::thread;
use tokio::runtime::Runtime;
use tokio_tungstenite::{accept_async, tungstenite};
use futures_util::sink::SinkExt;
use dotenv;
mod service;
mod model;
mod db;
mod handle;

#[derive(Debug)]
pub struct State {
    db_pool: MySqlPool,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: u64,
    pub email: Option<String>,
    pub xpub: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct UserQuery {
    id: Option<u64>,
    name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WalletQuery {
    id: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiParams {
    method: Option<String>,
    params: Option<String>,
}

async fn accept_connection(stream: TcpStream) {
    if let Err(_) = handle_connection(stream).await {

    }
}

async fn handle_connection(stream: TcpStream) -> tungstenite::Result<()> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            println!("got a message {:?}", msg);
            ws_stream.send(msg).await?;
        }
    }

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();

    let server_addr = env::var("SERVER_ADDR").expect("`SERVER_ADDR` must be set to run this app");

    thread::spawn(|| {
        let mut rt = Runtime::new().unwrap();
        rt.block_on(async {
        
            let addr = env::args()
                .nth(1)
                .unwrap_or_else(|| "127.0.0.1:8080".to_string());

            // Create the event loop and TCP listener we'll accept connections on.
            let try_socket = TcpListener::bind(&addr).await;
            let mut listener = try_socket.expect("Failed to bind");

            while let Ok((stream, _)) = listener.accept().await {
                tokio::spawn(accept_connection(stream));
            }
        });
    });

    // init acm
    db::init_acm();

    let db_url = env::var("DATABASE_URL").expect("`DATABASE_URL` must be set to run this app");
    let db_pool:MySqlPool = Pool::new(&db_url).await.expect("can not connect to mysql server");

    // let tables = query!("show tables").fetch_all(&db_pool).await.unwrap();
    // for table in tables {
    //     println!("tables is {:?}", table);
    //     let cols = query!("show columns from ?", tables.unwrap()).fetch_all(&db_pool).await.unwrap();
    //     println!("table cols is {:?}", cols);
    // }
    
    let mut app: Server<State> = Server::with_state(State { db_pool });

    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.at("/api.json").all(|req: Request<State>| async move {
        
        let api_params = req.query::<ApiParams>().map_err(|err| {
            println!("req query error {:?}", err);
            err
        })?;

        let method = api_params.method.as_ref().ok_or(Error::from_str(StatusCode::BadRequest, "method is required"))?.as_str();
    
        match method {
            "user.query" => {
                handle::user::query(api_params.params).await
            },

            "wallet.queryById" => {
                handle::wallet::query_by_id(api_params.params).await
            },

            _ => {
                return Err(Error::from_str(StatusCode::BadRequest, "api not found"));
            }
        }

    });

    let server = app.listen(&server_addr);
    println!("server is running at http://{}", server_addr);
    server.await?;
    Ok(())
}
