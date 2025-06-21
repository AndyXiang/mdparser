use mdparser::ast::*;
use mdparser::lexer::*;

#[test]
fn test_to_ast() {
    assert_eq!(
        vec![Node::Block {
            children: Some(vec![Node::Emphasis {
                content: String::from("foo bar")
            }])
        }],
        to_ast(to_tokens(String::from("*foo bar*")))
    );
    assert_eq!(
        vec![Node::Block {
            children: Some(vec![Node::Strong {
                content: String::from("foo bar")
            }])
        }],
        to_ast(to_tokens(String::from("**foo bar**")))
    );
}
