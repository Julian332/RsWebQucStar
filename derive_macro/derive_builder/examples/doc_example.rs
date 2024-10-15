// NOTE: generate fully expanded version with `cargo expand`.
//
//       cargo expand --example doc_example

use derive_builder::PageQuery;

#[allow(dead_code)]
#[derive(PageQuery)]
struct Lorem {
    ipsum: u32,
}

fn main() {}
