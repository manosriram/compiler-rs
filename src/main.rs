use comp::{ast::Literal, symboltable, tiny::Tiny};

fn main() {
    let _program = Tiny::new("source.l").compile();

    // let mut s = symboltable::SymbolTable::new();
    // s.enter_scope();
    // s.define(
    //     String::from("one"),
    //     Literal::String(String::from("hahahah")),
    // );

    // println!("{:?}", s.lookup("one").unwrap());
}
