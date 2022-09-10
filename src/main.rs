#[macro_use]
extern crate lazy_static;
extern crate dotenv;

mod entities;
mod user_service;

use dotenv::dotenv;
use entities::user::Entity as UserEntity;
use poem::{handler, listener::TcpListener, web::Path, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
use sea_orm::*;
use std::env;
use tokio::sync::OnceCell;
use tracing::log::warn;

use user_service::controller::UserRouter;

use crate::entities::user;

lazy_static! {
    static ref DATABASE: OnceCell<DatabaseConnection> = OnceCell::new();
}
struct Api;

#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello Root")
    }
}

#[handler]
async fn hello(Path(name): Path<String>) -> String {
    if let Some(db) = DATABASE.get() {
        let user_query: Result<Option<user::Model>, DbErr> =
            UserEntity::find_by_id("1232".to_string()).one(db).await;
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

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    let db_con = Database::connect(env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    if let Err(e) = DATABASE.set(db_con) {
        warn!("set global db error {}", e);
    }

    let bind_addr = format!(
        "{}:{}",
        env::var("SERVER").unwrap(),
        env::var("PORT").unwrap()
    );

    let api_service = OpenApiService::new(Api, "Hello World", "1.0").server(&bind_addr);
    let ui = api_service.swagger_ui();

    let user_service = OpenApiService::new(UserRouter, "User Service", "1.0").server(&bind_addr);
    let user_ui = user_service.swagger_ui();

    let app = Route::new()
        .nest("/", api_service)
        .nest("/user", user_service)
        .nest("/docs", ui)
        .nest("/docs/user", user_ui);
    Server::new(TcpListener::bind(&bind_addr)).run(app).await
}
