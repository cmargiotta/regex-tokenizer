use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

pub(crate) fn error(span: Span, message: &str) -> TokenStream {
    [
        TokenTree::Ident(Ident::new("compile_error", span)),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            [TokenTree::Literal(Literal::string(message))]
                .into_iter()
                .collect(),
        )),
    ]
    .into_iter()
    .map(|mut t| {
        t.set_span(span);
        t
    })
    .collect()
}
