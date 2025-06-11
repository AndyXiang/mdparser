use markdown_parser::tokenize::*;

#[test]
fn test_to_tokens() {
    assert_eq!(
        vec![
            Token::Block,
            Token::ATXHeader {
                level: 1,
                content: String::from("H1")
            },
            Token::Block
        ],
        to_tokens(String::from("# H1\n"))
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::ATXHeader {
                level: 1,
                content: String::from("H1")
            },
            Token::Block,
            Token::Text {
                content: String::from("here is a text.")
            },
            Token::Block,
        ],
        to_tokens(String::from("# H1\n here is a text.\n"))
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::ATXHeader {
                level: 1,
                content: String::from("H1")
            },
            Token::Block,
            Token::Text {
                content: String::from("here is a text.")
            },
            Token::Block,
            Token::ATXHeader {
                level: 2,
                content: String::from("H2")
            },
            Token::Block,
            Token::ATXHeader {
                level: 2,
                content: String::new(),
            },
            Token::Block,
        ],
        to_tokens(String::from("# H1\n\nhere is a text.\n\n## H2\n\n ##\n"))
    );
    assert_eq!(
        vec![Token::Block, Token::LeftAsterisk { count: 1 }, Token::Block,],
        to_tokens(String::from("*"))
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::LeftRightAsterisk { count: 1 },
            Token::Text {
                content: String::from("foo")
            },
            Token::Block,
        ],
        to_tokens(String::from("*foo"))
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::LeftRightAsterisk { count: 1 },
            Token::Text {
                content: String::from("foo")
            },
            Token::RightAsterisk { count: 1 },
            Token::Block,
        ],
        to_tokens(String::from("*foo*"))
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::Text {
                content: String::from("foo ")
            },
            Token::LeftAsterisk { count: 2 },
            Token::Block
        ],
        to_tokens(String::from("foo\n**"))
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::Text {
                content: String::from("foo ")
            },
            Token::Text {
                content: String::from("** foo")
            },
            Token::Block
        ],
        to_tokens(String::from("foo ** foo"))
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::Text {
                content: String::from("foo")
            },
            Token::LeftRightAsterisk { count: 2 },
            Token::Text {
                content: String::from("foo")
            },
            Token::Block
        ],
        to_tokens(String::from("foo**foo"))
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::LeftRightAsterisk { count: 1 },
            Token::Text {
                content: String::from("foo bar")
            },
            Token::RightAsterisk { count: 1 },
            Token::Block
        ],
        to_tokens(String::from("*foo bar*"))
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::Text {
                content: String::from("a ")
            },
            Token::Text {
                content: String::from("* foo bar")
            },
            Token::RightAsterisk { count: 1 },
            Token::Block
        ],
        to_tokens(String::from("a * foo bar*"))
    );
}
