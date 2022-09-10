#[macro_use]
extern crate lazy_static;
extern crate dotenv;

mod entities;

use std::env;
use dotenv::dotenv;
use poem::{get, handler, listener::TcpListener, web::Path, Route, Server};
use entities::*;
use poem_openapi::{OpenApi, payload::PlainText, OpenApiService};
use sea_orm::*;
use entities::user::Entity as UserEntity;
use tokio::sync::OnceCell;
use tracing::log::warn;

lazy_static! {
    static ref DATABASE : OnceCell<DatabaseConnection> = OnceCell::new();
}
struct Api;

#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }
}

#[handler]
async fn hello(Path(name): Path<String>) -> String {
    if let Some(db) = DATABASE.get() {
        let user_query :Result<Option<user::Model>, DbErr> = UserEntity::find_by_id("1232".to_string()).one(db).await;
        if let Ok(Some(user)) = user_query {
            return format!("hello: {}, db is connected!", user.user_name);
        } else {
            return format!("get empty user, hello: {}", name);
        }  
    } else {
        return format!("connect db error");
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let db_con = Database::connect(env::var("DATABASE_URL").unwrap()).await.unwrap();
    if let Err(e) = DATABASE.set(db_con) {
        warn!("set global db error {}", e);
    } 
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    let api_service =
    OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let api_service = Route::new().at("/hello/:name", get(hello));
    let app = Route::new().nest("/", api_service).nest("/docs", ui);
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
