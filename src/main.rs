mod tokenizer;

fn main() {
    let mut t = tokenizer::Tokenizer::new("source.l");
    t.lexer();
    t._show_tokens();
}
