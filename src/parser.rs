// use std::rc::Rc;

use crate::javascript_type::JavascriptType;
use crate::tag_attribute::TagAttribute;
use crate::tag_type::TagType;
use crate::tokeniser;

// Spans are not used to contain the inner text of these tags
static SPANLESS_INNER_TEXTS: [&'static str; 1] = ["script"];

pub type NodeIndex = usize;

pub struct Node {
    // The inner text of a node goes before its children when it is rendered.
    // If you want to intersperse text and children then you can just use spans as children to hold the text.
    pub data: NodeData,
    pub children: Vec<NodeIndex>,
    pub parent: Option<NodeIndex>,
}

pub enum NodeData {
    Markup(NodeMarkupData),
    JavascriptBlock(NodeJavascriptBlockData),
    JavascriptStandalone(NodeJavascriptStandaloneData),
}

// We can't pass specific enum variants around so just make structs that the enum wraps
pub struct NodeMarkupData {
    pub tag_name: String,
    pub tag_attributes: Vec<TagAttribute>,
    pub is_standalone: bool,
    pub inner_text: String,
}
pub struct NodeJavascriptBlockData {
    pub start_value: String,
    pub end_value: String,
}
pub struct NodeJavascriptStandaloneData {
    pub value: String,
}

pub struct Tree {
    pub nodes: Vec<Node>,
    pub root: NodeIndex,
}

#[allow(dead_code)]
impl Tree {
    pub fn new() -> Tree {
        let node = Node {
            data: NodeData::Markup(NodeMarkupData {
                tag_name: "".to_string(),
                tag_attributes: vec![],
                is_standalone: false,
                inner_text: "".to_string(),
            }),
            children: vec![],
            parent: None,
        };
        return Tree {
            nodes: vec![node],
            root: 0,
        };
    }

    pub fn add_node(&mut self, parent: NodeIndex, mut node: Node) -> NodeIndex {
        let index = self.nodes.len();
        let parent_node = &mut self.nodes[parent];
        parent_node.children.push(index);

        node.parent = Some(parent);
        self.nodes.push(node);
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

    pub fn depth_first_map<T>(&self, enter_func: &mut T)
    where
        T: FnMut(&Node, bool),
    {
        self.inner_depth_first_map(self.root, enter_func);
    }

    fn inner_depth_first_map<T>(&self, node: NodeIndex, enter_func: &mut T)
    where
        T: FnMut(&Node, bool),
    {
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

    let mut node_stack: Vec<NodeIndex> = vec![tree.root];

    for token in tokens {
        match token {
            tokeniser::Token::Tag(inner_token) => {
                read_tag_token(&mut tree, &mut node_stack, &inner_token)
            }
            tokeniser::Token::Content(inner_token) => {
                read_content_token(&mut tree, &mut node_stack, &inner_token)
            }
            tokeniser::Token::InlineJavascript(inner_token) => {
                read_javascript_token(&mut tree, &mut node_stack, &inner_token)
            }
        }
    }

    return tree;
}

fn read_tag_token(tree: &mut Tree, node_stack: &mut Vec<NodeIndex>, token: &tokeniser::TagToken) {
    // read a HTML tag token and use it to update the node tree

    match token.tag_type {
        // Open tag
        TagType::Start => {
            let new_node_idx = tree.add_node(
                *node_stack.last().unwrap(),
                Node {
                    data: NodeData::Markup(NodeMarkupData {
                        tag_name: token.name.clone(),
                        tag_attributes: token.attributes.clone(),
                        is_standalone: false,
                        inner_text: "".to_string(),
                    }),
                    parent: None,
                    children: vec![],
                },
            );
            node_stack.push(new_node_idx);
        }
        // End tag
        TagType::End => {
            node_stack.pop();
        }
        // Standalone tag
        TagType::Standalone => {
            tree.add_node(
                *node_stack.last().unwrap(),
                Node {
                    data: NodeData::Markup(NodeMarkupData {
                        tag_name: token.name.clone(),
                        tag_attributes: token.attributes.clone(),
                        is_standalone: true,
                        inner_text: "".to_string(),
                    }),
                    parent: None,
                    children: vec![],
                },
            );
        }
    }
}

fn read_content_token(
    tree: &mut Tree,
    node_stack: &mut Vec<NodeIndex>,
    token: &tokeniser::ContentToken,
) {
    // Transform a content token into a span node

    let mut wrap_in_span = true;
    let parent = tree.get_node_mut(*node_stack.last().unwrap());
    if let NodeData::Markup(inner_data) = &parent.data {
        wrap_in_span = !SPANLESS_INNER_TEXTS.contains(&inner_data.tag_name.as_str())
    }
    if wrap_in_span {
        tree.add_node(
            *node_stack.last().unwrap(),
            Node {
                data: NodeData::Markup(NodeMarkupData {
                    tag_name: "span".to_string(),
                    tag_attributes: vec![],
                    is_standalone: false,
                    inner_text: token.value.to_string(),
                }),
                parent: None,
                children: vec![],
            },
        );
    } else if let NodeData::Markup(inner_data) = &mut parent.data {
        inner_data.inner_text = token.value.clone();
    }
}

fn read_javascript_token(
    tree: &mut Tree,
    node_stack: &mut Vec<NodeIndex>,
    token: &tokeniser::InlineJavascriptToken,
) {
    // Read a javascript token and modify the node tree based on it
    // (yes is very similar to the code for tag tokens, but on different types)

    match token.javascript_type {
        // Javascript open block
        JavascriptType::BlockStart => {
            let new_node_idx = tree.add_node(
                *node_stack.last().unwrap(),
                Node {
                    data: NodeData::JavascriptBlock(NodeJavascriptBlockData {
                        start_value: token.value.to_string(),
                        end_value: "".to_string(),
                    }),
                    parent: None,
                    children: vec![],
                },
            );
            node_stack.push(new_node_idx);
        }
        // Javascript end block
        JavascriptType::BlockEnd => {
            let node = tree.get_node_mut(node_stack.pop().unwrap());
            if let NodeData::JavascriptBlock(ref mut inside_data) = node.data {
                inside_data.end_value = "}".to_string();
            }
        }
        // Standalone JS
        JavascriptType::Standalone => {
            tree.add_node(
                *node_stack.last().unwrap(),
                Node {
                    data: NodeData::JavascriptStandalone(NodeJavascriptStandaloneData {
                        value: token.value.clone(),
                    }),
                    parent: None,
                    children: vec![],
                },
            );
        }
    }
}
