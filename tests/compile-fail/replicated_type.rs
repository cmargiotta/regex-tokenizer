use regex_tokenizer::tokenizer;

fn main() {
    tokenizer! {
        Test

        "a" => A
        "b" => A
        "c" => C
    }
}
