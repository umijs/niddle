use html5ever::{
  serialize::{
    Serialize, Serializer,
    TraversalScope::{self, ChildrenOnly, IncludeNode},
  },
  QualName,
};
use kuchikiki::NodeData;
use std::io::Result;

use crate::node_repr::NodeRepr;

impl Serialize for NodeRepr {
  fn serialize<S: Serializer>(
    &self,
    serializer: &mut S,
    traversal_scope: TraversalScope,
  ) -> Result<()> {
    match (traversal_scope, self.node_ref.data()) {
      (ref scope, NodeData::Element(element)) => {
        if *scope == IncludeNode {
          let attrs = element.attributes.borrow();

          let attrs = attrs
            .map
            .iter()
            .map(|(name, attr)| {
              (
                QualName::new(attr.prefix.clone(), name.ns.clone(), name.local.clone()),
                &attr.value,
              )
            })
            .collect::<Vec<_>>();

          serializer.start_elem(
            element.name.clone(),
            attrs.iter().map(|&(ref name, value)| (name, &**value)),
          )?
        }

        for child in self.node_ref.children() {
          Serialize::serialize(&child, serializer, IncludeNode)?
        }

        if *scope == IncludeNode {
          serializer.end_elem(element.name.clone())?
        }
        Ok(())
      }

      (_, &NodeData::DocumentFragment) | (_, &NodeData::Document(_)) => {
        for child in self.node_ref.children() {
          Serialize::serialize(&child, serializer, IncludeNode)?
        }
        Ok(())
      }

      (ChildrenOnly(_), _) => Ok(()),

      (IncludeNode, NodeData::Doctype(doctype)) => serializer.write_doctype(&doctype.name),
      (IncludeNode, NodeData::Text(text)) => serializer.write_text(&text.borrow()),
      (IncludeNode, NodeData::Comment(text)) => serializer.write_comment(&text.borrow()),
      (IncludeNode, NodeData::ProcessingInstruction(contents)) => {
        let contents = contents.borrow();
        serializer.write_processing_instruction(&contents.0, &contents.1)
      }
    }
  }
}
