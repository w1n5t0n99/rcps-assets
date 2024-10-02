use anyhow::Context;
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions, PgSslMode}, PgPool};

use crate::{domain::identityaccess::model::{user_repository::{UserRepository, UserRepositoryError}, users::{User, UserDescriptor, AccessToken, EmailAddress, NewUser, PasswordHash, Picture, Provider, Role}}, settings::DatabaseConfig};


#[derive(Debug, Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(config: &DatabaseConfig) -> anyhow::Result<Self> {
        let ssl_mode = if config.require_ssl == true { PgSslMode::Require } else { PgSslMode::Prefer };
        let pg_connect_options = PgConnectOptions::new()
            .host(&config.host)
            .username(&config.username)
            .password(&config.password)
            .port(config.port)
            .ssl_mode(ssl_mode)
            .database(&config.database_name);

        let pg_pool = PgPoolOptions::new().connect_lazy_with(pg_connect_options);

        Ok(PostgresUserRepository { pool: pg_pool })
    }
}

const UNIQUE_CONSTRAINT_VIOLATION_CODE: &str = "2067";

fn is_unique_constraint_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        if let Some(code) = db_err.code() {
            if code == UNIQUE_CONSTRAINT_VIOLATION_CODE {
                return true;
            }
        }
    }

    false
}


impl UserRepository for PostgresUserRepository {
    async fn add_user(&self, user: NewUser) -> Result<UserDescriptor, UserRepositoryError> {
        let user_descriptor = sqlx::query_as!(
            UserDescriptor,
            r#"
            INSERT INTO users (password_hash, email, email_verified, name, given_name, family_name, role, picture)
            VALUES($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id,  email as "email: EmailAddress", name, role as "role: Role", picture as "picture: Picture"
            "#,
            user.password_hash.to_string(),
            user.email.to_string(),
            user.email_verified,
            user.name.clone(),
            user.given_name.clone(),
            user.family_name.clone(),
            user.role.to_string(),
            user.picture.to_string(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if is_unique_constraint_violation(&e) == true { UserRepositoryError::Duplicate }
            else { UserRepositoryError::Unknown(e.into()) }
        })?;

        Ok(user_descriptor)
    }

    async fn get_user(&self, id: &uuid::Uuid) ->Result<Option<User>, UserRepositoryError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, password_hash as "password_hash: PasswordHash",  email as "email: EmailAddress", email_verified, name, given_name, family_name, role as "role: Role", picture as "picture: Picture", created_at, updated_at
            FROM users
            WHERE id = $1
            "#, 
            id
        )
        .fetch_optional(&self.pool)
        .await
        .context("could not retrieve user from database")?;

        Ok(user)
    }

    async fn get_user_for_auth(&self, email: EmailAddress) ->Result<Option<User>, UserRepositoryError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, password_hash as "password_hash: PasswordHash",  email as "email: EmailAddress", email_verified, name, given_name, family_name, role as "role: Role", picture as "picture: Picture", created_at, updated_at
            FROM users
            WHERE email = $1
            "#, 
            email.to_string()
        )
        .fetch_optional(&self.pool)
        .await
        .context("could not retrieve user from database")?;

        Ok(user)
    }

    async fn get_user_descriptor_for_auth(&self, email: EmailAddress, updated_picture: Option<Picture>) -> Result<Option<UserDescriptor>, UserRepositoryError> {
        let user = match updated_picture {
            Some(picture) => {
                sqlx::query_as!(
                    UserDescriptor,
                    r#"
                    UPDATE users
                        SET picture = $1
                    WHERE email = $2
                    RETURNING id,  email as "email: EmailAddress", name, role as "role: Role", picture as "picture: Picture"
                    "#, 
                    picture.to_string(),
                    email.to_string()
                )
                .fetch_optional(&self.pool)
                .await
                .context("database failure retrieving userdescriptor from database")
            }
            None => {
                sqlx::query_as!(
                    UserDescriptor,
                    r#"
                    SELECT id,  email as "email: EmailAddress", name, role as "role: Role", picture as "picture: Picture"
                    FROM users
                    WHERE email = $1
                    "#, 
                    email.to_string()
                )
                .fetch_optional(&self.pool)
                .await
                .context("database failure retrieving userdescriptor from database")
            }
        }?;

        Ok(user)
    }

    async fn get_user_descriptor(&self, id: &uuid::Uuid) -> Result<Option<UserDescriptor>, UserRepositoryError> {
        let user = sqlx::query_as!(
            UserDescriptor,
            r#"
            SELECT id,  email as "email: EmailAddress", name, role as "role: Role", picture as "picture: Picture"
            FROM users
            WHERE id = $1
            "#, 
            id
        )
        .fetch_optional(&self.pool)
        .await
        .context("database failure retrieving userdescriptor from database")?;

        Ok(user)
    }

    async fn get_all_user_descriptors(&self) -> Result<Vec<UserDescriptor>, UserRepositoryError> {
        let users = sqlx::query_as!(
            UserDescriptor,
            r#"
            SELECT id,  email as "email: EmailAddress", name, role as "role: Role", picture as "picture: Picture"
            FROM users
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .context("database failure retrieving userdescriptors from database")?;

        Ok(users)
    }
}