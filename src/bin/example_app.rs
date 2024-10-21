use std::env;
use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::openapi::OpenApi;
use axum::Extension;

use web_quick_start::api_doc::docs::docs_routes;
use web_quick_start::api_doc::{api_docs, fallback};
use web_quick_start::{
    api_auth, controller, get_auth_layer, get_connection_pool, set_api_doc, set_env, set_scheduler,
    GLOBAL_CONNECTION_POOL,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    set_env();
    set_api_doc();

    let connection_pool = get_connection_pool();
    GLOBAL_CONNECTION_POOL.get_or_init(|| connection_pool.clone());
    set_scheduler().await;
    let mut api = OpenApi::default();

    let app = ApiRouter::new()
        .nest_api_service("/auth", api_auth::router::router())
        .nest_api_service(
            "/users",
            controller::user::web_routes(connection_pool.clone()),
        )
        .nest_api_service(
            "/groups",
            controller::group::web_routes(connection_pool.clone()),
        )
        .nest_api_service(
            "/permissions",
            controller::permission::web_routes(connection_pool.clone()),
        )
        .nest_api_service(
            "/group_permission",
            crate::controller::group_permission::web_routes(connection_pool.clone()),
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
