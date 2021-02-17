#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::needless_return)]
#![allow(clippy::cargo_common_metadata)]

use std::vec::Vec;

mod parser;
use parser::{ParenthesizedUpdateFn, UpdateFn, UpdateFnName};

mod printer;
use printer::{UpdatableApply, UpdatableEnumEntry};

#[proc_macro_derive(
    Updatable,
    attributes(
        update_fn,
        update_fn_name,
        no_update,
        update_state_name,
        update_error_name
    )
)]
pub fn derive_answer_fn(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let name = input.ident;
    let update_state_enum_name = input
        .attrs
        .iter()
        .find(|x| x.path.is_ident("update_state_name"));
    let update_state_error_name = input
        .attrs
        .iter()
        .find(|x| x.path.is_ident("update_error_name"));

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
        for att in &f.attrs {
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
    let update_state_enum_name = if let Some(att) = update_state_enum_name {
        let hehe = att.tokens.clone().into();
        syn::parse_macro_input!(hehe as parser::ParName).name
    } else {
        quote::format_ident!("{}StateUpdate", name)
    };
    let update_state_error_name = if let Some(att) = update_state_error_name {
        let hehe = att.tokens.clone().into();
        syn::parse_macro_input!(hehe as parser::ParName).name
    } else {
        quote::format_ident!("{}StateUpdateError", name)
    };
    let bla = update_fns.iter().map(UpdatableEnumEntry);
    let match_cases = update_fns.iter().map(UpdatableApply);
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
