use regex_tokenizer::tokenizer;

fn main() {
    tokenizer! {
        Test

        "a[1" => A
        "b" => B
        "c" => C
    }
}
