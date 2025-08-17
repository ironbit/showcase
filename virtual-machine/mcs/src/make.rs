use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{DeriveInput, parse_macro_input},
};

pub fn parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let definition = quote! {
        impl #ident {
            pub fn new<S>(cmd: crate::core::Cmd) -> std::rc::Rc<dyn crate::isa::CommandExt<S>>
            where
                S: crate::mem::StackExt + std::fmt::Debug,
            {
                std::rc::Rc::new(Self(cmd))
            }

            pub fn cmd(&self) -> &crate::core::Cmd {
                &self.0
            }
        }
    };

    TokenStream::from(definition)
}
