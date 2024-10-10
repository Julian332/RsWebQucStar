use crate::models::{Permission, User};
use crate::openapi::errors::AppError;
use crate::schema::groups::dsl::groups;
use crate::schema::groups_permissions::dsl::groups_permissions;
use crate::schema::groups_permissions::{group_id, permission_id};
use crate::schema::permissions::dsl::permissions;
use crate::schema::users::dsl::users;
use crate::schema::users::username;
use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, AuthzBackend, UserId};
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use password_auth::verify_password;
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct AuthBackend {
    db: Pool<ConnectionManager<PgConnection>>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

// impl From<&str> for Permission {
//     fn from(name: &str) -> Self {
//         Permission {
//             name: name.to_string(),
//         }
//     }
// }
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
impl AuthBackend {
    pub(crate) fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { db: pool }
    }
}

#[async_trait]
impl AuthnBackend for AuthBackend {
    type User = User;
    type Credentials = Credentials;
    type Error = AppError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        match users
            .filter(username.eq(creds.username))
            .select(User::as_select())
            .first(&mut self.db.get().expect("cannot get db"))
        {
            Ok(user) => verify_password(creds.password, &user.password)
                .map_err(|e| AppError {
                    error: format!("{}", e),
                    error_id: Default::default(),
                    status: Default::default(),
                    error_details: None,
                })
                .map(|_| Some(user)),
            Err(e) => Err(e.into()),
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        match users
            .find(user_id)
            .select(User::as_select())
            .first(&mut self.db.get().expect("cannot get db"))
        {
            Ok(user) => Ok(Some(user)),
            Err(e) => Err(e.into()),
        }
    }
}

#[async_trait]
impl AuthzBackend for AuthBackend {
    type Permission = Permission;

    async fn get_group_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        let conn = &mut self.db.get().expect("cannot get db");
        match users
            .inner_join(groups::table())
            .inner_join(groups_permissions.on(group_id.eq(crate::schema::groups::id)))
            .inner_join(permissions.on(permission_id.eq(crate::schema::permissions::id)))
            .filter(crate::schema::users::id.eq(user.id))
            .select(Permission::as_select())
            .load(conn)
        {
            Ok(res) => Ok(res.into_iter().collect()),
            Err(e) => Err(e.into()),
        }
    }
}
