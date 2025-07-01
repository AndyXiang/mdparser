#![allow(unused)]

// parsing phase1:
// lines of input are consumed and create a tree structure

use std::str::Chars;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    block: Block,
}

impl Node {
    fn new(block: Block) -> Self {
        Self {
            children: vec![],
            block,
        }
    }
}

#[derive(Debug)]
enum ListType {
    Bullet,
    Ordered,
}

#[derive(Debug)]
enum Block {
    Document,
    Header { level: usize },
    Quote,
    SoftBreak,
    Code { info: String, code: String },
    List { list_type: ListType },
    ListItem,
    Paragraph,
    Emph,
    Strong,
    CodeSpan,
    Text { content: String },
}

struct Parser {
    state: State,
}

enum State {
    Document,
    Header,
    Quote,
    List,
    ListItem,
    Paragraph,
}

fn parse(parent_block: Block, char_iter: &mut Chars<'_>) -> Vec<Node> {
    match parent_block {
        Block::Document => _parse_document(char_iter),
        _ => {
            todo!();
        }
    }
}

fn _parse_document(char_iter: &mut Chars<'_>) -> Vec<Node> {
    let mut output_nodes: Vec<Node> = vec![];
    while let Some(c) = char_iter.next() {
        match c {
            '#' => {
                output_nodes.push(_parse_header(char_iter));
            }
            '>' => {
                //create quote
                output_nodes.push(_parse_quote(char_iter));
            }
            '-' => {
                //create list
                output_nodes.push(_parse_list(ListType::Bullet, char_iter));
            }
            '\n' => {}
            _ => (),
        }
    }

    output_nodes
}

fn _parse_header(char_iter: &mut Chars<'_>) -> Node {
    let mut level = 1;
    let mut children = vec![];
    while let Some(ch) = char_iter.next() {
        match ch {
            '#' => level += 1,
            ' ' => {
                children.push(_parse_text("", char_iter));
            }
            _ => {
                // no whitespace, treat as normal Paragraph
                // FIXME: no handle of softbreak situation
                let mut pre_str = "#".repeat(level);
                pre_str.push(ch);
                let mut text_block = _parse_text(&pre_str, char_iter);
                return Node {
                    block: Block::Paragraph,
                    children: vec![text_block],
                };
            }
        }
    }

    Node {
        children,
        block: Block::Header { level },
    }
}

fn _parse_para(char_iter: &mut Chars<'_>) -> Node {
    Node {
        block: Block::Paragraph,
        children: vec![_parse_text("", char_iter)],
    }
}

fn _parse_quote(char_iter: &mut Chars<'_>) -> Node {
    let mut children = vec![];

    while let Some(ch) = char_iter.next() {
        match ch {
            ' ' => {
                continue;
            }
            '>' => {
                children.push(_parse_quote(char_iter));
            }
            '#' => {
                children.push(_parse_header(char_iter));
            }
            '-' => {
                children.push(_parse_list(ListType::Bullet, char_iter));
            }
            _ => {
                let mut text_block = _parse_text(&ch.to_string(), char_iter);
                children.push(Node {
                    block: Block::Paragraph,
                    children: vec![text_block],
                });
            }
        }
    }

    Node {
        block: Block::Quote,
        children,
    }
}

fn _parse_list(list_type: ListType, char_iter: &mut Chars<'_>) -> Node {
    let mut children = vec![];

    while let Some(ch) = char_iter.next() {
        match ch {
            ' ' => {
                children.push(_parse_listitem(char_iter));
            }
            '\n' => {
                if children.is_empty() {
                    // allow empty list and list item
                    return Node {
                        children: vec![Node {
                            children: vec![],
                            block: Block::ListItem,
                        }],
                        block: Block::List { list_type },
                    };
                }
                return Node {
                    block: Block::List {
                        list_type: ListType::Bullet,
                    },
                    children,
                };
            }
            '-' => {
                if children.is_empty() {
                    // pattern as `--`, parse as raw text
                    return Node {
                        children: vec![_parse_text(&String::from("--"), char_iter)],
                        block: Block::Paragraph,
                    };
                }
                if let Some(ch) = char_iter.next() {
                    match ch {
                        ' ' => {
                            children.push(_parse_listitem(char_iter));
                        }
                        _ => {
                            // get the text child of the last listitem
                            let mut lastitem_text = children
                                .pop()
                                .unwrap() // get the last listitem
                                .children
                                .pop()
                                .unwrap() // get the last paragraph
                                .children
                                .pop()
                                .unwrap()
                                .block; // get the last text
                            let mut previous_str = String::from("-");
                            previous_str.push(ch);
                            children.push(Node {
                                children: vec![Node {
                                    children: vec![_parse_softbreak(
                                        lastitem_text,
                                        &previous_str,
                                        char_iter,
                                    )],
                                    block: Block::Paragraph,
                                }],
                                block: Block::ListItem,
                            });
                        }
                    }
                }
            }
            _ => {
                // no whitespace, treat as Paragraph
                // FIXME: no handle of softbreak
                let mut previous_str = String::from("-");
                previous_str.push(ch);
                return Node {
                    children: vec![_parse_text(&previous_str, char_iter)],
                    block: Block::Paragraph,
                };
            }
        }
    }
    todo!();
}

fn _parse_listitem(char_iter: &mut Chars<'_>) -> Node {
    Node {
        children: vec![_parse_para(char_iter)],
        block: Block::ListItem,
    }
}

fn _parse_text(previous_str: &str, char_iter: &mut Chars<'_>) -> Node {
    let mut content = String::from(previous_str);
    for ch in char_iter {
        match ch {
            '\n' => {
                break;
            }
            _ => {
                content.push(ch);
            }
        }
    }

    Node {
        children: vec![],
        block: Block::Text { content },
    }
}

fn _parse_softbreak(block: Block, previous_str: &str, char_iter: &mut Chars<'_>) -> Node {
    if let Block::Text { mut content } = block {
        content.push(' ');
        content.push_str(previous_str);
        for ch in char_iter {
            match ch {
                '\n' => {
                    break;
                }
                _ => {
                    content.push(ch);
                }
            }
        }
        Node {
            children: vec![],
            block: Block::Text { content },
        }
    } else {
        // FIXME: may need further consideration
        Node {
            children: vec![],
            block: Block::Text {
                content: String::new(),
            },
        }
    }
}
