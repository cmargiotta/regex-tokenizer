use regex_tokenizer::tokenizer;

fn main() {
    tokenizer! {
        test

        "a[1" => A
        "b" => B
        "c" => C
    }
}
