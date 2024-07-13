use html5ever::LocalName;
use indexmap::IndexMap;
use kuchikiki::NodeRef;

use super::NodeRepr;

#[napi]
impl NodeRepr {
  #[napi]
  pub fn append(&self, new_child: &NodeRepr) {
    let node_ref = clone_node_recursive(&new_child.node_ref);
    self.node_ref.append(node_ref)
  }

  #[napi]
  pub fn prepend(&self, new_child: &NodeRepr) {
    let node_ref = clone_node_recursive(&new_child.node_ref);
    self.node_ref.prepend(node_ref)
  }

  #[napi]
  pub fn insert_after(&self, new_sibling: &NodeRepr) {
    let node_ref = clone_node_recursive(&new_sibling.node_ref);
    self.node_ref.insert_after(node_ref)
  }

  #[napi]
  pub fn insert_before(&self, new_sibling: &NodeRepr) {
    let node_ref = clone_node_recursive(&new_sibling.node_ref);
    self.node_ref.insert_before(node_ref)
  }

  #[napi]
  pub fn set_attribute(&self, name: String, value: String) {
    if let Some(ele) = self.node_ref.as_element() {
      ele
        .attributes
        .borrow_mut()
        .insert(LocalName::from(name), value);
    }
  }

  #[napi]
  pub fn set_attributes(&self, attrs: IndexMap<String, String>) {
    if let Some(ele) = self.node_ref.as_element() {
      attrs.into_iter().for_each(|(name, value)| {
        ele
          .attributes
          .borrow_mut()
          .insert(LocalName::from(name), value);
      });
    }
  }
}

fn clone_node_recursive(node_ref: &NodeRef) -> NodeRef {
  let node_data = node_ref.data().clone();
  let new_node_ref = NodeRef::new(node_data);
  node_ref.children().for_each(|child| {
    let child_node_ref = clone_node_recursive(&child);
    new_node_ref.append(child_node_ref);
  });

  new_node_ref
}
