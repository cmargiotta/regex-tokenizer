use regex_tokenizer::tokenizer;

tokenizer! {
    Test

    r"[a-zA-Z]\w*" => Identifier
    r"\d+" => Number
    r"\s+" => _
}

fn main() {
    let query = "Identifier  11";
    let tokenizer = Test::new();

    let mut tokens = tokenizer.tokenize(query);

    let token = tokens.next().unwrap();

    assert_eq!(token.position, 0);
    assert_eq!(token.value, "Identifier");
    assert_eq!(token.type_, Test_types::Identifier);

    let token = tokens.next().unwrap();

    assert_eq!(token.position, 12);
    assert_eq!(token.value, "11");
    assert_eq!(token.type_, Test_types::Number);
}
