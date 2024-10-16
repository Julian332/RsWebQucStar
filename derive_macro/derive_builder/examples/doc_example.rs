// NOTE: generate fully expanded version with `cargo expand`.
//
//       cargo expand --example doc_example

use derive_builder::WebApiGen;

#[allow(dead_code)]
#[derive(WebApiGen)]
struct Lorem {
    ipsum: u32,
}

fn main() {}
