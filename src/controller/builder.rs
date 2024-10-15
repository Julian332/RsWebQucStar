// use chrono::{DateTime, Utc};
// use derive_builder::Builder;
// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};
//
// #[derive(Builder)]
// #[builder(derive(Deserialize, Serialize, JsonSchema))]
// pub struct NewUser {
//     pub username: String,
//     pub password: String,
//     pub group_id: i64,
//     pub tenantry: String,
//     pub remark: Option<String>,
//     pub create_time: DateTime<Utc>,
//     pub create_by: i64,
//     pub is_delete: bool,
// }
// // #[allow(clippy::all)]
// // #[derive(Clone, Deserialize, Serialize, JsonSchema)]
// // pub struct NewUserBuilder {
// //     username: ::derive_builder::export::core::option::Option<String>,
// //     password: ::derive_builder::export::core::option::Option<String>,
// //     group_id: ::derive_builder::export::core::option::Option<i64>,
// //     tenantry: ::derive_builder::export::core::option::Option<String>,
// //     remark: ::derive_builder::export::core::option::Option<Option<String>>,
// //     create_time: ::derive_builder::export::core::option::Option<DateTime<Utc>>,
// //     create_by: ::derive_builder::export::core::option::Option<i64>,
// //     is_delete: ::derive_builder::export::core::option::Option<bool>,
// // }
// // #[allow(clippy::all)]
// // #[allow(dead_code)]
// // impl NewUserBuilder {
// //     #[doc = r" Create an empty builder, with all fields set to `None` or `PhantomData`."]
// //     fn create_empty() -> Self { Self { username: ::derive_builder::export::core::default::Default::default(), password: ::derive_builder::export::core::default::Default::default(), group_id: ::derive_builder::export::core::default::Default::default(), tenantry: ::derive_builder::export::core::default::Default::default(), remark: ::derive_builder::export::core::default::Default::default(), create_time: ::derive_builder::export::core::default::Default::default(), create_by: ::derive_builder::export::core::default::Default::default(), is_delete: ::derive_builder::export::core::default::Default::default() } }
// // }
// // impl ::derive_builder::export::core::default::Default for NewUserBuilder {
// //     fn default() -> Self { Self::create_empty() }
// // }
// // #[doc = "Error type for NewUserBuilder"]
// // #[derive(Debug)]
// // #[non_exhaustive]
// // pub enum NewUserBuilderError {
// //     #[doc = r" Uninitialized field"] UninitializedField(&'static str),
// //     #[doc = r" Custom validation error"] ValidationError(::derive_builder::export::core::string::String),
// // }
// // impl ::derive_builder::export::core::convert::From<::derive_builder::UninitializedFieldError> for NewUserBuilderError {
// //     fn from(s: ::derive_builder::UninitializedFieldError) -> Self { Self::UninitializedField(s.field_name()) }
// // }
// // impl ::derive_builder::export::core::convert::From<::derive_builder::export::core::string::String> for NewUserBuilderError {
// //     fn from(s: ::derive_builder::export::core::string::String) -> Self { Self::ValidationError(s) }
// // }
// // impl ::derive_builder::export::core::fmt::Display for NewUserBuilderError {
// //     fn fmt(&self, f: &mut ::derive_builder::export::core::fmt::Formatter) -> ::derive_builder::export::core::fmt::Result {
// //         match self {
// //             Self::UninitializedField(ref field) => write!(f, "`{}` must be initialized", field),
// //             Self::ValidationError(ref error) => write!(f, "{}", error),
// //         }
// //     }
// // }
// // impl std::error::Error for NewUserBuilderError {}
