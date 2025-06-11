#![allow(unused)]

const PUNCTUATION: [char; 26] = [
    ',', '.', '/', '<', '>', '?', ';', ':', '\'', '\"', '{', '}', '\\', '|', '!', '@', '#', '$',
    '%', '^', '&', '(', ')', '-', '+', '=',
];

const ALPHABETIC: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const NUMERIC: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const ALPHANUMERIC: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

pub mod tokenize {
    use std::process::Output;

    #[derive(Debug, PartialEq, Clone)]
    pub enum Token {
        ATXHeader { level: usize, content: String },
        Text { content: String },
        LeftAsterisk { count: usize },
        RightAsterisk { count: usize },
        LeftRightAsterisk { count: usize },
        LeftUnderscore { count: usize },
        RightUnderscore { count: usize },
        LeftRightUnderscore { count: usize },
        ThematicBreak,
        Block,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum TokenizerState {
        Block,
        ATXHeader,
        ATXHeaderContent,
        Text,
        LazyContinuation,
        CountAsterisk,
        CountUnderscore,
    }

    #[derive(Debug)]
    pub struct Tokenizer {
        token: Token,
        state: TokenizerState,
    }

    impl Tokenizer {
        pub fn new() -> Self {
            Tokenizer {
                token: Token::Block,
                state: TokenizerState::Block,
            }
        }
        pub fn step(&mut self, c: Option<char>) -> Option<Token> {
            match c {
                Some('#') => self._match_numbersign(),
                Some(' ') => self._match_space(),
                Some('\n') => self._match_linebreak(),
                None => self._match_none(),
                Some('*') => self._match_asterisk(),
                Some('_') => self._match_underscore(),
                _ => self._match_letter(&c.unwrap()),
            }
        }

        fn _match_numbersign(&mut self) -> Option<Token> {
            let mut output_token: Option<Token> = None;
            let mut current_token = self.token.clone();
            match self.state {
                TokenizerState::Block => {
                    output_token = Some(current_token);
                    self.state = TokenizerState::ATXHeader;
                    self.token = Token::ATXHeader {
                        level: 1,
                        content: String::new(),
                    };
                }
                TokenizerState::ATXHeader => {
                    if let Token::ATXHeader { level, content } = current_token {
                        self.token = Token::ATXHeader {
                            level: level + 1,
                            content,
                        };
                    }
                }
                TokenizerState::Text => {
                    if let Token::Text { content } = current_token {
                        self.token = Token::Text {
                            content: content + "#",
                        };
                    }
                }
                TokenizerState::ATXHeaderContent => {
                    if let Token::ATXHeader { level, content } = current_token {
                        self.token = Token::ATXHeader {
                            level,
                            content: content + "#",
                        };
                    }
                }
                TokenizerState::LazyContinuation => {
                    self.state = TokenizerState::ATXHeader;
                    if let Token::Text { mut content } = current_token {
                        content.pop();
                        output_token = Some(Token::Text { content });
                    }
                    self.token = Token::ATXHeader {
                        level: 1,
                        content: String::new(),
                    };
                }
                TokenizerState::CountAsterisk => {
                    self.state = TokenizerState::Text;
                    output_token = Some(current_token);
                    self.token = Token::Text {
                        content: String::from("#"),
                    };
                }
                TokenizerState::CountUnderscore => {
                    self.state = TokenizerState::Text;
                    output_token = Some(current_token);
                    self.token = Token::Text {
                        content: String::from("#"),
                    };
                }
            }
            output_token
        }

        fn _match_space(&mut self) -> Option<Token> {
            let mut output_token: Option<Token> = None;
            let mut current_token = self.token.clone();
            match self.state {
                TokenizerState::Block => (),
                TokenizerState::LazyContinuation => (),
                TokenizerState::ATXHeader => {
                    self.state = TokenizerState::ATXHeaderContent;
                }
                TokenizerState::Text => {
                    if let Token::Text { mut content } = current_token {
                        self.token = Token::Text {
                            content: content + " ",
                        };
                    }
                }
                TokenizerState::ATXHeaderContent => {
                    if let Token::ATXHeader { level, content } = current_token {
                        self.token = Token::ATXHeader {
                            level,
                            content: content + " ",
                        };
                    }
                }
                TokenizerState::CountAsterisk => {
                    self.state = TokenizerState::Text;
                    match current_token {
                        Token::RightAsterisk { count } => {
                            self.token = Token::Text {
                                content: "*".repeat(count) + " ",
                            };
                        }
                        Token::LeftAsterisk { count } => {
                            self.token = Token::Text {
                                content: "*".repeat(count) + " ",
                            };
                        }
                        _ => (),
                    }
                }
                TokenizerState::CountUnderscore => {
                    self.state = TokenizerState::Text;
                    match current_token {
                        Token::RightUnderscore { count } => {
                            self.token = Token::Text {
                                content: "_".repeat(count) + " ",
                            };
                        }
                        Token::LeftUnderscore { count } => {
                            self.token = Token::Text {
                                content: "_".repeat(count) + " ",
                            };
                        }
                        Token::LeftRightUnderscore { count } => {
                            self.token = Token::Text {
                                content: "_".repeat(count),
                            };
                        }
                        _ => (),
                    }
                }
            }
            output_token
        }

        fn _match_linebreak(&mut self) -> Option<Token> {
            let mut output_token: Option<Token> = None;
            let mut current_token = self.token.clone();
            match self.state {
                TokenizerState::Block => (),
                TokenizerState::Text => {
                    self.state = TokenizerState::LazyContinuation;
                }
                TokenizerState::LazyContinuation => {
                    self.state = TokenizerState::Block;
                    self.token = Token::Block;
                    output_token = Some(current_token);
                }
                TokenizerState::ATXHeaderContent => {
                    self.state = TokenizerState::Block;
                    self.token = Token::Block;
                    output_token = Some(current_token);
                }
                TokenizerState::ATXHeader => {
                    self.state = TokenizerState::Block;
                    self.token = Token::Block;
                    output_token = Some(current_token);
                }
                TokenizerState::CountAsterisk => {
                    self.state = TokenizerState::LazyContinuation;
                    match current_token {
                        Token::RightAsterisk { count } => {
                            self.token = Token::Text {
                                content: "*".repeat(count),
                            };
                        }
                        Token::LeftAsterisk { count } => {
                            self.token = Token::Text {
                                content: "*".repeat(count),
                            };
                        }
                        Token::LeftRightAsterisk { count } => {
                            self.token = Token::Text {
                                content: "*".repeat(count),
                            };
                        }
                        _ => (),
                    }
                }
                TokenizerState::CountUnderscore => {
                    self.state = TokenizerState::LazyContinuation;
                    match current_token {
                        Token::RightUnderscore { count } => {
                            self.token = Token::Text {
                                content: "_".repeat(count),
                            };
                        }
                        Token::LeftUnderscore { count } => {
                            self.token = Token::Text {
                                content: "_".repeat(count),
                            };
                        }
                        Token::LeftRightUnderscore { count } => {
                            self.token = Token::Text {
                                content: "_".repeat(count),
                            };
                        }
                        _ => (),
                    }
                }
            }
            output_token
        }

        fn _match_letter(&mut self, c: &char) -> Option<Token> {
            let mut output_token: Option<Token> = None;
            let mut current_token = self.token.clone();
            match self.state {
                TokenizerState::Block => {
                    self.state = TokenizerState::Text;
                    self.token = Token::Text {
                        content: String::from(*c),
                    };
                    output_token = Some(current_token);
                }
                TokenizerState::ATXHeader => {
                    self.state = TokenizerState::Text;
                    if let Token::ATXHeader { level, content } = current_token {
                        let mut new_content = "#".repeat(level);
                        new_content.push(*c);
                        self.token = Token::Text {
                            content: new_content,
                        };
                    }
                }
                TokenizerState::ATXHeaderContent => {
                    if let Token::ATXHeader { level, mut content } = current_token {
                        content.push(*c);
                        self.token = Token::ATXHeader { level, content };
                    }
                }
                TokenizerState::Text => {
                    if let Token::Text { mut content } = current_token {
                        content.push(*c);
                        self.token = Token::Text { content };
                    }
                }
                TokenizerState::LazyContinuation => {
                    if let Token::Text { mut content } = current_token {
                        content.push(' ');
                        content.push(*c);
                        self.token = Token::Text { content };
                    }
                }
                TokenizerState::CountAsterisk => {
                    self.state = TokenizerState::Text;
                    self.token = Token::Text {
                        content: String::from(*c),
                    };
                    match current_token {
                        Token::LeftAsterisk { count } => {
                            output_token = Some(Token::LeftRightAsterisk { count });
                        }
                        Token::RightAsterisk { count } => {
                            output_token = Some(Token::LeftRightAsterisk { count });
                        }
                        _ => (),
                    }
                }
                TokenizerState::CountUnderscore => {
                    self.state = TokenizerState::Text;
                    self.token = Token::Text {
                        content: String::from(*c),
                    };
                    match current_token {
                        Token::LeftUnderscore { count } => {
                            output_token = Some(Token::LeftRightUnderscore { count });
                        }
                        Token::RightUnderscore { count } => {
                            output_token = Some(Token::LeftRightUnderscore { count });
                        }
                        _ => (),
                    }
                }
            }
            output_token
        }

        fn _match_none(&mut self) -> Option<Token> {
            Some(self.token.clone())
        }

        fn _match_asterisk(&mut self) -> Option<Token> {
            let mut output_token: Option<Token> = None;
            let mut current_token = self.token.clone();
            match self.state {
                TokenizerState::Block => {
                    self.state = TokenizerState::CountAsterisk;
                    self.token = Token::LeftAsterisk { count: 1 };
                    output_token = Some(current_token);
                }
                TokenizerState::Text => {
                    output_token = Some(current_token.clone());
                    if let Token::Text { mut content } = current_token {
                        let last_char = content.pop().unwrap();
                        if let ' ' = last_char {
                            self.token = Token::LeftAsterisk { count: 1 };
                        } else {
                            self.token = Token::RightAsterisk { count: 1 };
                        }
                    }
                    self.state = TokenizerState::CountAsterisk;
                }
                TokenizerState::LazyContinuation => {
                    self.state = TokenizerState::CountAsterisk;
                    self.token = Token::LeftAsterisk { count: 1 };
                    if let Token::Text { content } = current_token {
                        output_token = Some(Token::Text {
                            content: content + " ",
                        });
                    }
                }
                TokenizerState::ATXHeader => {
                    self.state = TokenizerState::CountAsterisk;
                    self.token = Token::LeftAsterisk { count: 1 };
                    if let Token::ATXHeader { level, content } = current_token {
                        output_token = Some(Token::Text {
                            content: "#".repeat(level),
                        });
                    }
                }
                TokenizerState::ATXHeaderContent => {
                    self.state = TokenizerState::CountAsterisk;
                    self.token = Token::LeftAsterisk { count: 1 };
                    output_token = Some(current_token);
                }
                TokenizerState::CountAsterisk => match current_token {
                    Token::LeftAsterisk { count } => {
                        self.token = Token::LeftAsterisk { count: count + 1 };
                    }
                    Token::RightAsterisk { count } => {
                        self.token = Token::RightAsterisk { count: count + 1 };
                    }
                    Token::LeftRightAsterisk { count } => {
                        self.token = Token::LeftRightAsterisk { count: count + 1 };
                    }
                    _ => (),
                },
                TokenizerState::CountUnderscore => {
                    self.state = TokenizerState::CountAsterisk;
                    self.token = Token::LeftAsterisk { count: 1 };
                    output_token = Some(current_token);
                }
            }
            output_token
        }

        fn _match_underscore(&mut self) -> Option<Token> {
            let mut output_token: Option<Token> = None;
            let mut current_token = self.token.clone();
            match self.state {
                TokenizerState::Block => {
                    self.state = TokenizerState::CountUnderscore;
                    self.token = Token::LeftUnderscore { count: 1 };
                    output_token = Some(current_token);
                }
                TokenizerState::Text => {
                    output_token = Some(current_token);
                    self.state = TokenizerState::CountUnderscore;
                    self.token = Token::LeftUnderscore { count: 1 };
                }
                TokenizerState::LazyContinuation => {
                    self.state = TokenizerState::CountUnderscore;
                    self.token = Token::LeftUnderscore { count: 1 };
                    if let Token::Text { content } = current_token {
                        output_token = Some(Token::Text {
                            content: content + " ",
                        });
                    }
                }
                TokenizerState::ATXHeader => {
                    self.state = TokenizerState::CountUnderscore;
                    self.token = Token::LeftUnderscore { count: 1 };
                    if let Token::ATXHeader { level, content } = current_token {
                        output_token = Some(Token::Text {
                            content: "#".repeat(level),
                        });
                    }
                }
                TokenizerState::ATXHeaderContent => {
                    self.state = TokenizerState::CountUnderscore;
                    self.token = Token::LeftUnderscore { count: 1 };
                    output_token = Some(current_token);
                }
                TokenizerState::CountUnderscore => match current_token {
                    Token::LeftUnderscore { count } => {
                        self.token = Token::LeftUnderscore { count: count + 1 };
                    }
                    Token::RightUnderscore { count } => {
                        self.token = Token::RightUnderscore { count: count + 1 };
                    }
                    Token::LeftRightUnderscore { count } => {
                        self.token = Token::LeftRightUnderscore { count: count + 1 };
                    }
                    _ => (),
                },
                TokenizerState::CountAsterisk => {
                    self.state = TokenizerState::CountUnderscore;
                    self.token = Token::LeftUnderscore { count: 1 };
                    output_token = Some(current_token);
                }
            }
            output_token
        }
    }

    impl Default for Tokenizer {
        fn default() -> Self {
            Self::new()
        }
    }

    pub fn to_tokens(markdown_text: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut tokenizer = Tokenizer::new();

        let source: Vec<char> = markdown_text.chars().collect();

        for c in source {
            let output_token = tokenizer.step(Some(c));

            if let Some(t) = output_token {
                tokens.push(t);
            }
        }

        let output_token = tokenizer.step(None);

        if let Some(t) = output_token {
            match t {
                Token::Block => tokens.push(Token::Block),
                _ => {
                    tokens.push(t);
                    tokens.push(Token::Block);
                }
            }
        }

        tokens
    }

    pub trait Error {}
}

pub mod ast {
    use super::tokenize::Token;

    #[derive(Debug)]
    pub enum Node {
        Document {
            children: Vec<Node>,
        },
        Block {
            children: Vec<Node>,
        },
        Paragraph {
            children: Vec<Node>,
        },
        Header {
            level: usize,
            children: Vec<Node>,
        },
        Text {
            content: String,
            children: Vec<Node>,
        },
    }

    pub fn to_ast(tokens: Vec<Token>) -> Node {
        let mut document = Node::Document { children: vec![] };
        let mut current_block = Node::Block { children: vec![] };

        for token in tokens {
            match token {
                Token::Block => {
                    if let Node::Document { mut children } = document {
                        children.push(current_block);
                        document = Node::Document { children };
                        current_block = Node::Block { children: vec![] };
                    }
                }
                Token::Text { content } => {
                    if let Node::Block { mut children } = current_block {
                        children.push(Node::Paragraph {
                            children: vec![Node::Text {
                                content,
                                children: vec![],
                            }],
                        });
                        current_block = Node::Block { children };
                    }
                }
                Token::ATXHeader { level, content } => {
                    if let Node::Block { mut children } = current_block {
                        children.push(Node::Header {
                            level,
                            children: vec![Node::Text {
                                content,
                                children: vec![],
                            }],
                        });
                        current_block = Node::Block { children };
                    }
                }
                _ => (),
            }
        }

        if let Node::Document { mut children } = document {
            children.remove(0);
            document = Node::Document { children };
        } else {
            document = Node::Document { children: vec![] };
        };

        document
    }
}

pub mod html {
    use super::ast::Node;

    pub fn to_html(ast: Node) -> String {
        let mut html = String::from(
            "<!DOCTYPE html>
<html lang=\"en\">
<head>
\t<meta charset\"UTF-8\">
\t<meta http-equiv=\"X-UA-Compatible\" content=\"IE-edge\">
\t<meta name=\"viewprot\" content=\"width=device-width, initial-scale=1.0\">
\t<title>Document</title>
</head>
<body>\n",
        );

        if let Node::Document { children: root } = ast {
            for block in root {
                if let Node::Block { children } = block {
                    for node in children {
                        match node {
                            Node::Header {
                                level,
                                children: h_child,
                            } => {
                                for node in h_child {
                                    if let Node::Text {
                                        content,
                                        children: _,
                                    } = node
                                    {
                                        html += &format!("<h{level}>{content}</h{level}>\n");
                                    }
                                }
                            }
                            Node::Paragraph { children: p_child } => {
                                for node in p_child {
                                    if let Node::Text {
                                        content,
                                        children: _,
                                    } = node
                                    {
                                        html += &format!("<p>{content}</p>\n");
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
        }

        html += "\n</body>";

        html
    }
}
