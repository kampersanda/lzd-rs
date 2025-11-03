use std::cmp;

pub const NIL_ID: usize = usize::MAX;

#[derive(Debug)]
struct Node<'text> {
    factor_id: usize,
    child_id: usize,
    sibling_id: usize,
    edge_text: &'text [u8],
}

#[derive(Debug)]
pub struct Trie<'text> {
    nodes: Vec<Node<'text>>,
}

/// Get the length of longest common prefix.
fn longest_common_prefix(a: &[u8], b: &[u8]) -> usize {
    let min_len = cmp::min(a.len(), b.len());
    for i in 0..min_len {
        if a[i] != b[i] {
            return i;
        }
    }
    min_len
}

/// A first-child next sibling trie implementaton.
impl<'text> Default for Trie<'text> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'text> Trie<'text> {
    pub fn new() -> Self {
        Trie {
            nodes: vec![Node {
                factor_id: NIL_ID,
                child_id: NIL_ID,
                sibling_id: NIL_ID,
                edge_text: &[],
            }],
        }
    }

    pub fn get_factor_id(&self, node_id: usize) -> usize {
        self.nodes[node_id].factor_id
    }

    pub fn set_factor_id(&mut self, node_id: usize, factor_id: usize) {
        assert_eq!(self.nodes[node_id].factor_id, NIL_ID);
        self.nodes[node_id].factor_id = factor_id;
    }

    pub fn get_edge_len(&self, node_id: usize) -> usize {
        self.nodes[node_id].edge_text.len()
    }

    /// Add from the given node a new child with the given Factor ID.
    pub fn add_child(&mut self, node_id: usize, new_factor_id: usize, text: &'text [u8]) {
        if self.nodes[node_id].child_id == NIL_ID {
            let new_child_id = self.push_node(Node {
                factor_id: new_factor_id,
                child_id: NIL_ID,
                sibling_id: NIL_ID,
                edge_text: text,
            });
            self.nodes[node_id].child_id = new_child_id;
            return;
        }

        let mut child_id = self.nodes[node_id].child_id;
        let mut prev_id = NIL_ID;

        loop {
            let edge_text = self.nodes[child_id].edge_text;

            if text[0] == edge_text[0] {
                // Insert a new branch
                let lcp = longest_common_prefix(text, edge_text);
                self.nodes[child_id].edge_text = &edge_text[lcp..];

                let middle_id = self.push_node(Node {
                    factor_id: NIL_ID,
                    child_id,
                    sibling_id: self.nodes[child_id].sibling_id,
                    edge_text: &edge_text[..lcp],
                });
                self.nodes[child_id].sibling_id = NIL_ID;

                if prev_id == NIL_ID {
                    self.nodes[node_id].child_id = middle_id;
                } else {
                    self.nodes[prev_id].sibling_id = middle_id;
                }

                if lcp < text.len() {
                    // Add a new child in the middle
                    let new_sibling_id = self.push_node(Node {
                        factor_id: new_factor_id,
                        child_id: NIL_ID,
                        sibling_id: NIL_ID,
                        edge_text: &text[lcp..],
                    });
                    self.nodes[child_id].sibling_id = new_sibling_id;
                } else {
                    self.nodes[middle_id].factor_id = new_factor_id;
                }
                return;
            }

            if self.nodes[child_id].sibling_id == NIL_ID {
                let new_sibling_id = self.push_node(Node {
                    factor_id: new_factor_id,
                    child_id: NIL_ID,
                    sibling_id: NIL_ID,
                    edge_text: text,
                });
                self.nodes[child_id].sibling_id = new_sibling_id;
                return;
            }

            prev_id = child_id;
            child_id = self.nodes[child_id].sibling_id;
        }
    }

    /// Find the child with the edge text.
    pub fn find_child(&self, node_id: usize, text: &'text [u8]) -> usize {
        debug_assert!(!text.is_empty());

        let mut child_id = self.nodes[node_id].child_id;
        loop {
            if child_id == NIL_ID {
                return NIL_ID;
            }
            if text[0] == self.nodes[child_id].edge_text[0] {
                break;
            }
            child_id = self.nodes[child_id].sibling_id;
        }

        let edge_text = self.nodes[child_id].edge_text;
        if text.len() < edge_text.len() {
            return NIL_ID;
        }

        for i in 0..edge_text.len() {
            if text[i] != edge_text[i] {
                return NIL_ID;
            }
        }
        child_id
    }

    fn push_node(&mut self, node: Node<'text>) -> usize {
        let new_node_id = self.nodes.len();
        self.nodes.push(node);
        new_node_id
    }
}
