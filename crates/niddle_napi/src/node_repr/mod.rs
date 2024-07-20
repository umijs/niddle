use kuchikiki::{ElementData, NodeDataRef, NodeRef};

mod modify;
mod query;

#[napi]
pub struct NodeRepr(pub(crate) NodeRef);

impl From<NodeDataRef<ElementData>> for NodeRepr {
  fn from(element: NodeDataRef<ElementData>) -> Self {
    Self(element.as_node().clone())
  }
}

impl From<NodeRef> for NodeRepr {
  fn from(node_ref: NodeRef) -> Self {
    Self(node_ref)
  }
}

#[napi]
impl NodeRepr {
  ///
  /// @private
  ///
  #[napi(constructor, ts_return_type = "void")]
  pub fn constructor() {
    unreachable!()
  }
}

#[napi]
impl NodeRepr {
  /// Clone this node to a new instance, not clone its descendants.
  #[napi(js_name = "clone")]
  pub fn clone_self_only(&self) -> NodeRepr {
    let new_node_ref = NodeRef::new(self.0.data().clone());
    NodeRepr::from(new_node_ref)
  }

  /// Clone this node to a new instance, including its all descendants.
  #[napi]
  pub fn clone_recursive(&self) -> NodeRepr {
    NodeRepr::from(clone_node_ref_recursive(&self.0))
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
