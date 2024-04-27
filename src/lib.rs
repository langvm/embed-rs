// Copyright 2024 Jelly Terra
// Use of this source code form is governed under the MIT license.

use proc_macro::{Literal, TokenStream, TokenTree};
use std::fs;

fn unquote_string(str: String) -> String {
    let seq = str.chars().collect::<Vec<char>>();
    seq[1..seq.len() - 1].iter().collect::<String>()
}

#[proc_macro]
pub fn embed_as_string(input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = input.into_iter().collect();

    let filename = unquote_string(tokens[0].to_string());

    let content = fs::read_to_string(filename).unwrap();
    TokenStream::from(TokenTree::from(Literal::string(content.as_str())))
}
