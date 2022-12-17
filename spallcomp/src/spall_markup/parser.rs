// Big file to convert list of tokens into AST-like thing. Includes custom tree implementation because ones on crates.io were lacking.

use crate::errs;
use super::tag_attribute::TagAttribute;
use super::tag_type::TagType;
use super::tokeniser;

// Spans are not used to contain the inner text of these tags
static SPANLESS_INNER_TEXTS: [&'static str; 3] = ["script", "title", "pageroute"];

pub type NodeIndex = usize;

pub struct Node {
    // Node of the syntax tree

    // The inner text of a node goes before its children when it is rendered.
    // If you want to intersperse text and children then you can just use spans as children to hold the text.
    pub data: NodeData,
    pub children: Vec<NodeIndex>,
    pub parent: Option<NodeIndex>,
}

pub enum NodeData {
    // Payload of a node

    Markup(NodeMarkupData),
    InlineJavascript(NodeInlineJavascriptData),
}

pub struct NodeMarkupData {
    pub tag_name: String,
    pub tag_attributes: Vec<TagAttribute>,
    pub is_standalone: bool,
    pub inner_text: String,
}
pub struct NodeInlineJavascriptData {
    pub value: String,
}

pub struct Tree {
    pub nodes: Vec<Node>,
    pub root: NodeIndex,
}

#[allow(dead_code)]
impl Tree {
    // Class representing syntax tree of the markup nodes

    pub fn new() -> Tree {
        // Create
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
        Tree {
            nodes: vec![node],
            root: 0,
        }
    }

    pub fn add_node(&mut self, parent: NodeIndex, mut node: Node) -> NodeIndex {
        // Add a node to a given other node, returning the index of the node

        let index = self.nodes.len();
        let parent_node = &mut self.nodes[parent];
        parent_node.children.push(index);

        node.parent = Some(parent);
        self.nodes.push(node);
        index
    }

    pub fn get_root(&self) -> &Node {
        &self.nodes[self.root]
    }

    pub fn get_node(&self, node: NodeIndex) -> &Node {
        &self.nodes[node]
    }

    pub fn get_node_mut(&mut self, node: NodeIndex) -> &mut Node {
        &mut self.nodes[node]
    }

    pub fn get_root_mut(&mut self) -> &mut Node {
        &mut self.nodes[self.root]
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

pub fn parse_element(tokens: &Vec<tokeniser::Token>) -> Result<Tree, errs::MarkupSyntaxError> {
    // Convert tokens into a syntax tree - main fn of mod

    // The root-most node is not a real node in the element, but is just there to hold all of the children
    let mut tree = Tree::new();

    let mut node_stack: Vec<NodeIndex> = vec![tree.root];

    // Go through the tokens and delegate to functions for each of the possible tokens
    for token in tokens {
        match token {
            tokeniser::Token::Tag(inner_token) => {
                read_tag_token(&mut tree, &mut node_stack, &inner_token)?
            }
            tokeniser::Token::Content(inner_token) => {
                read_content_token(&mut tree, &mut node_stack, &inner_token)?
            }
            tokeniser::Token::InlineJavascript(inner_token) => {
                read_javascript_token(&mut tree, &mut node_stack, &inner_token)?
            }
        }
    }

    // If we have unclosed nodes, complain!
    if node_stack.len() > 1 {
        let parent = tree.get_node(*node_stack.last().unwrap());
        match &parent.data {
            NodeData::Markup(inner_data) => {
                return Err(errs::MarkupSyntaxError::UnbalancedTag(
                    errs::UnbalancedTag::UnclosedStartTag {
                        tag_name: inner_data.tag_name.to_string(),
                    },
                ))
            }
            _ => (),
        }
    }

    Ok(tree)
}

fn read_tag_token(
    tree: &mut Tree,
    node_stack: &mut Vec<NodeIndex>,
    token: &tokeniser::TagToken,
) -> Result<(), errs::MarkupSyntaxError> {
    // read a HTML tag token and use it to update the node tree

    match token.tag_type {
        // Open tag
        TagType::Start => {
            let new_node_idx = tree.add_node(
                *node_stack
                    .last()
                    .ok_or(errs::MarkupSyntaxError::OrphanedNode)?,
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
            let parent_index = node_stack
                .pop()
                .ok_or(errs::MarkupSyntaxError::OrphanedNode)?;
            let parent = tree.get_node(parent_index);
            if let NodeData::Markup(inner_data) = &parent.data {
                if inner_data.tag_name != token.name {
                    // If parent is root then this tag was never opened
                    if parent.parent.is_none() {
                        return Err(errs::MarkupSyntaxError::UnbalancedTag(
                            errs::UnbalancedTag::UnopenedEndTag {
                                tag_name: token.name.to_string(),
                            },
                        ));
                    }
                    // Otherwise we have an unmatching issue
                    return Err(errs::MarkupSyntaxError::UnbalancedTag(
                        errs::UnbalancedTag::UnmatchingNames {
                            start_tag_name: inner_data.tag_name.to_string(),
                            end_tag_name: token.name.to_string(),
                        },
                    ));
                }
            } else {
                return Err(errs::MarkupSyntaxError::UnmatchedTokenTypes);
            }
        }
        // Standalone tag
        TagType::Standalone => {
            tree.add_node(
                *node_stack
                    .last()
                    .ok_or(errs::MarkupSyntaxError::OrphanedNode)?,
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
    };
    Ok(())
}

fn read_content_token(
    tree: &mut Tree,
    node_stack: &mut Vec<NodeIndex>,
    token: &tokeniser::ContentToken,
) -> Result<(), errs::MarkupSyntaxError> {
    // Transform a content token into a span node

    let parent_idx = *node_stack
    .last()
    .ok_or(errs::MarkupSyntaxError::OrphanedNode)?;
    let parent = tree.get_node_mut(parent_idx);
    
    // Certain nodes should have their inner text injected directly into them instead of having a span inside - look out for those here
    let mut wrap_in_span = true;
    if let NodeData::Markup(inner_data) = &parent.data {
        wrap_in_span = !SPANLESS_INNER_TEXTS.contains(&inner_data.tag_name.as_str())
    }

    if wrap_in_span {
        tree.add_node(
            parent_idx,
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
    Ok(())
}

fn read_javascript_token(
    tree: &mut Tree,
    node_stack: &mut Vec<NodeIndex>,
    token: &tokeniser::InlineJavascriptToken,
) -> Result<(), errs::MarkupSyntaxError> {
    // Read a javascript token and modify the node tree based on it
    // (yes is very similar to the code for tag tokens, but on different types)

    tree.add_node(
        *node_stack
            .last()
            .ok_or(errs::MarkupSyntaxError::OrphanedNode)?,
        Node {
            data: NodeData::InlineJavascript(NodeInlineJavascriptData {
                value: token.value.clone(),
            }),
            parent: None,
            children: vec![],
        },
    );

    Ok(())
}
