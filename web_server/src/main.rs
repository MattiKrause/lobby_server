use std::str::FromStr;
use sqlx::{Column, Executor, Row, SqlitePool};
use tokio::main as tokio_main;
use uuid::Uuid;
use rust_impl::auth_service_impl::TokenContainer;
use rust_impl::services::UserId;

#[tokio_main]
async fn main() {
    println!("Hello, world!, achuchuicsdochsdvnkdsnc");
    let db = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    db.execute("CREATE TABLE users ( id UUID PRIMARY KEY, email TEXT NOT NULL, name TEXT NOT NULL, password TEXT NOT NULL);").await.unwrap();
    db.execute("CREATE TABLE tokens (token STRING PRIMARY KEY, id UUID, creation_date DATETIME NOT NULL, last_accessed DATETIME)").await.unwrap();

    let uuid1 = "00010001-0001-0001-0001-000100010001";
    let uuid2 = "a0ee-bc99-9c0b-4ef8-bb6d-6bb9-bd38-0a11";
    db.execute(sqlx::query("INSERT INTO users (id, email, name, password) VALUES (?, \"example@examp.com\", \"Name\", \"Password\")").bind(uuid1)).await.unwrap();


    
    let results = sqlx::query("SELECT name FROM users").fetch_all(&db).await.unwrap();
    dbg!(results.into_iter().map(|it| it.get::<String, _>(0)).collect::<Vec<_>>());
    let container= sql_container_impl::sql_token_repository::SqlTokenRepository::new(db.clone());
    let id1 = UserId(Uuid::from_str(uuid1).unwrap());
    let token = container.generate(&id1).await;
    assert_eq!(container.verify(&token).await, Some(id1));
    container.invalidate(&token).await;
    rust_impl::auth_service_impl::
    dbg!(db.fetch_all("SELECT * FROM tokens").await.unwrap().into_iter().map(|row| row.columns().iter().map(|it| row.get::<String,_>(it.ordinal())).collect::<Vec<_>>()).collect::<Vec<_>>());
    assert!(container.verify(&token).await.is_none());

}
