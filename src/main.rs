use std::env;
use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::openapi::{OpenApi, Tag};
use aide::transform::TransformOpenApi;
use axum::{Extension, http::StatusCode, Json};
use axum::http::Uri;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use openapi::docs::docs_routes;
use openapi::errors::AppError;

mod openapi;
mod domain;
mod controller;
pub mod schema;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();
  let connection_pool = get_connection_pool();

  aide::gen::on_error(|error| {
    println!("{error}");
  });
  aide::gen::extract_schemas(true);
  let mut api = OpenApi::default();


  let app = ApiRouter::new()
    .nest_api_service("/tg_users",controller::tg_user::tg_user_routes(connection_pool.clone()))
    .nest_api_service("/trading_order",controller::trading_order::trading_order_routes(connection_pool.clone()))
    // .nest_api_service("/trading_order", trading_order_routes(connection_pool.clone()))
    .nest_api_service("/docs", docs_routes())

    .finish_api_with(&mut api, api_docs)
    .layer(Extension(Arc::new(api)))
    .with_state(connection_pool);


  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
  api.title("Aide axum Open API")
    .summary("An example Todo application")
    .description(include_str!("README.md"))
    .tag(Tag {
      name: "todo".into(),
      description: Some("Todo Management".into()),
      ..Default::default()
    })
    .security_scheme(
      "ApiKey",
      aide::openapi::SecurityScheme::ApiKey {
        location: aide::openapi::ApiKeyLocation::Header,
        name: "X-Auth-Key".into(),
        description: Some("A key that is ignored.".into()),
        extensions: Default::default(),
      },
    )
    .default_response_with::<Json<AppError>, _>(|res| {
      res.example(AppError {
        error: "some error happened".to_string(),
        error_details: None,
        error_id: Uuid::nil(),
        // This is not visible.
        status: StatusCode::IM_A_TEAPOT,
      })
    })
}

// basic handler that responds with a static string
async fn root() -> &'static str {
  "Hello, World!"
}
async fn fallback(uri: Uri) -> (StatusCode, String) {
  (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}


// async fn create_user(
//   // this argument tells axum to parse the request body
//   // as JSON into a `CreateUser` type
//   Json(payload): Json<TgUser>,
// ) -> (StatusCode, Json<TgUser>) {
//   // insert your application logic here
//   let user = TgUser {
//     id: 1337,
//     deleted: false,
//     create_time: Default::default(),
//     update_time: None,
//     address: String::default(),
//     private_key: None,
//     fee_staged: None,
//     fee_received: None,
//     parent: None,
//   };
// 
//   // this will be converted into a JSON response
//   // with a status code of `201 Created`
//   (StatusCode::CREATED, Json(user))
// }


pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
  dotenv().ok();

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



