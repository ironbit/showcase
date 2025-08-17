use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{DeriveInput, parse_macro_input},
};

pub fn parse(input: TokenStream, kind: &str) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let expr = format!("<Self as crate::isa::{kind}<S>>::process(self, ctx, code)");
    let execute: syn::Expr = syn::parse_str(&expr).unwrap();

    let definition = quote! {
        impl<S> crate::isa::AryExt<S> for #ident
        where
            S: crate::mem::StackExt + std::fmt::Debug,
        { }

        impl<S> crate::isa::CommandExt<S> for #ident
        where
            S: crate::mem::StackExt + std::fmt::Debug,
        {
            fn execute(&self, ctx: &mut crate::vm::Context<S>, code: &crate::core::Code) -> anyhow::Result<crate::isa::Status> {
                #execute
            }
        }
    };

    TokenStream::from(definition)
}
