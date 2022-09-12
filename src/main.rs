#[macro_use]
extern crate lazy_static;
extern crate dotenv;

mod entities;
mod role_service;
mod team_service;
mod user_service;

use dotenv::dotenv;
use poem::{
    error::NotFoundError, http::StatusCode, listener::TcpListener, EndpointExt, Response, Route,
    Server,
};
use poem_openapi::OpenApiService;
use role_service::controller::UserRoleRouter;
use sea_orm::*;
use std::env;
use team_service::controller::TeamRouter;
use tokio::sync::OnceCell;
use tracing::log::warn;

use user_service::controller::UserRouter;

lazy_static! {
    static ref DATABASE: OnceCell<DatabaseConnection> = OnceCell::new();
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

    let api_service = OpenApiService::new(
        (UserRouter, UserRoleRouter, TeamRouter),
        "Truck Billing Service",
        "1.0",
    )
    .server(&bind_addr);

    let ui = api_service.swagger_ui();

    let app = Route::new()
        .nest("/", api_service)
        .nest("/docs", ui)
        .catch_error(|_err: NotFoundError| async move {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("custom not found")
        });
    Server::new(TcpListener::bind(&bind_addr)).run(app).await
}
