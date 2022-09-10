// main.rs

use sea_orm::{Database, DbErr};
use poem::{get, handler, listener::TcpListener, web::Path, IntoResponse, Route, Server};

//postgreSql
const DATABASE_URL: &str = "postgres://postgres:01010727.@localhost:5432/truck-db";

#[handler]
async fn hello(Path(name): Path<String>) -> String {
    if let Err(err) = run().await {
        panic!("{}", err);
    }
    format!("hello: {}, db is connected!", name)
}

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/hello/:name", get(hello));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
