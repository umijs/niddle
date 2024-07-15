use html5ever::LocalName;
use indexmap::IndexMap;
use kuchikiki::NodeRef;

use super::NodeRepr;

#[napi]
impl NodeRepr {
  #[napi]
  pub fn append(&self, new_child: &NodeRepr) {
    let node_ref = clone_node_ref_recursive(&new_child.node_ref);
    self.node_ref.append(node_ref)
  }

  #[napi]
  pub fn append_sequence(&self, new_children: Vec<&NodeRepr>) {
    new_children
      .into_iter()
      .for_each(|new_child| self.append(new_child))
  }

  #[napi]
  pub fn prepend(&self, new_child: &NodeRepr) {
    let node_ref = clone_node_ref_recursive(&new_child.node_ref);
    self.node_ref.prepend(node_ref)
  }

  #[napi]
  pub fn prepend_sequence(&self, new_children: Vec<&NodeRepr>) {
    if !new_children.is_empty() {
      self.prepend(new_children[0]);
      new_children
        .iter()
        .skip(1)
        .enumerate()
        .for_each(|(index, new_sibling)| self.get_children()[index].insert_after(new_sibling))
    }
  }

  #[napi]
  pub fn insert_after(&self, new_sibling: &NodeRepr) {
    let node_ref = clone_node_ref_recursive(&new_sibling.node_ref);
    self.node_ref.insert_after(node_ref)
  }

  #[napi]
  pub fn insert_sequence_after(&self, new_siblings: Vec<&NodeRepr>) {
    if !new_siblings.is_empty() {
      self.insert_after(new_siblings[0]);
      new_siblings
        .iter()
        .skip(1)
        .enumerate()
        .for_each(|(index, new_sibling)| {
          if let Some(sibling) = self.node_ref.following_siblings().nth(index) {
            NodeRepr::from(sibling).insert_after(new_sibling)
          }
        });
    }
  }

  #[napi]
  pub fn insert_before(&self, new_sibling: &NodeRepr) {
    let node_ref = clone_node_ref_recursive(&new_sibling.node_ref);
    self.node_ref.insert_before(node_ref)
  }

  #[napi]
  pub fn insert_sequence_before(&self, new_siblings: Vec<&NodeRepr>) {
    if !new_siblings.is_empty() {
      self.insert_before(new_siblings[0]);
      new_siblings.iter().skip(1).for_each(|new_sibling| {
        if let Some(sibling) = self.node_ref.preceding_siblings().last() {
          NodeRepr::from(sibling).insert_after(new_sibling)
        }
      });
    }
  }

  #[napi]
  pub fn remove(&self) {
    self.node_ref.detach()
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

  #[napi]
  pub fn remove_attribute(&self, name: String) {
    if let Some(ele) = self.node_ref.as_element() {
      ele.attributes.borrow_mut().remove(LocalName::from(name));
    }
  }

  #[napi]
  pub fn remove_all_attributes(&self) {
    if let Some(ele) = self.node_ref.as_element() {
      ele.attributes.borrow_mut().map.clear();
    }
  }
}

fn clone_node_ref_recursive(node_ref: &NodeRef) -> NodeRef {
  let new_node_ref = NodeRef::new(node_ref.data().clone());

  node_ref.children().for_each(|child| {
    let child_node_ref = clone_node_ref_recursive(&child);
    new_node_ref.append(child_node_ref);
  });

  new_node_ref
}
