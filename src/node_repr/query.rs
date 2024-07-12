use html5ever::serialize::{self, serialize, SerializeOpts};
use indexmap::IndexMap;
use kuchikiki::ExpandedName;
use napi::{
  bindgen_prelude::{FromNapiValue, ToNapiValue},
  Env, JsObject,
};

use super::NodeRepr;

#[napi]
impl NodeRepr {
  #[napi]
  pub fn query_selector(&self, selectors: String) -> Option<NodeRepr> {
    self.node_ref.select_first(&selectors).ok().map(Into::into)
  }

  #[napi]
  pub fn query_selector_all(&self, selectors: String) -> Vec<NodeRepr> {
    self
      .node_ref
      .select(&selectors)
      .map_or(vec![], |els| els.map(Into::into).collect())
  }

  #[napi]
  pub fn get_attribute(&self, name: String) -> Option<String> {
    self
      .node_ref
      .as_element()
      .and_then(|e| e.attributes.borrow().get(name).map(|v| v.to_string()))
  }

  #[napi]
  pub fn get_attributes(&self, env: Env) -> JsObject {
    self.node_ref.as_element().map_or_else(
      || env.create_object().unwrap(),
      |e| {
        let attrs_map = e
          .attributes
          .borrow()
          .map
          .iter()
          .map(|(expanded_name, attr)| {
            let ExpandedName { local, ns: _ } = expanded_name;
            (local.to_string(), attr.value.to_string())
          })
          .collect::<IndexMap<String, String>>();

        unsafe {
          let js_value = ToNapiValue::to_napi_value(env.raw(), attrs_map).unwrap();
          JsObject::from_napi_value(env.raw(), js_value).unwrap()
        }
      },
    )
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
    String::from_utf8(u8_vec).unwrap()
  }

  #[napi]
  pub fn inner_html(&self) -> String {
    let mut buf = Vec::new();
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
    String::from_utf8(buf).unwrap()
  }
}
