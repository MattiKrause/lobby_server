use async_trait::async_trait;
use sqlx::{Database, Executor as DBExecutor, Row};
use uuid::Uuid;
use rust_impl::auth_service_impl::TokenContainer;
use rust_impl::services::{UserId, UserToken};
use crate::sqlx_generic_fn;

pub struct SqlTokenRepository<DB: Database> {
    pool: sqlx::Pool<DB>,
}

impl <DB: Database> SqlTokenRepository<DB> {
    pub fn new(pool: sqlx::Pool<DB>) -> Self {
        Self {
            pool,
        }
    }
}

#[async_trait]
impl <DB: Database> TokenContainer for SqlTokenRepository<DB> where
        for<'c> &'c mut DB::Connection: sqlx::Executor<'c, Database = DB>,
        for <'b> String: sqlx::Type<DB> + sqlx::Encode<'b, DB> + sqlx::Decode<'b, DB>,
        for<'b> <DB as sqlx::database::HasArguments<'b>>::Arguments: sqlx::IntoArguments<'b, DB>,
        usize: sqlx::ColumnIndex<DB::Row>,
        for <'b> &'b  str: sqlx::ColumnIndex<DB::Row>
{

    async fn generate(&self, id: &UserId) -> UserToken {
        generate_impl1(&id, &self.pool).await
    }

    async fn verify(&self, token: &UserToken) -> Option<UserId> {
        verify_impl1(&token, &self.pool).await
    }

    async fn invalidate(&self, token: &UserToken) {
        delete_impl1(token, &self.pool).await
    }
}

sqlx_generic_fn!(generate_impl1(e, id: &UserId) -> UserToken: with 'b String = {
    let uuid = uuid::Uuid::new_v4();
    let query = sqlx::query("INSERT INTO tokens (token, id, creation_date, last_accessed) VALUES (?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
        .bind(uuid.to_string())
        .bind(id.0.to_string());
    e.execute(query)
        .await
        .unwrap();
    UserToken(uuid)
});

sqlx_generic_fn!(verify_impl1(e, token: &UserToken) -> Option<UserId>: with 'b String = {
    let query = sqlx::query("SELECT id FROM tokens WHERE token = ?")
        .bind(token.0.to_string());
    e.fetch_optional(query)
        .await
        .ok().flatten()
        .and_then(|row| row.try_get::<String, _>(0).ok())
        .and_then(|id| Uuid::try_parse(&id).ok())
        .map(UserId)
});

sqlx_generic_fn!(delete_impl1(e, token: &UserToken) : with 'b String = {
    let query = sqlx::query("DELETE FROM tokens WHERE token = ?")
        .bind(token.0.to_string());
    let _ = e.execute(query).await.unwrap();
});