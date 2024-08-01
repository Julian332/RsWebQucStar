use diesel::prelude::*;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::io::{Read, stdin};
use std::sync::Arc;
use aide::axum::ApiRouter;
use aide::openapi::{OpenApi, Tag};
use aide::transform::TransformOpenApi;
use alloy_primitives::Address;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::{Post, TgUser};

mod following_order;
mod tg_user;
mod trading_order;
mod schema;
mod models;
mod errors;
mod extractors;
mod docs;
mod state;

use axum::{routing::{get, post}, http::StatusCode, Json, Router, Extension};
use axum::http::Uri;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::docs::docs_routes;
use crate::errors::AppError;

#[tokio::main]
async fn main() {
  // initialize tracing
  tracing_subscriber::fmt::init();
  let connection_pool = get_connection_pool();

  aide::gen::on_error(|error| {
    println!("{error}");
  });

  aide::gen::extract_schemas(true);
  let mut api = OpenApi::default();


  let app = ApiRouter::new()
    // .nest_api_service("/tg_user", tg_user_routes(connection_pool.clone()))
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


async fn create_user(
  // this argument tells axum to parse the request body
  // as JSON into a `CreateUser` type
  Json(payload): Json<TgUser>,
) -> (StatusCode, Json<TgUser>) {
  // insert your application logic here
  let user = TgUser {
    id: 1337,
    deleted: false,
    create_time: Default::default(),
    update_time: None,
    address: Address::default(),
    private_key: None,
    fee_staged: None,
    fee_received: None,
    parent: None,
  };

  // this will be converted into a JSON response
  // with a status code of `201 Created`
  (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
// #[derive(Deserialize)]
// struct CreateUser {
//   username: String,
// }
// 
// // the output to our `create_user` handler
// #[derive(Serialize)]
// struct User {
//   id: u64,
//   username: String,
// }
// fn main2() {
//   let connection = &mut establish_connection();
// 
//   let mut title2 = String::new();
//   let mut body2 = String::new();
// 
//   println!("What would you like your title to be?");
//   stdin().read_line(&mut title2).unwrap();
//   let title2 = title2.trim_end(); // Remove the trailing newline
// 
//   println!(
//     "\nOk! Let's write {} (Press {} when finished)\n",
//     title2, EOF
//   );
//   stdin().read_to_string(&mut body2).unwrap();
// 
//   let post = create_post(connection, title2, &body2);
//   println!("\nSaved draft {} with id {}", title2, post.id);
// 
// 
// 
//   use self::schema::posts::dsl::*;
// 
//   let connection = &mut establish_connection();
//   let results = posts
//     .filter(published.eq(true))
//     .limit(5)
//     .select(Post::as_select())
//     .load(connection)
//     .expect("Error loading posts");
// 
//   println!("Displaying {} posts", results.len());
//   for post in results {
//     println!("{}", post.title);
//     println!("-----------\n");
//     println!("{}", post.body);
//   }
// }
// pub fn establish_connection() -> PgConnection {
//   dotenv().ok();
// 
//   let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//   PgConnection::establish(&database_url)
//     .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
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

// use self::models::{NewPost};

// pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
//   use crate::schema::posts;
// 
//   let new_post = NewPost { title, body };
// 
//   diesel::insert_into(posts::table)
//     .values(&new_post)
//     .returning(Post::as_returning())
//     .get_result(conn)
//     .expect("Error saving new post")
// }

// #[cfg(not(windows))]
// const EOF: &str = "CTRL+D";
// 
// #[cfg(windows)]
// const EOF: &str = "CTRL+Z";

