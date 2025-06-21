#![allow(unused)]

use mdparser::ast::*;
// use mdparser::html::to_html;
use mdparser::lexer::*;
use std::fs;

fn main() {
    let contents = fs::read_to_string("tests/test.md").expect("read file");

    let tokens = to_tokens(contents);
    let ast = to_ast(tokens);
    // let html = to_html(ast);

    // fs::write("tests/test.html", html);
}
