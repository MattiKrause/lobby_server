use std::str::FromStr;
use std::sync::Arc;
use axum::body::HttpBody;
use axum::extract::Query;
use axum::Router;
use axum::routing::MethodRouter;
use sqlx::{Column, Executor, Row, SqlitePool};
use tokio::main as tokio_main;
use uuid::Uuid;
use rust_impl::auth_service_impl::TokenContainer;
use rust_impl::services::UserId;
use serde::{Deserialize, Serialize};
use rust_impl::user_management_service_impl::UserManagementServiceImpl;

#[derive(Serialize, Deserialize)]
struct UsernamePassword {
    username: String,
    password: String
}

#[tokio_main]
async fn main() -> Result<(), String> {
    let user_db = std::env::var("USER_DATABASE").map_err(|_| String::from("no user database url  \"USER_DATABASE\" given"))?;
    let db = sqlx::PgPool::connect(&user_db).await.map_err(|db| format!("failed to connect to user db: {db}"))?;
    let uuid1 = "00010001-0001-0001-0001-000100010001";
    let uuid2 = "a0ee-bc99-9c0b-4ef8-bb6d-6bb9-bd38-0a11";

    let token_container = sql_container_impl::sql_token_repository::SqlTokenRepository::new(db.clone());
    let user_container = sql_container_impl::postgres_user_repository::PostgresUserRepository::new(db.clone());
    let user_container = Box::new(user_container);
    let user_management_service = UserManagementServiceImpl::new(user_container);

    Router::new()
        .route("/request_token", MethodRouter::new().get(request_token).with_state(user_management_service));

    container.invalidate(&token).await;
    rust_impl::auth_service_impl::
    dbg!(db.fetch_all("SELECT * FROM tokens").await.unwrap().into_iter().map(|row| row.columns().iter().map(|it| row.get::<String,_>(it.ordinal())).collect::<Vec<_>>()).collect::<Vec<_>>());
    assert!(container.verify(&token).await.is_none());

    Ok(())
}

async fn request_token(service: Arc<UserManagementServiceImpl>, query: Query<UsernamePassword>) {

}
