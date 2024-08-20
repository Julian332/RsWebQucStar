use std::env;
use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::openapi::OpenApi;
use axum::Extension;
use axum_login::AuthManagerLayerBuilder;
use axum_login::tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use axum_login::tower_sessions::cookie::time::Duration;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::{Deserialize, Serialize};

use crate::controller::analysis::listen_and_send;
use crate::openapi::{api_docs, fallback};
use openapi::docs::docs_routes;
use crate::api_auth::Backend;

mod openapi;
mod domain;
mod controller;
pub mod schema;
pub mod apis;
pub mod contract;
pub mod models;
mod api_auth;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();
  set_env();

  let connection_pool = get_connection_pool();

  let session_store = MemoryStore::default();
  let session_layer = SessionManagerLayer::new(session_store)
    .with_secure(false)
    .with_expiry(Expiry::OnInactivity(Duration::days(1)));
  
  let backend = Backend::new(connection_pool.clone());
  let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();




  aide::gen::on_error(|error| {
    println!("{error}");
  });
  aide::gen::extract_schemas(true);
  let mut api = OpenApi::default();


  let app = ApiRouter::new()
    .nest_api_service("/tg_users", controller::tg_user::web_routes(connection_pool.clone()))
    .nest_api_service("/analysis", controller::analysis::analysis_routes(connection_pool.clone()))
    .nest_api_service("/trading_order", controller::trading_order::trading_order_routes(connection_pool.clone()))
    // .nest_api_service("/trading_order", trading_order_routes(connection_pool.clone()))
    .nest_api_service("/docs", docs_routes())

    .finish_api_with(&mut api, api_docs)
    .layer(Extension(Arc::new(api)))
    .fallback(fallback)
    .with_state(connection_pool.clone());
  tokio::spawn(listen_and_send(connection_pool.clone()));

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind(
    format!("0.0.0.0:{}", env::var("SERVER_PORT").unwrap_or("4090".to_string()))).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

fn set_env() {
  let profile = get_build_profile_name();
  tracing::info!("profile :{} is active",profile);
  match profile.as_str() {
    "release" => { dotenvy::from_filename("env_prod.env").ok(); }
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
