/// Virtual DOM node types and diffing algorithm.
///
/// RustView uses a server-side virtual DOM. On each re-run, the app function produces
/// a new VNode tree. The differ computes a minimal patch set which is sent to the
/// browser via SSE.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A virtual DOM node representing a UI element.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VNode {
    /// Stable element ID for diffing.
    pub id: String,
    /// HTML tag or component type (e.g., "div", "input", "button").
    pub tag: String,
    /// HTML attributes (e.g., class, type, value).
    pub attrs: HashMap<String, String>,
    /// Text content of the node (if any).
    pub text: Option<String>,
    /// Child nodes.
    pub children: Vec<VNode>,
}

impl VNode {
    /// Create a new VNode with the given tag.
    pub fn new(id: impl Into<String>, tag: impl Into<String>) -> Self {
        VNode {
            id: id.into(),
            tag: tag.into(),
            attrs: HashMap::new(),
            text: None,
            children: Vec::new(),
        }
    }

    /// Set a text content.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Add an attribute.
    pub fn with_attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attrs.insert(key.into(), value.into());
        self
    }

    /// Add a child node.
    pub fn with_child(mut self, child: VNode) -> Self {
        self.children.push(child);
        self
    }
}

/// A patch operation to apply to the live DOM.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "op")]
pub enum Patch {
    /// Replace an entire subtree.
    #[serde(rename = "replace")]
    Replace {
        id: String,
        #[serde(rename = "node")]
        new_node: VNode,
    },
    /// Update text content of a node.
    #[serde(rename = "update_text")]
    UpdateText { id: String, text: String },
    /// Update attributes of a node.
    #[serde(rename = "update_attrs")]
    UpdateAttrs {
        id: String,
        attrs: HashMap<String, String>,
    },
    /// Append a new child to a parent.
    #[serde(rename = "append_child")]
    AppendChild { parent_id: String, node: VNode },
    /// Remove a child node.
    #[serde(rename = "remove_child")]
    RemoveChild { id: String },
    /// Full re-render (used on reconnection).
    #[serde(rename = "full_render")]
    FullRender { root: VNode },
}

/// Compute a minimal list of patches to transform `old` into `new`.
///
/// Uses a subtree-level diffing strategy: if a node's content or attributes
/// have changed, the entire subtree is replaced. This is simpler to implement
/// and debug than attribute-level diffing.
pub fn diff(old: &VNode, new: &VNode) -> Vec<Patch> {
    let mut patches = Vec::new();
    diff_recursive(old, new, &mut patches);
    patches
}

fn diff_recursive(old: &VNode, new: &VNode, patches: &mut Vec<Patch>) {
    // If IDs differ, this is a completely different node — replace.
    if old.id != new.id || old.tag != new.tag {
        patches.push(Patch::Replace {
            id: old.id.clone(),
            new_node: new.clone(),
        });
        return;
    }

    // Check text content change.
    if old.text != new.text {
        match &new.text {
            Some(text) => {
                patches.push(Patch::UpdateText {
                    id: new.id.clone(),
                    text: text.clone(),
                });
            }
            None => {
                patches.push(Patch::UpdateText {
                    id: new.id.clone(),
                    text: String::new(),
                });
            }
        }
    }

    // Check attribute changes.
    if old.attrs != new.attrs {
        patches.push(Patch::UpdateAttrs {
            id: new.id.clone(),
            attrs: new.attrs.clone(),
        });
    }

    // Diff children.
    let old_len = old.children.len();
    let new_len = new.children.len();
    let min_len = old_len.min(new_len);

    // Compare shared children.
    for i in 0..min_len {
        diff_recursive(&old.children[i], &new.children[i], patches);
    }

    // Handle added children.
    for i in min_len..new_len {
        patches.push(Patch::AppendChild {
            parent_id: new.id.clone(),
            node: new.children[i].clone(),
        });
    }

    // Handle removed children.
    for i in min_len..old_len {
        patches.push(Patch::RemoveChild {
            id: old.children[i].id.clone(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_identical_trees() {
        let tree =
            VNode::new("root", "div").with_child(VNode::new("c1", "span").with_text("hello"));
        let patches = diff(&tree, &tree);
        assert!(
            patches.is_empty(),
            "Identical trees should produce no patches"
        );
    }

    #[test]
    fn test_diff_text_change() {
        let old = VNode::new("root", "div").with_child(VNode::new("c1", "span").with_text("hello"));
        let new = VNode::new("root", "div").with_child(VNode::new("c1", "span").with_text("world"));
        let patches = diff(&old, &new);
        assert_eq!(patches.len(), 1);
        match &patches[0] {
            Patch::UpdateText { id, text } => {
                assert_eq!(id, "c1");
                assert_eq!(text, "world");
            }
            _ => panic!("Expected UpdateText patch"),
        }
    }

    #[test]
    fn test_diff_attr_change() {
        let old = VNode::new("root", "div")
            .with_child(VNode::new("c1", "input").with_attr("value", "10"));
        let new = VNode::new("root", "div")
            .with_child(VNode::new("c1", "input").with_attr("value", "20"));
        let patches = diff(&old, &new);
        assert_eq!(patches.len(), 1);
        match &patches[0] {
            Patch::UpdateAttrs { id, attrs } => {
                assert_eq!(id, "c1");
                assert_eq!(attrs.get("value").unwrap(), "20");
            }
            _ => panic!("Expected UpdateAttrs patch"),
        }
    }

    #[test]
    fn test_diff_child_added() {
        let old = VNode::new("root", "div").with_child(VNode::new("c1", "span").with_text("a"));
        let new = VNode::new("root", "div")
            .with_child(VNode::new("c1", "span").with_text("a"))
            .with_child(VNode::new("c2", "span").with_text("b"));
        let patches = diff(&old, &new);
        assert_eq!(patches.len(), 1);
        match &patches[0] {
            Patch::AppendChild { parent_id, node } => {
                assert_eq!(parent_id, "root");
                assert_eq!(node.id, "c2");
            }
            _ => panic!("Expected AppendChild patch"),
        }
    }

    #[test]
    fn test_diff_child_removed() {
        let old = VNode::new("root", "div")
            .with_child(VNode::new("c1", "span").with_text("a"))
            .with_child(VNode::new("c2", "span").with_text("b"));
        let new = VNode::new("root", "div").with_child(VNode::new("c1", "span").with_text("a"));
        let patches = diff(&old, &new);
        assert_eq!(patches.len(), 1);
        match &patches[0] {
            Patch::RemoveChild { id } => {
                assert_eq!(id, "c2");
            }
            _ => panic!("Expected RemoveChild patch"),
        }
    }

    #[test]
    fn test_diff_tag_change_replaces() {
        let old = VNode::new("c1", "span").with_text("hello");
        let new = VNode::new("c1", "div").with_text("hello");
        let patches = diff(&old, &new);
        assert_eq!(patches.len(), 1);
        match &patches[0] {
            Patch::Replace { id, new_node } => {
                assert_eq!(id, "c1");
                assert_eq!(new_node.tag, "div");
            }
            _ => panic!("Expected Replace patch"),
        }
    }

    #[test]
    fn test_diff_multiple_changes() {
        let old = VNode::new("root", "div")
            .with_child(VNode::new("c1", "span").with_text("a"))
            .with_child(VNode::new("c2", "span").with_text("b"))
            .with_child(VNode::new("c3", "span").with_text("c"));
        let new = VNode::new("root", "div")
            .with_child(VNode::new("c1", "span").with_text("x"))
            .with_child(VNode::new("c2", "span").with_text("b"))
            .with_child(VNode::new("c3", "span").with_text("z"));
        let patches = diff(&old, &new);
        assert_eq!(patches.len(), 2); // c1 text change + c3 text change
    }

    #[test]
    fn test_diff_no_change_100_nodes() {
        let mut root = VNode::new("root", "div");
        for i in 0..100 {
            root.children
                .push(VNode::new(format!("c{i}"), "span").with_text(format!("text{i}")));
        }
        let patches = diff(&root, &root.clone());
        assert!(patches.is_empty());
    }
}
