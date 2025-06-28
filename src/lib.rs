#![allow(unused)]

pub mod lexer {
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

        // step the tokenizer when the input char is #
        fn _match_numbersign(&mut self) -> Option<Token> {
            let mut output_token: Option<Token> = None;
            let mut current_token = self.token.clone();
            match self.state {
                // If the # appears on the beginning of a block
                // then change tokenizer to ATXHeader state
                // push the block token and create a new ATXHeader token
                TokenizerState::Block => {
                    output_token = Some(current_token);
                    self.state = TokenizerState::ATXHeader;
                    self.token = Token::ATXHeader {
                        level: 1,
                        content: String::new(),
                    };
                }
                // If the tokenizer is currently in ATXHeader state
                // then add the header level
                TokenizerState::ATXHeader => {
                    if let Token::ATXHeader { level, content } = current_token {
                        if level <= 4 {
                            self.token = Token::ATXHeader {
                                level: level + 1,
                                content,
                            };
                        } else {
                            self.state = TokenizerState::Text;
                            self.token = Token::Text {
                                content: "#".repeat(level + 1),
                            }
                        }
                    }
                }
                // If the tokenizer is in Text state
                // then the # will be counted in the text string
                TokenizerState::Text => {
                    if let Token::Text { content } = current_token {
                        self.token = Token::Text {
                            content: content + "#",
                        };
                    }
                }
                // If the tokenizer is in ATXHeaderContent state
                // then the # will be counted in the header content string
                TokenizerState::ATXHeaderContent => {
                    if let Token::ATXHeader { level, content } = current_token {
                        self.token = Token::ATXHeader {
                            level,
                            content: content + "#",
                        };
                    }
                }
                // If the tokenizer is in LazyContinuation state
                // then the # will be considered as the continuation of string
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
                // If the tokenizer is in CountAsterisk state
                // then the # will be considered as a char and stop counting asterisk
                // and push the asterisk token
                TokenizerState::CountAsterisk => {
                    self.state = TokenizerState::Text;
                    output_token = Some(current_token);
                    self.token = Token::Text {
                        content: String::from("#"),
                    };
                }
                // similar to asterisk situation
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
                // the space on the head of a new line is ignored
                TokenizerState::Block => (),
                TokenizerState::LazyContinuation => (),
                // A space behind a ATXHeader is considered as the stop of
                // counting levels of the header.
                // The following string is the content of the header.
                TokenizerState::ATXHeader => {
                    self.state = TokenizerState::ATXHeaderContent;
                }
                // A space in Text state is a normal space char
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
                // Each delimiter can not be followed by space,
                // then the asterisks or underscores ahead are considered as normal char
                TokenizerState::CountAsterisk => {
                    self.state = TokenizerState::Text;
                    match current_token {
                        Token::RightAsterisk { count } => {
                            output_token = Some(Token::Text {
                                content: "*".repeat(count),
                            });
                            self.token = Token::Text {
                                content: String::from(" "),
                            };
                        }
                        Token::LeftAsterisk { count } => {
                            output_token = Some(Token::Text {
                                content: "*".repeat(count),
                            });
                            self.token = Token::Text {
                                content: String::from(" "),
                            };
                        }
                        _ => (),
                    }
                }
                TokenizerState::CountUnderscore => {
                    self.state = TokenizerState::Text;
                    match current_token {
                        Token::RightUnderscore { count } => {
                            output_token = Some(Token::Text {
                                content: "_".repeat(count),
                            });
                            self.token = Token::Text {
                                content: String::from(" "),
                            };
                        }
                        Token::LeftUnderscore { count } => {
                            output_token = Some(Token::Text {
                                content: "_".repeat(count),
                            });
                            self.token = Token::Text {
                                content: String::from(" "),
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
                // multiple line breaks are ignored
                TokenizerState::Block => (),
                // single line break is ignored
                // multiple line breaks ends the last block and start a new block.
                TokenizerState::Text => {
                    self.state = TokenizerState::LazyContinuation;
                }
                TokenizerState::LazyContinuation => {
                    self.state = TokenizerState::Block;
                    self.token = Token::Block;
                    output_token = Some(current_token);
                }
                // single line break ends the ATXHeader.
                TokenizerState::ATXHeaderContent => {
                    self.state = TokenizerState::Block;
                    self.token = Token::Block;
                    output_token = Some(current_token);
                }
                // header with no content is allowed
                TokenizerState::ATXHeader => {
                    self.state = TokenizerState::Block;
                    self.token = Token::Block;
                    output_token = Some(current_token);
                }
                // asterisks and underscores end with line break are considered
                // as normal string rather then key words
                TokenizerState::CountAsterisk => match current_token {
                    Token::RightAsterisk { count } => {
                        self.state = TokenizerState::LazyContinuation;
                        self.token = Token::Text {
                            content: "*".repeat(count),
                        };
                    }
                    Token::LeftAsterisk { count } => {
                        if count < 3 {
                            self.state = TokenizerState::LazyContinuation;
                            self.token = Token::Text {
                                content: "*".repeat(count),
                            };
                        }
                    }
                    _ => (),
                },
                TokenizerState::CountUnderscore => match current_token {
                    Token::RightUnderscore { count } => {
                        self.state = TokenizerState::LazyContinuation;
                        self.token = Token::Text {
                            content: "_".repeat(count),
                        };
                    }
                    Token::LeftUnderscore { count } => {
                        if count < 3 {
                            self.state = TokenizerState::LazyContinuation;
                            self.token = Token::Text {
                                content: "_".repeat(count),
                            };
                        }
                    }
                    _ => (),
                },
            }
            output_token
        }

        // this function should be replace with a more specific function
        // for unicode alphanumeric characters.
        fn _match_letter(&mut self, c: &char) -> Option<Token> {
            let mut output_token: Option<Token> = None;
            let mut current_token = self.token.clone();
            match self.state {
                // a char in Block state is the beginning of a paragraph
                TokenizerState::Block => {
                    self.state = TokenizerState::Text;
                    self.token = Token::Text {
                        content: String::from(*c),
                    };
                    output_token = Some(current_token);
                }
                // a space between the #s and content is required
                // thus the # followed with char is considered as normal char
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
                // char in ATXHeaderContent or Text state extends the string
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
                // char in LazyContinuation state restarts the paragraph
                TokenizerState::LazyContinuation => {
                    if let Token::Text { mut content } = current_token {
                        content.push(' ');
                        content.push(*c);
                        self.token = Token::Text { content };
                    }
                }
                // delimiters should ends with unicode characters
                // more specific condition should be added
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

        // more specific conditions are required
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

        // more specific conditions are required
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

    /// `parse` transfers input string (read from file) to internal tokens
    pub fn parse(markdown_text: &str) -> Vec<Token> {
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
    use std::fmt;
    use std::fmt::{Debug, Display, Formatter};
    use std::{option::IntoIter, vec};

    use super::lexer::Token;

    const ASTERISK: &str = "asterisk";
    const UNDERSCORE: &str = "underscore";

    #[derive(Debug, Clone, PartialEq)]
    pub enum Node {
        Document,
        Block { children: Option<Vec<Node>> },
        Text { content: String },
        ATXHeader { level: usize, content: String },
        Emphasis { content: String },
        Strong { content: String },
        ThematicBreak,
    }

    impl Display for Node {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            self.fmt_with_indent(f, 0)
        }
    }

    impl Node {
        fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
            match self {
                Node::Document => writeln!(f, "<document>")?,
                Node::Block { children } => {
                    write!(f, "{:indent$}", "", indent = indent * 2)?;
                    writeln!(f, "<block>");
                    if let Some(children) = children {
                        for child in children {
                            child.fmt_with_indent(f, indent + 1)?;
                        }
                    }
                    write!(f, "{:indent$}", "", indent = indent * 2)?;
                    writeln!(f, "</block>");
                }
                Node::ThematicBreak => {
                    write!(f, "{:indent$}", "", indent = indent * 2)?;
                    writeln!(f, "<thematic_break />")?;
                }
                Node::Strong { content } => {
                    write!(f, "{:indent$}", "", indent = indent * 2)?;
                    writeln!(f, "<strong>")?;
                    write!(f, "{:indent$}", "", indent = (indent + 1) * 2)?;
                    writeln!(f, "<text>{content}</text>")?;
                    write!(f, "{:indent$}", "", indent = indent * 2)?;
                    writeln!(f, "</strong>")?;
                }
                Node::Emphasis { content } => {
                    write!(f, "{:indent$}", "", indent = indent * 2)?;
                    writeln!(f, "<emph>")?;
                    write!(f, "{:indent$}", "", indent = (indent + 1) * 2)?;
                    writeln!(f, "<text>{content}</text>")?;
                    write!(f, "{:indent$}", "", indent = indent * 2)?;
                    writeln!(f, "</emph>")?;
                }
                Node::Text { content } => {
                    write!(f, "{:indent$}", "", indent = indent * 2)?;
                    writeln!(f, "<text>{content}</text>")?;
                }
                Node::ATXHeader { level, content } => {
                    write!(f, "{:indent$}", "", indent = indent * 2)?;
                    writeln!(f, "<heading level=\"{level}\">")?;
                    write!(f, "{:indent$}", "", indent = (indent + 1) * 2)?;
                    writeln!(f, "<text>{content}</text>")?;
                    write!(f, "{:indent$}", "", indent = indent * 2)?;
                    writeln!(f, "</heading>")?;
                }
                _ => (),
            }
            Ok(())
        }
    }

    pub struct ASTCreator {
        ast: Vec<Node>,
        current_block: Node,
    }

    impl fmt::Display for ASTCreator {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for node in &self.ast {
                node.fmt_with_indent(f, 0)?;
            }
            Ok(())
        }
    }

    impl Default for ASTCreator {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ASTCreator {
        pub fn new() -> Self {
            Self {
                ast: vec![],
                current_block: Node::Document,
            }
        }

        // call this function when encounter a Block Token
        // push the current_node and create new block node
        // if document is empty, do not push
        fn _accept_block(&mut self) {
            if let Node::Document = self.current_block {
                self.current_block = Node::Block {
                    children: Some(vec![]),
                }
            } else {
                // FIXME using clone bad for performance
                self.ast.push(self.current_block.clone());
                self.current_block = Node::Block {
                    children: Some(vec![]),
                }
            }
        }

        fn _accept_atxheader(&mut self, level: &usize, content: &str) {
            if let Node::Block { ref mut children } = self.current_block {
                let mut nodes = children.get_or_insert(vec![]);
                nodes.push(Node::ATXHeader {
                    level: *level,
                    content: String::from(content),
                });
            }
        }

        fn _accept_text(&mut self, content: &str) {
            if let Node::Block { ref mut children } = self.current_block {
                let mut nodes = children.get_or_insert(vec![]);
                nodes.push(Node::Text {
                    content: String::from(content),
                });
            }
        }

        fn _accept_emph(&mut self, from_which: &str, iter: &mut std::vec::IntoIter<Token>) {
            let mut content = String::new();
            let mut is_closed = false;
            for token in iter.by_ref() {
                match token {
                    Token::Text {
                        content: emph_content,
                    } => {
                        content += &emph_content;
                    }
                    Token::RightAsterisk { count: c } | Token::LeftRightAsterisk { count: c } => {
                        if ASTERISK == from_which && c == 1 {
                            is_closed = true;
                        }
                        break;
                    }
                    Token::RightUnderscore { count: c }
                    | Token::LeftRightUnderscore { count: c } => {
                        if UNDERSCORE == from_which && c == 1 {
                            is_closed = true;
                        }
                        break;
                    }
                    Token::ATXHeader { .. }
                    | Token::Block
                    | Token::LeftAsterisk { .. }
                    | Token::LeftUnderscore { .. } => break,
                }
            }
            if let Node::Block { ref mut children } = self.current_block {
                if is_closed {
                    let mut nodes = children.get_or_insert(vec![]);
                    nodes.push(Node::Emphasis { content });
                } else {
                    iter.next_back();
                    let mut nodes = children.get_or_insert(vec![]);
                    match from_which {
                        ASTERISK => {
                            nodes.push(Node::Text {
                                content: String::from("*"),
                            });
                        }
                        UNDERSCORE => {
                            nodes.push(Node::Text {
                                content: String::from("_"),
                            });
                        }
                        _ => (),
                    }
                    nodes.push(Node::Text { content });
                }
            }
        }

        fn _accept_strong(&mut self, from_which: &str, iter: &mut std::vec::IntoIter<Token>) {
            let mut content = String::new();
            let mut is_closed = false;
            for token in iter.by_ref() {
                match token {
                    Token::Text {
                        content: emph_content,
                    } => {
                        content += &emph_content;
                    }
                    Token::RightAsterisk { count: c } | Token::LeftRightAsterisk { count: c } => {
                        if ASTERISK == from_which && c == 2 {
                            is_closed = true;
                        }
                        break;
                    }
                    Token::RightUnderscore { count: c }
                    | Token::LeftRightUnderscore { count: c } => {
                        if UNDERSCORE == from_which && c == 2 {
                            is_closed = true;
                        }
                        break;
                    }
                    Token::ATXHeader { .. }
                    | Token::Block
                    | Token::LeftAsterisk { .. }
                    | Token::LeftUnderscore { .. } => break,
                }
            }
            if let Node::Block { ref mut children } = self.current_block {
                if is_closed {
                    let mut nodes = children.get_or_insert(vec![]);
                    nodes.push(Node::Strong { content });
                } else {
                    iter.next_back();
                    let mut nodes = children.get_or_insert(vec![]);
                    match from_which {
                        ASTERISK => {
                            nodes.push(Node::Text {
                                content: String::from("**"),
                            });
                        }
                        UNDERSCORE => {
                            nodes.push(Node::Text {
                                content: String::from("__"),
                            });
                        }
                        _ => (),
                    }
                    nodes.push(Node::Text { content });
                }
            }
        }

        fn _accept_thm(&self) {
            todo!();
        }
    }

    pub fn to_ast(tokens: Vec<Token>) -> Vec<Node> {
        let mut current_node = Node::Block {
            children: Some(vec![]),
        };
        let mut ast = ASTCreator::new();
        let mut token_iter = tokens.into_iter();

        while let Some(token) = token_iter.next() {
            match token {
                Token::Block => ast._accept_block(),
                Token::Text { ref content } => ast._accept_text(content),
                Token::ATXHeader {
                    ref level,
                    ref content,
                } => ast._accept_atxheader(level, content),
                Token::LeftRightAsterisk { count } | Token::LeftAsterisk { count } => match count {
                    0 => (),
                    1 => ast._accept_emph(ASTERISK, &mut token_iter),
                    2 => ast._accept_strong(ASTERISK, &mut token_iter),
                    3 => ast._accept_thm(),
                    4_usize.. => ast._accept_text(&"*".repeat(count)),
                },
                Token::RightAsterisk { count } => match count {
                    3 => ast._accept_thm(),
                    4_usize.. => ast._accept_text(&"*".repeat(count)),
                    _ => (),
                },
                Token::LeftRightUnderscore { count } | Token::LeftUnderscore { count } => {
                    match count {
                        0 => (),
                        1 => ast._accept_emph(UNDERSCORE, &mut token_iter),
                        2 => ast._accept_strong(UNDERSCORE, &mut token_iter),
                        3 => ast._accept_thm(),
                        4_usize.. => ast._accept_text(&"*".repeat(count)),
                    }
                }
                Token::RightUnderscore { count } => match count {
                    3 => ast._accept_thm(),
                    4_usize.. => ast._accept_text(&"*".repeat(count)),
                    _ => (),
                },
            }
        }

        ast.ast
    }
}

pub mod html {
    use super::ast::Node;
    use indoc::indoc;

    pub fn to_html(ast: Vec<Node>) -> String {
        let mut html = String::from(indoc! {"
            <!DOCTYPE html>
            <html lang=\"en\">
            <head>
                <meta charset\"UTF-8\">
                <meta name=\"viewprot\" content=\"width=device-width, initial-scale=1.0\">
                <title>Document</title>
            </head>
            <body>\n
        "});

        for block in ast {
            if let Node::Block {
                children: Some(children),
            } = block
            {
                for child in &children {
                    match child {
                        Node::ATXHeader { level, content } => {
                            html += _visit_atxheader(level, content).as_str()
                        }
                        Node::ThematicBreak => html += _visit_thm().as_str(),
                        Node::Strong { content } => html += _visit_strong(content).as_str(),
                        Node::Text { content } => html += _visit_text(content).as_str(),
                        Node::Emphasis { content } => html += _visit_emph(content).as_str(),
                        _ => (),
                    };
                }
            }
        }
        html += "\n</body>";
        html
    }

    fn _visit_atxheader(level: &usize, content: &str) -> String {
        format!("<h{}>{}</h{}>\n", level, content, level)
    }

    fn _visit_thm() -> String {
        String::from("<hr />")
    }

    fn _visit_emph(content: &str) -> String {
        format!("<em>{}</em>", content)
    }

    fn _visit_strong(content: &str) -> String {
        format!("<strong>{}</strong>", content)
    }

    fn _visit_text(content: &str) -> String {
        String::from(content)
    }
}
