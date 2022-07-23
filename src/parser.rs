// use std::rc::Rc;

use crate::tokeniser;

pub type NodeIndex = usize;

pub struct Node {
    pub tag_name: String,

    // The inner text of a node goes before its children when it is rendered. If you want to intersperse text and children then you can just use spans to hold the text.
    pub inner_text: String,

    pub children: Vec<NodeIndex>,
    pub parent: Option<NodeIndex>
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
            children: vec!(),
            parent: None
        };
        return Tree {
            nodes: vec!(node),
            root: 0
        }
    }

    pub fn add_node(&mut self, parent: NodeIndex, tag_name: String, inner_text: String) -> NodeIndex {
        let index = self.nodes.len();
        let parent_node = &mut self.nodes[parent];
        parent_node.children.push(index);

        self.nodes.push(Node {
            tag_name: tag_name,
            inner_text: inner_text,
            children: vec!(),
            parent: Some(parent)
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

    pub fn depth_first_map<T>(&self, enter_func: &mut T) where T: FnMut(&Node, bool) {
        self.inner_depth_first_map(self.root, enter_func);
    }

    fn inner_depth_first_map<T>(&self, node: NodeIndex, enter_func: &mut T) where T: FnMut(&Node, bool) {
        let node = self.get_node(node);
        enter_func(node, true);
        for child in &node.children {
            self.inner_depth_first_map(*child, enter_func);
        }
        enter_func(node, false);
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
                tree.add_node(*node_stack.last().unwrap(), "span".to_string(), value.to_string());
            }
        }
    }

    return tree;
}