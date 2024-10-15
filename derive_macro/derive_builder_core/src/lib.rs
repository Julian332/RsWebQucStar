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
                if let Some(filter) = page.filters.clone() {
                    if let Some(filter_param) = filter.#ident() {
                        statement = statement.filter(crate::schema::#schema::#ident.eq(filter_param));
                    }
                }
        ));
    }

    let f = quote!(
        pub mod #schema {
            use crate::controller::{PageParam, PageRes};
            use crate::models::{User, UserBuilder};
            use crate::openapi::extractors::Json;
            use axum::extract::State;
            use diesel::r2d2::{ConnectionManager, Pool};
            use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};


            pub async fn get_entity_page(
            State(pool): State<Pool<ConnectionManager<PgConnection>>>,
            Json(page): Json<PageParam<#builder_ident>>,
        ) -> Result<Json<PageRes<#model, #builder_ident>>, String> {
            let mut connection = pool.get().unwrap();
            let off_lim = page.get_offset_limit();

            let mut statement = crate::schema::#schema::dsl::#schema.into_boxed();

            #(#filters)*

            let res;
            let x_table = diesel_dynamic_schema::table(stringify!(crate::schema::#schema));

            let order_column = x_table.column::<diesel::sql_types::Text, _>(page.order_column.clone());
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

            let page_res = PageRes::from_param_records(page, res);
            Ok(Json(page_res))
        }
        }

    );

    // let x_header = quote! {};

    for field in opts.fields() {
        builder.push_field(field.as_builder_field());
        builder.push_setter_fn(field.as_setter());
        // build_fn.push_initializer(field.as_initializer());
    }

    // builder.push_build_fn();

    let mut stream = quote!(#builder);
    stream.append_all(f);
    stream.into()
}
