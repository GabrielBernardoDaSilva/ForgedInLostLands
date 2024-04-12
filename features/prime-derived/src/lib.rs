use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(DestinyRiftArcaneScript)]
pub fn destiny_rift_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_destiny_rift(&ast)
}

fn impl_destiny_rift(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl forged_in_lost_lands::destiny_rift::DestinyRift for #name {}
    };
    gen.into()
}

#[proc_macro_derive(EtherealFlowArcaneScript)]
pub fn ethereal_flow_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_ethereal_flow(&ast)
}

fn impl_ethereal_flow(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl forged_in_lost_lands::EtherealFlow for #name {
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

#[proc_macro_attribute]
pub fn hierarchy_ethereal_flow(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let struct_pos = item.to_string().find("struct").unwrap();
    let first_parenthesis_pos = item.to_string().find("{").unwrap();
    let str_item = item.to_string();
    let struct_name = str_item[struct_pos + 6..first_parenthesis_pos].trim();

    // add ethereal flow derive
    let item = add_ethereal_flow_derive(item);

    let father_str = "pub father: Option<String>} 
                            impl ::forged_in_lost_lands::forged_trait::ForgedHierarchy for #### {
                                fn get_father(&self) -> Option<String> {
                                   if let Some(father) = &self.father {
                                       return Some(father.clone());
                                   }
                                   None
                                }
                                fn set_father(&mut self, father_id:String) {
                                    self.father = Some(father_id);
                                }
                            }";
    let father_str = father_str.replace("####", struct_name);
    let item = item.to_string().replace("}", &father_str).parse().unwrap();
    item
}

fn add_ethereal_flow_derive(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let gen = quote! {
        #[derive(EtherealFlowArcaneScript)]
        #ast
    };
    gen.into()
}
