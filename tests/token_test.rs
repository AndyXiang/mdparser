use indoc::indoc;
use mdparser::lexer::*;

#[test]
fn test_parse() {
    assert_eq!(
        vec![Token::Block, Token::LeftAsterisk { count: 3 }, Token::Block,],
        parse(indoc! {"***"})
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::LeftUnderscore { count: 3 },
            Token::Block,
        ],
        parse(indoc! {"___"})
    );
    assert_eq!(
        vec![Token::Block, Token::LeftAsterisk { count: 2 }, Token::Block],
        parse(indoc! {"**"})
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::LeftUnderscore { count: 2 },
            Token::Block
        ],
        parse(indoc! {"__"})
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::LeftUnderscore { count: 5 },
            Token::Block
        ],
        parse(indoc! {"_____"})
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::Text {
                content: String::from("Foo ")
            },
            Token::LeftRightAsterisk { count: 3 },
            Token::Text {
                content: String::from("Bar")
            },
            Token::Block
        ],
        parse(indoc! {"
            Foo
            ***
            Bar
        "})
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::ATXHeader {
                level: 1,
                content: String::from("foo")
            },
            Token::Block,
            Token::ATXHeader {
                level: 2,
                content: String::from("foo")
            },
            Token::Block,
            Token::ATXHeader {
                level: 3,
                content: String::from("foo")
            },
            Token::Block,
            Token::ATXHeader {
                level: 4,
                content: String::from("foo")
            },
            Token::Block,
            Token::ATXHeader {
                level: 5,
                content: String::from("foo")
            },
            Token::Block,
        ],
        parse(indoc! {"
            # foo
            ## foo
            ### foo
            #### foo
            ##### foo
        "})
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::Text {
                content: String::from("###### foo")
            },
            Token::Block
        ],
        parse(indoc! {"###### foo"})
    );
    assert_eq!(
        vec![
            Token::Block,
            Token::ATXHeader {
                level: 1,
                content: String::from("foo ")
            },
            Token::LeftRightAsterisk { count: 1 },
            Token::Text {
                content: String::from("a")
            },
            Token::RightAsterisk { count: 1 },
            Token::Block,
        ],
        parse(indoc! {"# foo *a*"})
    )
}
