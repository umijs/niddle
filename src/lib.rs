#[macro_use]
extern crate napi_derive;

use element_ref::ElementRef;
use kuchikiki::{parse_html, traits::*};

mod serializer;

mod element_ref;

#[napi]
pub fn parse(html: String) -> ElementRef {
  let parser = parse_html();
  let document_node = parser.one(html).document_node;
  ElementRef {
    node_ref: document_node,
  }
}
