use std::collections::HashSet;
use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, AuthzBackend, UserId};
use chrono::{DateTime, Utc};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use crate::openapi::errors::AppError;

#[derive(Debug, Clone)]
pub struct Backend {
  db: Pool<ConnectionManager<PgConnection>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
  pub id: i64,
  pub deleted: bool,
  pub create_time: DateTime<Utc>,
  pub update_time: Option<DateTime<Utc>>,
  pub username: String,
  pub group_id: i64,
  password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
  pub username: String,
  pub password: String,
  pub next: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, )]
pub struct Permission {
  pub name: String,
}

impl From<&str> for Permission {
  fn from(name: &str) -> Self {
    Permission {
      name: name.to_string(),
    }
  }
}
impl AuthUser for User {
  type Id = i64;

  fn id(&self) -> Self::Id {
    self.id
  }

  fn session_auth_hash(&self) -> &[u8] {
    self.password.as_bytes() // We use the password hash as the auth
    // hash--what this means
    // is when the user changes their password the
    // auth session becomes invalid.
  }
}

impl std::fmt::Debug for User {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("User")
      .field("id", &self.id)
      .field("username", &self.username)
      .field("password", &"[redacted]")
      .finish()
  }
}
impl Backend {
  pub(crate) fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
    Self {
      db: pool
    }
  }
}

#[async_trait]
impl AuthnBackend for Backend {
  type User = User;
  type Credentials = Credentials;
  type Error = AppError;

  async fn authenticate(
    &self,
    creds: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    let user: Option<Self::User> = sqlx::query_as("select * from users where username = ? ")
      .bind(creds.username)
      .fetch_optional(&self.db)
      .await?;

    // Verifying the password is blocking and potentially slow, so we'll do so via
    // `spawn_blocking`.
    tokio::task::spawn_blocking(|| {
      // We're using password-based authentication: this works by comparing our form
      // input with an argon2 password hash.
      Ok(user.filter(|user| verify_password(creds.password, &user.password).is_ok()))
    })
      .await?
  }

  async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
    let user = sqlx::query_as("select * from users where id = ?")
      .bind(user_id)
      .fetch_optional(&self.db)
      .await?;

    Ok(user)
  }
}

#[async_trait]
impl AuthzBackend for Backend {
  type Permission = Permission;

  async fn get_group_permissions(
    &self,
    user: &Self::User,
  ) -> Result<HashSet<Self::Permission>, Self::Error> {
    let permissions: Vec<Self::Permission> = sqlx::query_as(
      r#"
            select distinct permissions.name
            from users
            join users_groups on users.id = users_groups.user_id
            join groups_permissions on users_groups.group_id = groups_permissions.group_id
            join permissions on groups_permissions.permission_id = permissions.id
            where users.id = ?
            "#,
    )
      .bind(user.id)
      .fetch_all(&self.db)
      .await?;

    Ok(permissions.into_iter().collect())
  }
}