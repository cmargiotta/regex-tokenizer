use regex_tokenizer::tokenizer;

fn main() {
    tokenizer! {
        Test

        r"\w[^\s]*" => Identifier
        r"\d+" => Number
        r"\s+" => _
    }
}
