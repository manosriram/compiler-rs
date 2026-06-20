
// code -> tokens
// -> ast -> clean_ast
// -> 3ac
//
//
//

mod tokenizer;

fn hello(c: &str, x: i32) {
    println!("{} = {}", c, x);
}

fn main() {
    println!("Hello, world!");

    let a = 1234;
    let b = a;

    hello("a", a);
    hello("b", b);

    let _: Box<i32> = Box::new(5);


    let mut t = tokenizer::Tokenizer::new("source.l");
    t.lexer();
    t._show_tokens();
}
