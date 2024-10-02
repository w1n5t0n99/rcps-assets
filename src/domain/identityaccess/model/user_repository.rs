use std::future::Future;

use thiserror::Error;

use super::users::{EmailAddress, NewUser, Picture, User, UserDescriptor};


#[derive(Error, Debug)]
pub enum UserRepositoryError {
    #[error("user already exists")]
    Duplicate,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

pub trait UserRepository: Send + Sync + Clone + 'static {

    fn add_user(
        &self,
        user: NewUser,
    ) -> impl Future<Output = Result<UserDescriptor, UserRepositoryError>> + Send;

    fn get_user(
        &self,
        id: &uuid::Uuid,
    ) -> impl Future<Output = Result<Option<User>, UserRepositoryError>> + Send;

    fn get_user_for_auth(
        &self,
        email: EmailAddress,
    ) -> impl Future<Output = Result<Option<User>, UserRepositoryError>> + Send;

    fn get_user_descriptor(
        &self,
        id: &uuid::Uuid,
    ) -> impl Future<Output = Result<Option<UserDescriptor>, UserRepositoryError>> + Send;

    fn get_user_descriptor_for_auth(
        &self,
        email: EmailAddress,
        updated_picture: Option<Picture>
    ) -> impl Future<Output = Result<Option<UserDescriptor>, UserRepositoryError>> + Send;

    fn get_all_user_descriptors(&self) -> impl Future<Output = Result<Vec<UserDescriptor>, UserRepositoryError>> + Send;

}