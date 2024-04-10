use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(DestinyRiftMacro)]
pub fn destiny_rift_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_destiny_rift(&ast)
}

fn impl_destiny_rift(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl forged_in_lost_lands::prime_forge::destiny_rift::DestinyRift for #name {}
    };
    gen.into()
}

#[proc_macro_derive(EtherealFlowMAcro)]
pub fn ethereal_flow_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_ethereal_flow(&ast)
}

fn impl_ethereal_flow(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl forged_in_lost_lands::prime_forge::EtherealFlow for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
    gen.into()
}
