use proc_macro2::TokenStream;

use crate::parser::{UpdateFields, UpdateFn};

pub struct UpdatableEnumEntry<'a>(pub &'a UpdateFn);

impl<'a> quote::ToTokens for UpdatableEnumEntry<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let variant_name = &self.0.name;
        let result_stream = match self.0.fields {
            UpdateFields::Parenthesized(ref field) => {
                let field_type = &field.ty;
                quote::quote! {
                    #variant_name ( #field_type )
                }
            }
            UpdateFields::Braced(ref fields) => {
                let field_iter = fields.iter();

                quote::quote! {
                  #variant_name { #( #field_iter),* }
                }
            }
        };

        result_stream.to_tokens(tokens);
    }
}

pub struct UpdatableApply<'a>(pub &'a UpdateFn);

impl<'a> quote::ToTokens for UpdatableApply<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant_name = &self.0.name;
        let func = &self.0.func;
        let result_stream = match self.0.fields {
            UpdateFields::Parenthesized(ref field) => {
                let member_name = &field.ident;
                quote::quote! {
                    #variant_name ( #member_name ) => #func
                }
            }
            UpdateFields::Braced(ref fields) => {
                let member_names = fields.iter().map(|x| &x.ident);
                quote::quote! {
                    #variant_name { #( #member_names ),* } => #func
                }
            }
        };
        result_stream.to_tokens(tokens);
    }
}
