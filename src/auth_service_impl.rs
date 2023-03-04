use std::ops::Add;
use std::time::{Duration, Instant, SystemTime};
use async_trait::async_trait;
use base64::Engine;
use crate::services::{AuthService, Password, UserId, UserName, UserToken};

#[async_trait]
pub trait CredentialChecker {
    async fn verify(&self, name: UserName, password: Password) -> Option<UserId>;
}

#[async_trait]
pub trait TokenContainer {
    async fn generate(&self, id: &UserId) -> UserToken;
    async fn verify(&self, token: &UserToken) -> Option<UserId>;
    async fn invalidate(&self, token: &UserToken);
}

struct AuthServiceImpl {
    credentials: Box<dyn CredentialChecker + Send + Sync>,
    tokens: Box<dyn TokenContainer + Send + Sync>,
}

static BASE64_ENGINE: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
    &base64::alphabet::URL_SAFE,
    base64::engine::GeneralPurposeConfig::new().with_encode_padding(false)
);

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