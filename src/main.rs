use std::env;
use std::sync::{Arc, OnceLock};

use crate::api_auth::login_impl::AuthBackend;
use crate::openapi::{api_docs, fallback};
use aide::axum::ApiRouter;
use aide::openapi::OpenApi;
use axum::Extension;
use axum_login::tower_sessions::cookie::time::Duration;
use axum_login::tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use openapi::docs::docs_routes;
use tokio_cron_scheduler::{Job, JobScheduler};

mod api_auth;
pub mod apis;
pub mod contract;
mod controller;
mod domain;
pub mod models;
mod openapi;
pub mod schema;
mod task;

static STATIC_CONNECTION_POOL: OnceLock<Pool<ConnectionManager<PgConnection>>> = OnceLock::new();

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    set_env();
    set_api_doc();

    let connection_pool = get_connection_pool();
    STATIC_CONNECTION_POOL.get_or_init(|| connection_pool.clone());
    set_scheduler().await;
    let mut api = OpenApi::default();

    let app = ApiRouter::new()
        .nest_api_service("/auth", api_auth::router::router())
        .nest_api_service(
            "/users",
            controller::user::web_routes(connection_pool.clone()),
        )
        .nest_api_service("/docs", docs_routes())
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)))
        .fallback(fallback)
        .with_state(connection_pool.clone())
        .merge(api_auth::router::router())
        .layer(get_auth_layer(connection_pool.clone()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!(
        "0.0.0.0:{}",
        env::var("SERVER_PORT").unwrap_or("4090".to_string())
    ))
    .await
    .expect("Can not bind to port");
    axum::serve(listener, app)
        .await
        .expect("Can not run server");
}

async fn set_scheduler() {
    let sched = JobScheduler::new()
        .await
        .expect("cannot create jobs scheduler");
    sched
        .add(
            Job::new("1/10 * * * * *", |_uuid, _l| {
                println!("I run every 10 seconds");
            })
            .expect("cannot create job"),
        )
        .await
        .expect("cannot join job");

    sched.start().await.expect("cannot start jobs scheduler");
}

fn set_api_doc() {
    aide::gen::on_error(|error| {
        println!("{error}");
    });
    aide::gen::extract_schemas(true);
}

fn get_auth_layer(
    connection_pool: Pool<ConnectionManager<PgConnection>>,
) -> AuthManagerLayer<AuthBackend, MemoryStore> {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    let backend = AuthBackend::new(connection_pool);
    AuthManagerLayerBuilder::new(backend, session_layer).build()
}

fn set_env() {
    let profile = get_build_profile_name();
    tracing::info!("profile :{} is active", profile);
    match profile.as_str() {
        "release" => {
            dotenvy::from_filename("env_prod.env").ok();
        }
        _ => {
            dotenvy::from_filename(".env").ok();
        }
    }
}

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .max_size(10)
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

fn get_build_profile_name() -> String {
    // The profile name is always the 3rd last part of the path (with 1 based indexing).
    // e.g. /code/core/target/cli/build/my-build-info-9f91ba6f99d7a061/out
    std::env!("OUT_DIR")
        .split(std::path::MAIN_SEPARATOR)
        .nth_back(3)
        .unwrap_or_else(|| "unknown")
        .to_string()
}
