mod tokenizer;

fn main() {
    let mut t = tokenizer::Tokenizer::new("source.l");
    t.tokenize();
    t.show_tokens();
}
