use std::future::Future;

use thiserror::Error;
use uuid::Uuid;

use super::{roles::Role, users::{EmailAddress, NewUser, Picture, UpdateUser, User, UserDescriptor}};


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

    fn update_session_user(
        &self,
        user_id: Uuid,
        user: UpdateUser,
    ) -> impl Future<Output = Result<Option<UserDescriptor>, UserRepositoryError>> + Send;

    fn update_user(
        &self,
        user_id: Uuid,
        user: UpdateUser,
    ) -> impl Future<Output = Result<Option<UserDescriptor>, UserRepositoryError>> + Send;

    fn delete_user(&self,
        user_id: Uuid,
    ) -> impl Future<Output = Result<Option<Uuid>, UserRepositoryError>> + Send;

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

    fn get_user_descriptors(&self) -> impl Future<Output = Result<Vec<UserDescriptor>, UserRepositoryError>> + Send;

    fn get_roles(&self) -> impl Future<Output = Result<Vec<Role>, UserRepositoryError>> + Send;
}