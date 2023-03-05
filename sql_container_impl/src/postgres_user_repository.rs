use std::future::Future;
use async_trait::async_trait;
use sqlx::{Executor, PgPool};
use rust_impl::auth_service_impl::UserCredentialChecker;
use rust_impl::services::{Password, UserDataRepositories, UserId, UserName};

pub struct PostgresUserRepository {
    pool: PgPool
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresUserRepository {
            pool,
        }
    }
}

#[async_trait]
impl UserDataRepositories for PostgresUserRepository {
    async fn set_name_of(&self, id: &UserId, name: &UserName) {
        let query = sqlx::query("UPDATE users SET user_name = ? WHERE user_id = ?")
            .bind(id.0.to_string())
            .bind(&name.0);
        self.pool.execute(query).await.unwrap();
    }

    async fn check_password(&self, id: &UserId, password: &Password) -> bool {
        let query = sqlx::query("SELECT user_id FROM users WHERE user_id = ? AND user_password = crypt(?, user_password)")
            .bind(id.0.to_string())
            .bind(password);
        self.pool.fetch_optional(query).await.unwrap().is_some()
    }

    async fn remove_by_id(&self, id: &Userid) {
        let query = sqlx::query("DELETE FROM users WHERE user_id = ?")
            .bind(id.0.to_string());
        self.pool.execute(query).await.unwrap();
    }

    async fn set_password(&self, id: &UserId, password: &Password) {
        let query = sqlx::query("UPDATE users SET user_password = crypt(?, gen_salt('bf')) WHERE user_id = ?")
            .bind(password)
            .bind(id.0.to_string());
        self.pool.execute(query).await.unwrap();
    }
}

#[async_trait]
impl UserCredentialChecker for PostgresUserRepository {
    fn verify(&self, name: UserName, password: Password) -> Option<UserId> {
        let query = sqlx::query("SELECT user_id FROM users WHERE user_email = ? AND user_password = crypt(?, user_password)")
            .bind(&name.0)
            .bind(&password);

    }
}