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

impl From<NodeRef> for NodeRepr {
  fn from(node_ref: NodeRef) -> Self {
    Self { node_ref }
  }
}

#[napi]
impl NodeRepr {
  #[napi(js_name = "clone")]
  pub fn clone_self_only(&self) -> NodeRepr {
    let new_node_ref = NodeRef::new(self.node_ref.data().clone());
    NodeRepr::from(new_node_ref)
  }

  #[napi]
  pub fn clone_recursive(&self) -> NodeRepr {
    NodeRepr::from(clone_node_ref_recursive(&self.node_ref))
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
