use std::vec;

use crate::tokeniser;

pub struct Node {
    is_root: bool,
    tag_name: String,
    inner_text: String,
    children: Vec<Node>
}

impl Node {
    fn new(tag_name: String) -> Node {
        return Node {
            is_root: false,
            tag_name: tag_name,
            inner_text: "".to_string(),
            children: vec!()
        }
    }

    fn new_root() -> Node {
        return Node {
            is_root: true,
            tag_name: "".to_string(),
            inner_text: "".to_string(),
            children: vec!()
        }
    }

    fn new_span(text: String) -> Node {
        return Node {
            is_root: false,
            tag_name: "span".to_string(),
            inner_text: text,
            children: vec!()
        }
    }
}

pub fn parse_element(tokens: Vec<tokeniser::Token>) -> Node {
    // Arrange tokens in a hierarchy.
    // The root-most node is not a real node in the element, but is just there to hold all of the children

    // todo: figure out how to have nodes in both a tree and a stack at the same time

    let mut tag_stack: Vec<String> = vec!();

    let mut root_node = Node::new_root();

    let mut node_stack: Vec<&mut Node> = vec!(&mut root_node);

    for token in tokens {
        match token {
            tokeniser::Token::Tag { name, is_start } => {
                if is_start {
                    let mut node = Node::new("".to_string());
                    node_stack.last_mut().unwrap().children.push(node);
                }
                else {
                    node_stack.pop();
                }
            }
            tokeniser::Token::Content { value } => {
                let node = Node::new_span(value);
                // node_stack.last().unwrap().children.push(node);
            }
        }
    }

    return root_node;
}