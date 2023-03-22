# regex-tokenizer

![Alla pugna!](https://img.shields.io/badge/ALLA-PUGNA-F70808?style=for-the-badge)
[![Crates.io (latest)](https://img.shields.io/crates/dv/regex-tokenizer?label=CRATES.IO&style=for-the-badge)](https://crates.io/crates/regex-tokenizer)

A regex-based tokenizer with a minimal DSL to define it!

## Usage

```rust
tokenizer! {
    SimpleTokenizer

    r"[a-zA-Z]\w*" => Identifier
    r"\d+" => Number
    r"\s+" => _
}
```

And, in a function

```rust
...
let tokenizer = SimpleTokenizer::new();
...
```

`SimpleTokenizer` will generate an `enum` called `SimpleTokenyzer_types`, containing `Identifier` and `Number`. Regexes with `_` as class are ignored; when a substring that does not match a specified regex is found, the tokenization is considered failed.

When multiple non-ignored regexes match with an input, priority is given to the one defined first.

Calling `tokenizer.tokenize(...)` will return an iterator that extracts tokens from the query.
A token is formed by:

```rust
{
    value: String,
    position: usize,
    type_: SimpleTokenyzer_types,
}
```

`position` will be the position of the token's first character inside the query. A call to `.next()` will return `None` if there are no more tokens to extract.