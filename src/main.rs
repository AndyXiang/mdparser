#![allow(unused)]

use mdparser::ast::*;
use mdparser::html::to_html;
use mdparser::lexer::*;
use std::fs;

fn main() {
    let contents = fs::read_to_string("testfiles/note.md").expect("read file");

    let tokens = parse(&contents);
    println!("{:?}", tokens);
    let ast = to_ast(tokens);
    println!("{:?}", ast);
    let html = to_html(ast);

    fs::write("testfiles/note.html", html);
}
