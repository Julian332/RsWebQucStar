use axum::{http::StatusCode, response::IntoResponse};
use axum_jsonschema::JsonSchemaRejection;
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::{json, Value};
use uuid::Uuid;

/// A default error response for most API errors.
#[derive(Debug, Serialize, JsonSchema)]
pub struct AppError {
    /// An error message.
    pub error: String,
    /// A unique error ID.
    pub error_id: Uuid,
    #[serde(skip)]
    pub status: StatusCode,
    /// Optional Additional error details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Value>,
}

impl AppError {
    pub fn new(error: &str) -> Self {
        Self {
            error: error.to_string(),
            error_id: Uuid::new_v4(),
            status: StatusCode::BAD_REQUEST,
            error_details: None,
        }
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn with_details(mut self, details: Value) -> Self {
        self.error_details = Some(details);
        self
    }
}
impl From<JsonSchemaRejection> for AppError {
    fn from(rejection: JsonSchemaRejection) -> Self {
        match rejection {
            JsonSchemaRejection::Json(j) => Self::new(&j.to_string()),
            JsonSchemaRejection::Serde(_) => Self::new("invalid request"),
            JsonSchemaRejection::Schema(s) => {
                Self::new("invalid request").with_details(json!({ "schema_validation": s }))
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let mut res = axum::Json(self).into_response();
        *res.status_mut() = status;
        res
    }
}
