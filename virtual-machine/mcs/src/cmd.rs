use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{DeriveInput, parse_macro_input},
};

pub fn parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let definition = quote! {
        impl crate::core::CmdExt for #ident {
            fn opcode(&self) -> crate::core::Opcode {
                self.0.opcode
            }

            fn rank(&self) -> crate::core::Rank {
                self.0.rank
            }

            fn kind(&self) -> crate::core::Kind {
                self.0.kind
            }

            fn task(&self) -> crate::core::Task {
                self.0.task
            }
        }
    };

    TokenStream::from(definition)
}
