use aide::transform::TransformOperation;
use schemars::JsonSchema;
use serde::Serialize;
use crate::openapi::extractors::Json;

pub mod docs;
pub mod errors;
pub mod extractors;

pub fn resp_docs_with_exam<Resp: JsonSchema + Default + Serialize>(op: TransformOperation) -> TransformOperation {
  op.description("default_docs")
    .response_with::<200, Json<Resp>, _>(|res| {
      res.example(Resp::default())
    })
}

pub fn resp_docs<Resp: JsonSchema + Serialize>(op: TransformOperation) -> TransformOperation {
  op.description("default_docs")
    .response::<200,Json<Resp>>()
}

pub fn empty_resp_docs(op: TransformOperation) -> TransformOperation {
  op.description("default_docs")
    // .response::<200,Json<Resp>>()
}