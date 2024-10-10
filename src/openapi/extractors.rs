use aide::operation::OperationIo;
use axum::response::IntoResponse;
use axum_macros::{FromRequest, FromRequestParts};
use schemars::JsonSchema;
use serde::Serialize;

use crate::openapi::errors::AppError;

#[derive(FromRequest, OperationIo, JsonSchema)]
#[from_request(via(axum_jsonschema::Json), rejection(AppError))]
#[aide(
    input_with = "axum_jsonschema::Json<T>",
    output_with = "axum_jsonschema::Json<T>",
    json_schema
)]
pub struct Json<T>(pub T);

#[derive(OperationIo, JsonSchema, FromRequestParts)]
#[from_request(via(axum_jsonschema::Json), rejection(AppError))]
#[aide(
    input_with = "axum_jsonschema::Json<T>",
    output_with = "axum_jsonschema::Json<T>",
    json_schema
)]
pub struct Parts<T>(pub T);

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}
