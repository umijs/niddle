use html5ever::serialize::{self, serialize, SerializeOpts};
use indexmap::IndexMap;
use kuchikiki::{ElementData, ExpandedName, NodeDataRef, NodeRef};
use napi::{
  bindgen_prelude::{FromNapiValue, ToNapiValue},
  Env, JsObject,
};

#[napi]
pub struct ElementRef {
  pub(crate) node_ref: NodeRef,
}

#[napi]
impl ElementRef {
  #[napi]
  pub fn query_selector(&self, selectors: String) -> Option<ElementRef> {
    self.node_ref.select_first(&selectors).ok().map(Into::into)
  }

  #[napi]
  pub fn query_selector_all(&self, selectors: String) -> Vec<ElementRef> {
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
  pub fn get_all_attributes(&self, env: Env) -> Option<JsObject> {
    self.node_ref.as_element().and_then(|e| {
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
        Some(JsObject::from_napi_value(env.raw(), js_value).unwrap())
      }
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
        ..Default::default()
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
        ..Default::default()
      },
    )
    .unwrap();
    String::from_utf8(buf).unwrap()
  }
}

impl From<NodeDataRef<ElementData>> for ElementRef {
  fn from(element: NodeDataRef<ElementData>) -> Self {
    Self {
      node_ref: element.as_node().clone(),
    }
  }
}
