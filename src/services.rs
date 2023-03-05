use async_trait::async_trait;
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct UserToken(pub Uuid);
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct UserId(pub Uuid);
pub struct Password(String);
pub struct UserName(pub String);

pub enum RegistrationError {
    InvalidEmail(String),
    EmailAlreadyRegistered(String),
    InvalidPassword,
    AliasEmpty,
}

pub enum FinaliseRegistrationError {
    CodeNotFound
}

#[async_trait]
pub trait AuthService {
    async fn auth(&self, primary_id: UserName, password: Password) -> Option<UserToken>;
    async fn verify(&self, token: &UserToken) -> Option<UserId>;
    async fn invalidate(&self, token:  UserToken);
}

#[async_trait]
pub trait RegistrationService {
    async fn register_initial(&self, primary_id: String, alias: String, password: Password) -> Result<(), RegistrationError>;
    async fn finalise_registration(&self, code: String) -> Result<(), FinaliseRegistrationError>;
}

#[async_trait]
pub trait   UserDataRepositories {
    async fn set_name_of(&self, id: &UserId, name: &UserName);
    async fn check_password(&self, id: &UserId, password: &Password) -> bool;
    async fn remove_by_id(&self, id: &UserId);
    async fn set_password(&self, id: &UserId, password: &Password);
}