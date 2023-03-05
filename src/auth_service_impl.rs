use async_trait::async_trait;
use crate::services::{AuthService, Password, UserId, UserName, UserToken};

#[async_trait]
pub trait UserCredentialChecker {
    async fn verify(&self, name: UserName, password: Password) -> Option<UserId>;
}

#[async_trait]
pub trait TokenContainer {
    async fn generate(&self, id: &UserId) -> UserToken;
    async fn verify(&self, token: &UserToken) -> Option<UserId>;
    async fn invalidate(&self, token: &UserToken);
}

struct AuthServiceImpl {
    credentials: Box<dyn UserCredentialChecker + Send + Sync>,
    tokens: Box<dyn TokenContainer + Send + Sync>,
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn auth(&self, primary_id: UserName, password: Password) -> Option<UserToken> {
        let id = self.credentials.verify(primary_id, password).await?;
        let token = self.tokens.generate(&id).await;
        Some(token)
    }

    async fn verify(&self, token: &UserToken) -> Option<UserId> {
        self.tokens.verify(token).await
    }

    async fn invalidate(&self, token: UserToken) {
        self.tokens.invalidate(&token);
    }
}