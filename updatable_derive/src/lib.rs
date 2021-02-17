use proc_macro2::TokenStream;
use std::vec::Vec;

mod parser;
use parser::{ParenthesizedUpdateFn, UpdateFields, UpdateFn, UpdateFnName};

struct UpdatableEnumEntry<'a> {
    hehe: &'a UpdateFn,
}

impl<'a> quote::ToTokens for UpdatableEnumEntry<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let variant_name = &self.hehe.name;
        let result_stream = match self.hehe.fields {
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

struct UpdatableApply<'a> {
    hehe: &'a UpdateFn,
}

impl<'a> quote::ToTokens for UpdatableApply<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let variant_name = &self.hehe.name;
        let func = &self.hehe.func;
        let result_stream = match self.hehe.fields {
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

#[proc_macro_derive(Updatable, attributes(update_fn, update_fn_name, no_update))]
pub fn derive_answer_fn(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let name = input.ident;
    let mut update_fns: Vec<UpdateFn> = vec![];
    let s = if let syn::Data::Struct(s) = input.data {
        s
    } else {
        panic!("Only works on structs");
    };
    for f in s.fields.iter() {
        let mut updated = false;
        let mut no_update = false;
        let member_name = if let Some(x) = f.ident.clone() {
            x
        } else {
            continue;
        };
        let mut update_fn_ident = member_name.clone();
        for att in f.attrs.iter() {
            if att.path.is_ident("update_fn") {
                let hehe = att.tokens.clone().into();
                update_fns.push(syn::parse_macro_input!(hehe as ParenthesizedUpdateFn).update_fn);
                updated = true;
            } else if att.path.is_ident("update_fn_name") {
                let hehe = att.tokens.clone().into();
                update_fn_ident = syn::parse_macro_input!(hehe as UpdateFnName).name;
            } else if att.path.is_ident("no_update") {
                no_update = true;
                break;
            }
        }
        if !updated && !no_update {
            let t = f.ty.clone();
            let members = quote::quote! {
                {
                    from: #t,
                    to: #t
                }
            };
            let func = quote::quote! {{
                if from != self.#member_name {
                    return Err(Self::StateUpdateError::WrongPrecondition);
                }
                self.#member_name = to;
            }};
            update_fns.push(syn::parse_quote! {
                #update_fn_ident , #members , #func
            });
        }
    }
    let update_state_enum_name = quote::format_ident!("{}StateUpdate", name);
    let update_state_error_name = quote::format_ident!("{}StateUpdateError", name);
    let bla = update_fns.iter().map(|x| UpdatableEnumEntry { hehe: x });
    let match_cases = update_fns.iter().map(|x| UpdatableApply { hehe: x });
    let update_states = quote::quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub enum #update_state_enum_name {
            #( #bla ),*
        }

        impl updatable::upd::Updatable< #update_state_enum_name > for #name {
            type StateUpdateError = #update_state_error_name;
             fn apply(&mut self, x: #update_state_enum_name) -> Result<(), Self::StateUpdateError>{
                match x {
                    #( #update_state_enum_name :: #match_cases )*
                }
                return Ok(());
            }
        }
    };
    return update_states.into();
}
