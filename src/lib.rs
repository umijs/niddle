#[macro_use]
extern crate napi_derive;

use element_ref::NodeRepr;
use kuchikiki::{parse_html, traits::*};

mod serializer;

mod element_ref;

#[napi]
pub fn parse(html: String) -> NodeRepr {
  let parser = parse_html();
  let document_node = parser.one(html).document_node;
  NodeRepr {
    node_ref: document_node,
  }
}
