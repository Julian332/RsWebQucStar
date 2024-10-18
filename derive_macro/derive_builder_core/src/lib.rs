//! Internal helper library for the `derive_builder` crate.
//!
//! **Important Note**:
//!
//! * You are probably looking for the [`derive_builder`] crate,
//!   which wraps this crate and is much more ergonomic to use.
//!
//! ## Purpose
//!
//! This is an internal helper library of [`derive_builder`], which allows for
//! all the logic of builder creation to be decoupled from the proc-macro entry
//! point.
//!
//!
//! [`derive_builder`]: https://!crates.io/crates/derive_builder
//! [`derive_builder_core`]: https://!crates.io/crates/derive_builder_core

#![deny(warnings, missing_docs)]
#![cfg_attr(test, recursion_limit = "100")]

#[macro_use]
extern crate darling;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

mod block;
mod build_method;
mod builder;
mod builder_field;
mod change_span;
mod default_expression;
mod doc_comment;
mod initializer;
mod macro_options;
mod options;
mod setter;

pub(crate) use block::BlockContents;
pub(crate) use build_method::BuildMethod;
pub(crate) use builder::Builder;
pub(crate) use builder_field::{BuilderField, BuilderFieldType};
pub(crate) use change_span::change_span;
use darling::FromDeriveInput;
pub(crate) use default_expression::DefaultExpression;
pub(crate) use doc_comment::doc_comment_from;
pub(crate) use initializer::{FieldConversion, Initializer};
pub(crate) use options::{BuilderPattern, Each};
use quote::TokenStreamExt;
pub(crate) use setter::Setter;

const DEFAULT_STRUCT_NAME: &str = "__default";

/// Derive a builder for a struct
pub fn builder_for_struct(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let opts = match macro_options::Options::from_derive_input(&ast) {
        Ok(val) => val,
        Err(err) => {
            return err.write_errors();
        }
    };
    let model = opts.ident.clone();
    let schema = format_ident!("{}s", model.to_string().to_lowercase());
    let new_model = format_ident!("New{}", model);
    let mut builder = opts.as_builder();
    let builder_ident = opts.builder_ident();
    // let  build_fn = opts.as_build_method();

    // builder.doc_comment(format!(
    //     include_str!("doc_tpl/builder_struct.md"),
    //     struct_name = ast.ident
    // ));
    // build_fn.doc_comment(format!(
    //     include_str!("doc_tpl/builder_method.md"),
    //     struct_name = ast.ident
    // ));

    let mut filters = vec![];
    for field in opts.fields() {
        let ident = field.field_ident();
        filters.push(quote!(
                    if let Some(filter_param) = filter.#ident {
                        match filter_param.compare {
                            Compare::NotEqual => {
                                statement = statement.filter(crate::schema::#schema::#ident.ne(filter_param.compare_value.clone()));
                                count_statement = count_statement.filter(crate::schema::#schema::#ident.ne(filter_param.compare_value));
                            }
                            Compare::Equal => {
                                statement = statement.filter(crate::schema::#schema::#ident.eq(filter_param.compare_value.clone()));
                                count_statement = count_statement.filter(crate::schema::#schema::#ident.eq(filter_param.compare_value));
                            }
                            Compare::Greater => {
                                statement = statement.filter(crate::schema::#schema::#ident.gt(filter_param.compare_value.clone()));
                                count_statement = count_statement.filter(crate::schema::#schema::#ident.gt(filter_param.compare_value));
                            }
                            Compare::GreaterAndEqual => {
                                statement = statement.filter(crate::schema::#schema::#ident.ge(filter_param.compare_value.clone()));
                                count_statement = count_statement.filter(crate::schema::#schema::#ident.ge(filter_param.compare_value));
                            }
                            Compare::Less => {
                                statement = statement.filter(crate::schema::#schema::#ident.lt(filter_param.compare_value.clone()));
                                count_statement = count_statement.filter(crate::schema::#schema::#ident.lt(filter_param.compare_value));
                            }
                            Compare::LessAndEqual => {
                                statement = statement.filter(crate::schema::#schema::#ident.le(filter_param.compare_value.clone()));
                                count_statement = count_statement.filter(crate::schema::#schema::#ident.le(filter_param.compare_value));
                            }
                        }
                    }
        ));
    }

    let f = quote!(
        use crate::api_auth::login_impl::AuthBackend;
        use crate::controller::LOGIN_URL;
        use crate::api_doc::{default_resp_docs_with_exam, empty_resp_docs};
        use crate::schema::#schema::dsl::#schema;
        use aide::axum::routing::{delete_with, get_with, post_with, put_with};
        use aide::axum::ApiRouter;
        use axum::extract::{Path};
        use diesel::r2d2::{ConnectionManager, Pool};
        use diesel::{ PgConnection};
        use crate::controller::Compare;
        use crate::controller::Filter;
        use axum_login::permission_required;
        pub(crate) fn web_routes(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
            let router_add = ApiRouter::new().api_route(
                "/create_entity",
                post_with(web::create_entity, empty_resp_docs),
            );
            let router_read = ApiRouter::new()
                .api_route(
                    "/get_entity_by_id/:id",
                    get_with(
                        web::get_entity_by_id,
                        default_resp_docs_with_exam::<#model>,
                    ),
                )
                .api_route(
                    "/get_entity_page",
                    post_with(web::get_entity_page, empty_resp_docs),
                );
            let router_update = ApiRouter::new().api_route(
                "/update_entity_by_id/:id",
                put_with(
                    web::update_entity_by_id,
                    default_resp_docs_with_exam::<#model>,
                ),
            );
            let router_delete = ApiRouter::new().api_route(
                "/delete_entity_by_id/:id",
                delete_with(
                    web::delete_entity_by_id,
                    default_resp_docs_with_exam::<#model>,
                ),
            );
            router_add
                .route_layer(permission_required!(AuthBackend, "common_add"))
                .merge(router_read.route_layer(permission_required!(AuthBackend, "common_read")))
                .merge(router_delete.route_layer(permission_required!(AuthBackend, "common_delete")))
                .merge(router_update.route_layer(permission_required!(AuthBackend, "common_update")))
                .with_state(conn_pool)
        }

        // web_fn_gen!(#schema, #new_model, #model);


        pub mod web {
            use crate::controller::{PageParam, PageRes};
            use super::*;
            use crate::api_doc::extractors::Json;
            use axum::extract::State;
            use diesel::r2d2::{ConnectionManager, Pool};
            use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};


            pub async fn create_entity(
                State(pool): State<Pool<ConnectionManager<PgConnection>>>,
                Json(new_entity): Json<#new_model>,
            ) -> Result<Json<#model>, String> {
                let mut connection = pool.get().unwrap();

                let result = diesel::insert_into(#schema)
                    .values(new_entity)
                    .returning(#model::as_returning())
                    .get_result(&mut connection)
                    .expect("Error saving new entity");

                Ok(Json(result))
            }

            pub async fn update_entity_by_id(
                State(pool): State<Pool<ConnectionManager<PgConnection>>>,
                Path(id_param): Path<i64>,
                Json(new): Json<#new_model>,
            ) -> Result<Json<#model>, String> {
                let mut connection = pool.get().unwrap();
                let result = diesel::update(#schema.find(id_param))
                    .set(&new)
                    .returning(#model::as_returning())
                    .get_result(&mut connection)
                    .expect("Error update  entity");
                Ok(Json(result))
            }

            pub async fn get_entity_by_id(
                State(pool): State<Pool<ConnectionManager<PgConnection>>>,
                Path(id_param): Path<i64>,
            ) -> Result<Json<#model>, String> {
                let mut connection = pool.get().unwrap();
                let result = #schema
                    .find(id_param)
                    .select(#model::as_select())
                    .get_result(&mut connection)
                    .expect("get entity by id failed");
                Ok(Json(result))
            }

            pub async fn delete_entity_by_id(
                State(pool): State<Pool<ConnectionManager<PgConnection>>>,
                Path(id_param): Path<i64>,
            ) -> Result<Json<#model>, String> {
                let mut connection = pool.get().unwrap();
                let result = diesel::update(#schema.find(id_param))
                    .set(crate::schema::#schema::is_delete.eq(true))
                    .returning(#model::as_returning())
                    .get_result(&mut connection)
                    .expect("Error delete  entity");
                Ok(Json(result))
            }

            pub async fn get_entity_page(
                State(pool): State<Pool<ConnectionManager<PgConnection>>>,
                Json(page): Json<PageParam<#builder_ident>>,
            ) -> Result<Json<PageRes<#model, #builder_ident>>, String> {
                let mut connection = pool.get().unwrap();
                let off_lim = page.get_offset_limit();

                let mut statement = crate::schema::#schema::dsl::#schema.into_boxed();
                let mut count_statement = crate::schema::#schema::dsl::#schema.into_boxed();
                let filter = page.filters.clone();
                    #(#filters)*
                count_statement = count_statement.filter(crate::schema::#schema::is_delete.eq(false));

                let total_count = count_statement.count().get_result::<i64>(&mut connection).expect("get count failer");

                let res;
                let x_table = diesel_dynamic_schema::table(stringify!(#schema));

                let order_column = x_table.column::<diesel::sql_types::Text, _>(page.order_column.clone());
                statement = statement.filter(crate::schema::#schema::is_delete.eq(false));

                if page.is_desc {
                    res = statement
                        .offset(off_lim.0)
                        .limit(off_lim.1)
                        .order(order_column.desc())
                        .select(#model::as_select())
                        .load(&mut connection)
                        .expect("Error loading page");
                } else {
                    res = statement
                        .offset(off_lim.0)
                        .limit(off_lim.1)
                        .order(order_column.asc())
                        .select(#model::as_select())
                        .load(&mut connection)
                        .expect("Error loading page");
                }

                let page_res = PageRes::from_param_records_count(page, res,total_count);
                Ok(Json(page_res))
            }
        }

    );

    // let x_header = quote! {};

    for field in opts.fields() {
        builder.push_field(field.as_builder_field());
        // builder.push_setter_fn(field.as_setter());
        // build_fn.push_initializer(field.as_initializer());
    }

    // builder.push_build_fn();

    let mut stream = quote!(#builder);
    stream.append_all(f);
    stream.into()
}
