use proc_macro2::{Delimiter, Literal, TokenStream, TokenTree};

// #[doc = "..."] -> "..."
fn lit_of_doc_comment(tokens: TokenStream) -> Literal {
    let mut iter = tokens.clone().into_iter();
    match iter.next().unwrap() {
        TokenTree::Punct(punct) => assert_eq!(punct.as_char(), '#'),
        _ => panic!("wrong token {:?}", tokens),
    }
    iter = match iter.next().unwrap() {
        TokenTree::Group(group) => {
            assert_eq!(group.delimiter(), Delimiter::Bracket);
            assert!(iter.next().is_none(), "unexpected token {:?}", tokens);
            group.stream().into_iter()
        }
        _ => panic!("wrong token {:?}", tokens),
    };
    match iter.next().unwrap() {
        TokenTree::Ident(ident) => assert_eq!(ident.to_string(), "doc"),
        _ => panic!("wrong token {:?}", tokens),
    }
    match iter.next().unwrap() {
        TokenTree::Punct(punct) => assert_eq!(punct.as_char(), '='),
        _ => panic!("wrong token {:?}", tokens),
    }
    match iter.next().unwrap() {
        TokenTree::Literal(literal) => {
            assert!(iter.next().is_none(), "unexpected token {:?}", tokens);
            literal
        }
        _ => panic!("wrong token {:?}", tokens),
    }
}

#[test]
fn tricky_doc_comment() {
    let stream = "/**/".parse::<TokenStream>().unwrap();
    let tokens = stream.into_iter().collect::<Vec<_>>();
    assert!(tokens.is_empty(), "not empty -- {:?}", tokens);

    let stream = "/// doc".parse::<TokenStream>().unwrap();
    let lit = lit_of_doc_comment(stream);
    assert_eq!(lit.to_string(), "\" doc\"");

    let stream = "//! doc".parse::<TokenStream>().unwrap();
    let tokens = stream.into_iter().collect::<Vec<_>>();
    assert!(tokens.len() == 3, "not length 3 -- {:?}", tokens);
}

#[test]
fn incomplete_comment_no_panic() {
    let s = "/*/";
    assert!(s.parse::<TokenStream>().is_err());
}
