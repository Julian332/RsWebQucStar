use crate::controller::user::NewUser;
use crate::models::{Permission, User};
use crate::openapi::errors::AppError;
use crate::schema::groups::dsl::groups;
use crate::schema::groups_permissions::dsl::groups_permissions;
use crate::schema::groups_permissions::{group_id, permission_id};
use crate::schema::permissions::dsl::permissions;
use crate::schema::users::dsl::users;
use crate::schema::users::username;
use alloy::hex::FromHex;
use alloy::primitives::Address;
use alloy::signers::Signature;
use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, AuthzBackend, UserId};
use chrono::DateTime;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use password_auth::verify_password;
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;
use std::string::ToString;
use std::time::SystemTime;

const LOGIN_MESSAGE: &str = "welcome";
const DEFAULT_TENANTRY: &str = "default";

#[derive(Debug, Clone)]
pub struct AuthBackend {
    db: Pool<ConnectionManager<PgConnection>>,
}

#[cfg(not(feature = "wallet_auth"))]
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

#[cfg(feature = "wallet_auth")]
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct Credentials {
    pub user_addr: String,
    pub signature: String,
    pub next: Option<String>,
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

    #[cfg(not(feature = "wallet_auth"))]
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

    #[cfg(feature = "wallet_auth")]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let signature = Signature::from_str(&creds.signature).expect("wrong signature");
        let recovered_addr = signature
            .recover_address_from_msg(LOGIN_MESSAGE)
            .expect("wrong signature recover");
        let user_addr = Address::from_hex(creds.user_addr.as_str()).expect("wrong address");

        assert_eq!(recovered_addr, user_addr, "not equal ");

        match users
            .filter(username.eq(user_addr.to_string()))
            .select(User::as_select())
            .first(&mut self.db.get().expect("cannot get db"))
            .optional()
        {
            Ok(Some(user)) => Ok(Some(user)),
            Ok(None) => {
                let user = diesel::insert_into(users)
                    .values(NewUser {
                        username: user_addr.to_string(),
                        password: password_auth::generate_hash(creds.signature),
                        group_id: 1,
                        tenantry: DEFAULT_TENANTRY.to_string(),
                        remark: None,
                        create_time: SystemTime::now().into(),
                        create_by: -1,
                        is_delete: false,
                    })
                    .returning(User::as_select())
                    .get_result(&mut self.db.get().expect("cannot get db"))
                    .expect("insert user failed");
                Ok(Some(user))
            }
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
