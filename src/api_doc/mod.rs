use aide::openapi::Tag;
use aide::transform::{TransformOpenApi, TransformOperation};
use alloy::transports::http::reqwest::StatusCode;
use axum::http::Uri;
use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

use crate::api_doc::errors::AppError;
use crate::api_doc::extractors::Json;

pub mod docs;
pub mod errors;
pub mod extractors;

pub fn default_resp_docs_with_exam<Resp: JsonSchema + Default + Serialize>(
    op: TransformOperation,
) -> TransformOperation {
    op.description("default_docs")
        .response_with::<200, Json<Resp>, _>(|res| res.example(Resp::default()))
}

pub fn default_resp_docs<Resp: JsonSchema + Serialize>(
    op: TransformOperation,
) -> TransformOperation {
    op.description("default_docs").response::<200, Json<Resp>>()
}

pub fn empty_resp_docs(op: TransformOperation) -> TransformOperation {
    op.description("default_docs")
    // .response::<200,Json<Resp>>()
}
pub fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("RsWebQuicStar")
        .summary("RsWebQuicStar")
        .description(include_str!("../api-doc.md"))
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
        .default_response_with::<axum::Json<AppError>, _>(|res| {
            res.example(AppError {
                error: "some error happened".to_string(),
                error_details: None,
                error_id: Uuid::nil(),
                // This is not visible.
                status: StatusCode::IM_A_TEAPOT,
            })
        })
}

pub async fn root() -> &'static str {
    "Hello, World!"
}
pub async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
