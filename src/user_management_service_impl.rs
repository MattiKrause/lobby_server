use crate::services::{Password, UserDataRepositories, UserId, UserName};

pub struct UserManagementServiceImpl<UDR> {
    user_repository: Box<UDR>
}

pub enum SetNewPasswordFailure {
    WrongPassword
}

pub enum RemoveAccountFailure {
    WrongPassword
}

impl <UDR> UserManagementServiceImpl<UDR> {
    pub fn new(user_repository: UDR) -> Self {
        Self {
            user_repository: Box::new(UDR),
        }
    }
}

impl <UDR: UserDataRepositories> UserManagementServiceImpl<UDR> {
    async fn set_alias(&self, id: UserId, name: UserName) {
        self.user_repository.set_name_of(&id, &name);
    }
    async fn set_new_password(&self, id:  &UserId, old_password: Password, new_password: Password) -> Result<(), SetNewPasswordFailure> {
        if !self.user_repository.check_password(id, &old_password).await {
            return Err(SetNewPasswordFailure::WrongPassword)
        }
        self.user_repository.set_password(id, &new_password).await;
        Ok(())
    }
    async fn remove_account(&self, id: &UserId, old_password: Password) -> Result<(), RemoveAccountFailure> {
        if !self.user_repository.check_password(id, &old_password) {
            return Err(RemoveAccountFailure::WrongPassword);
        }
        self.user_repository.remove_by_id(id)
    }
}