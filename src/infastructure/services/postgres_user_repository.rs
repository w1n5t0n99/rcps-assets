use anyhow::Context;
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions, PgSslMode}, PgPool};

use crate::{domain::identityaccess::model::{roles::Role, user_repository::{UserRepository, UserRepositoryError}, users::{EmailAddress, NewUser, PasswordHash, Picture, User, UserDescriptor}}, settings::DatabaseConfig};


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


const UNIQUE_CONSTRAINT_VIOLATION_CODE: &str = "23505";

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
            WITH inserted AS (
                INSERT INTO users (password_hash, email, email_verified, given_name, family_name, role_id, picture)
                VALUES($1, $2, $3, $4, $5, $6, $7)
                RETURNING id, email, given_name, family_name, role_id, picture
            )
            SELECT inserted.id, inserted.email as "email: EmailAddress", inserted.given_name, inserted.family_name, inserted.picture as "picture: Picture", roles.name as role
            FROM inserted INNER JOIN roles ON inserted.role_id = roles.id
            "#,
            user.password_hash.to_string(),
            user.email.to_string(),
            user.email_verified,
            user.given_name.clone(),
            user.family_name.clone(),
            user.role_id,
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
            SELECT users.id, users.password_hash as "password_hash: PasswordHash",  users.email as "email: EmailAddress", users.email_verified, users.given_name, users.family_name, roles.name as role, users.picture as "picture: Picture", users.created_at, users.updated_at
            FROM users INNER JOIN roles ON users.role_id = roles.id
            WHERE users.id = $1
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
            SELECT users.id, users.password_hash as "password_hash: PasswordHash",  users.email as "email: EmailAddress", users.email_verified, users.given_name, users.family_name, roles.name as role, users.picture as "picture: Picture", users.created_at, users.updated_at
            FROM users INNER JOIN roles ON users.role_id = roles.id
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
                    WITH inserted AS (
                        UPDATE users
                            SET picture = $1
                        WHERE email = $2
                        RETURNING id, email, given_name, family_name, role_id, picture
                    )
                    SELECT inserted.id, inserted.email as "email: EmailAddress", inserted.given_name, inserted.family_name, inserted.picture as "picture: Picture", roles.name as role
                    FROM inserted INNER JOIN roles ON inserted.role_id = roles.id
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
                    SELECT users.id,  users.email as "email: EmailAddress", users.given_name, users.family_name, roles.name as role, users.picture as "picture: Picture"
                    FROM users INNER JOIN roles ON users.role_id = roles.id
                    WHERE users.email = $1
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
            SELECT users.id,  users.email as "email: EmailAddress", users.family_name, users.given_name, roles.name as role, users.picture as "picture: Picture"
            FROM users INNER JOIN roles ON users.role_id = roles.id
            WHERE users.id = $1
            "#, 
            id
        )
        .fetch_optional(&self.pool)
        .await
        .context("database failure retrieving userdescriptor from database")?;

        Ok(user)
    }

    async fn get_user_descriptors(&self) -> Result<Vec<UserDescriptor>, UserRepositoryError> {
        let users = sqlx::query_as!(
            UserDescriptor,
            r#"
            SELECT users.id,  users.email as "email: EmailAddress", users.family_name, users.given_name, roles.name as role, users.picture as "picture: Picture"
            FROM users INNER JOIN roles ON users.role_id = roles.id
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .context("database failure retrieving userdescriptors from database")?;

        Ok(users)
    }

    async fn get_roles(&self) -> Result<Vec<Role>, UserRepositoryError> {
        let roles = sqlx::query_as!(
            Role,
            r#"
            SELECT id, name
            FROM roles
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .context("database failure retrieving userdescriptors from database")?;

        Ok(roles)
    }
}