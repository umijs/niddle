use kuchikiki::{ElementData, NodeDataRef, NodeRef};

mod modify;
mod query;

#[napi]
pub struct NodeRepr {
  pub(crate) node_ref: NodeRef,
}

impl From<NodeDataRef<ElementData>> for NodeRepr {
  fn from(element: NodeDataRef<ElementData>) -> Self {
    Self {
      node_ref: element.as_node().clone(),
    }
  }
}
