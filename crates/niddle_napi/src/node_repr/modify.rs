use html5ever::{tendril::StrTendril, LocalName};
use indexmap::IndexMap;

use super::NodeRepr;

#[napi]
impl NodeRepr {
  /// Append a child node to this node, after existing children.
  ///
  /// The child node will be remove from its previous position.
  #[napi]
  pub fn append(&self, new_child: &NodeRepr) {
    self.0.append(new_child.0.clone())
  }

  /// Append some children nodes to this node by order, after existing children.
  ///
  /// These children nodes will be remove from their previous position.
  #[napi]
  pub fn append_sequence(&self, new_children: Vec<&NodeRepr>) {
    new_children
      .into_iter()
      .for_each(|new_child| self.append(new_child))
  }

  /// Prepend a child node to this node, before existing children.
  ///
  /// The child node will be remove from its previous position.
  #[napi]
  pub fn prepend(&self, new_child: &NodeRepr) {
    self.0.prepend(new_child.0.clone())
  }

  /// Prepend some children nodes to this node by order, before existing children.
  ///
  /// These children nodes will be remove from their previous position.
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

  /// Insert a new sibling after this node.
  ///
  /// The sibling node will be remove from its previous position.
  #[napi]
  pub fn insert_after(&self, new_sibling: &NodeRepr) {
    self.0.insert_after(new_sibling.0.clone())
  }

  /// Insert some siblings after this node.
  ///
  /// These sibling nodes will be remove from their previous position.
  #[napi]
  pub fn insert_sequence_after(&self, new_siblings: Vec<&NodeRepr>) {
    if !new_siblings.is_empty() {
      self.insert_after(new_siblings[0]);
      new_siblings
        .iter()
        .skip(1)
        .enumerate()
        .for_each(|(index, new_sibling)| {
          if let Some(sibling) = self.0.following_siblings().nth(index) {
            NodeRepr::from(sibling).insert_after(new_sibling)
          }
        });
    }
  }

  /// Insert a new sibling before this node.
  ///
  /// The sibling node will be remove from its previous position.
  #[napi]
  pub fn insert_before(&self, new_sibling: &NodeRepr) {
    self.0.insert_before(new_sibling.0.clone())
  }

  /// Insert some siblings before this node.
  ///
  /// These sibling nodes will be remove from their previous position.
  #[napi]
  pub fn insert_sequence_before(&self, new_siblings: Vec<&NodeRepr>) {
    if !new_siblings.is_empty() {
      self.insert_before(new_siblings[0]);
      new_siblings.iter().skip(1).for_each(|new_sibling| {
        if let Some(sibling) = self.0.preceding_siblings().last() {
          NodeRepr::from(sibling).insert_after(new_sibling)
        }
      });
    }
  }

  /// Remove a node from its parent and siblings. Children are not affected.
  #[napi]
  pub fn remove(&self) {
    self.0.detach()
  }

  /// Assign an attribute K-V to this node
  #[napi]
  pub fn set_attribute(&self, name: String, value: String) {
    if let Some(ele) = self.0.as_element() {
      ele
        .attributes
        .borrow_mut()
        .insert(LocalName::from(name), StrTendril::from(value));
    }
  }

  /// Assign attributes K-V object to this node
  #[napi]
  pub fn set_attributes(&self, attrs: IndexMap<String, String>) {
    if let Some(ele) = self.0.as_element() {
      attrs.into_iter().for_each(|(name, value)| {
        ele
          .attributes
          .borrow_mut()
          .insert(LocalName::from(name), StrTendril::from(value));
      });
    }
  }

  /// Remove an attribute of this node by name
  #[napi]
  pub fn remove_attribute(&self, name: String) {
    if let Some(ele) = self.0.as_element() {
      ele.attributes.borrow_mut().remove(LocalName::from(name));
    }
  }

  /// Remove all attributes of this node
  #[napi]
  pub fn remove_all_attributes(&self) {
    if let Some(ele) = self.0.as_element() {
      ele.attributes.borrow_mut().map.clear();
    }
  }
}
