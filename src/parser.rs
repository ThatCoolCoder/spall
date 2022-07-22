// use std::rc::Rc;

use crate::tokeniser;

pub type NodeIndex = usize;

pub struct Node {
    pub tag_name: String,
    pub inner_text: String,
    pub children: Vec<NodeIndex>
}

pub struct Tree {
    pub nodes: Vec<Node>,
    pub root: NodeIndex
}

impl Tree {
    pub fn new() -> Tree {
        let node = Node {
            tag_name: "".to_string(),
            inner_text: "".to_string(),
            children: vec!()
        };
        return Tree {
            nodes: vec!(node),
            root: 0
        }
    }

    pub fn add_node(&mut self, parent: NodeIndex, tag_name: String, inner_text: String) -> NodeIndex {
        let index = self.nodes.len();
        let parent = &mut self.nodes[parent];
        parent.children.push(index);

        self.nodes.push(Node {
            tag_name: tag_name,
            inner_text: inner_text,
            children: vec!()
        });
        return index;
    }

    pub fn get_root(&self) -> &Node {
        return &self.nodes[self.root];
    }

    pub fn get_node(&self, node: NodeIndex) -> &Node {
        return &self.nodes[node];
    }

    pub fn get_node_mut(&mut self, node: NodeIndex) -> &mut Node {
        return &mut self.nodes[node];
    }

    pub fn get_root_mut(&mut self) -> &mut Node {
        return &mut self.nodes[self.root];
    }
}


pub fn parse_element(tokens: &Vec<tokeniser::Token>) -> Tree {
    // Arrange tokens in a hierarchy.
    // The root-most node is not a real node in the element, but is just there to hold all of the children

    // todo: figure out how to have nodes in both a tree and a stack at the same time

    let mut tree = Tree::new();

    let mut node_stack: Vec<NodeIndex> = vec!(tree.root);

    for token in tokens {
        match token {
            tokeniser::Token::Tag { name, is_start } => {
                if *is_start {
                    let new_node_idx = tree.add_node(*node_stack.last().unwrap(), name.clone(), "".to_string());
                    node_stack.push(new_node_idx);
                }
                else {
                    node_stack.pop();
                }
            }
            tokeniser::Token::Content { value } => {
                // let node = Node::new_span(value);
                // node_stack.last().unwrap().children.push(node);
                // tree.add_node(*node_stack.last().unwrap(), "span".to_string(), value.clone());
                tree.get_node_mut(*node_stack.last().unwrap()).inner_text = value.clone();
            }
        }
    }

    return tree;
}