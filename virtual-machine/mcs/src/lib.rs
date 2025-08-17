mod cmd;
mod kind;
mod make;

use proc_macro::TokenStream;

#[proc_macro_derive(Make)]
pub fn parse_make(input: TokenStream) -> TokenStream {
    make::parse(input)
}

#[proc_macro_derive(CmdExt)]
pub fn parse_cmd_ext(input: TokenStream) -> TokenStream {
    cmd::parse(input)
}

#[proc_macro_derive(NullaryExt)]
pub fn parse_nullary_ext(input: TokenStream) -> TokenStream {
    kind::parse(input, "NullaryExt")
}

#[proc_macro_derive(UnaryExt)]
pub fn parse_unary_ext(input: TokenStream) -> TokenStream {
    kind::parse(input, "UnaryExt")
}

#[proc_macro_derive(BinaryExt)]
pub fn parse_binary_ext(input: TokenStream) -> TokenStream {
    kind::parse(input, "BinaryExt")
}
