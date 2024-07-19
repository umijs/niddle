use html5ever::serialize::{self, serialize, SerializeOpts};
use indexmap::IndexMap;
use kuchikiki::ExpandedName;

use crate::serializer::serialize_text_only;

use super::NodeRepr;

#[napi]
impl NodeRepr {
  #[napi]
  pub fn select(&self, selectors: String) -> Option<NodeRepr> {
    self.node_ref.select_first(&selectors).ok().map(Into::into)
  }

  #[napi]
  pub fn select_all(&self, selectors: String) -> Vec<NodeRepr> {
    self
      .node_ref
      .select(&selectors)
      .map_or(vec![], |els| els.map(Into::into).collect())
  }

  #[napi]
  pub fn get_children(&self) -> Vec<NodeRepr> {
    self.node_ref.children().map(Into::into).collect()
  }

  #[napi]
  pub fn get_attribute(&self, name: String) -> Option<String> {
    self
      .node_ref
      .as_element()
      .and_then(|e| e.attributes.borrow().get(name).map(|v| v.to_string()))
  }

  #[napi]
  pub fn get_attributes(&self) -> IndexMap<String, String> {
    self.node_ref.as_element().map_or_else(IndexMap::new, |e| {
      e.attributes
        .borrow()
        .map
        .iter()
        .map(|(expanded_name, attr)| {
          let ExpandedName { local, ns: _ } = expanded_name;
          (local.to_string(), attr.value.clone())
        })
        .collect::<IndexMap<String, String>>()
    })
  }

  #[napi]
  pub fn outer_html(&self) -> String {
    let mut u8_vec = Vec::new();
    serialize(
      &mut u8_vec,
      self,
      SerializeOpts {
        traversal_scope: serialize::TraversalScope::IncludeNode,
        create_missing_parent: false,
        scripting_enabled: true,
      },
    )
    .unwrap();
    unsafe { String::from_utf8_unchecked(u8_vec) }
  }

  #[napi]
  pub fn inner_html(&self) -> String {
    let mut buf = Vec::<u8>::new();
    serialize(
      &mut buf,
      self,
      SerializeOpts {
        traversal_scope: serialize::TraversalScope::ChildrenOnly(None),
        create_missing_parent: false,
        scripting_enabled: true,
      },
    )
    .unwrap();
    unsafe { String::from_utf8_unchecked(buf) }
  }

  #[napi]
  pub fn text(&self) -> String {
    let mut buf = Vec::<u8>::new();
    serialize_text_only(&self.node_ref, &mut buf).unwrap();
    unsafe { String::from_utf8_unchecked(buf) }
  }
}
