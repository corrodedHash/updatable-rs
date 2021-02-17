use proc_macro2::Ident;

pub enum UpdateFields {
    Parenthesized(syn::Field),
    Braced(syn::punctuated::Punctuated<syn::Field, syn::Token![,]>),
}

pub enum UpdateDelimiter {
    Par(syn::token::Paren),
    Brace(syn::token::Brace),
}

pub struct UpdateFn {
    pub name: Ident,
    #[allow(dead_code)]
    first_punc: syn::Token![,],
    #[allow(dead_code)]
    delimiter_token: UpdateDelimiter,
    pub fields: UpdateFields,
    #[allow(dead_code)]
    second_punc: syn::Token![,],
    pub func: syn::Block,
}

impl syn::parse::Parse for UpdateFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let mut braced = false;
        let name = input.parse()?;
        let first_punc = input.parse()?;
        let delimiter_token = if input.lookahead1().peek(syn::token::Brace) {
            braced = true;
            UpdateDelimiter::Brace(syn::braced!(content in input))
        } else {
            UpdateDelimiter::Par(syn::parenthesized!(content in input))
        };
        let fields = if braced {
            UpdateFields::Braced(content.parse_terminated(syn::Field::parse_named)?)
        } else {
            UpdateFields::Parenthesized(syn::Field::parse_named(&content)?)
        };
        let second_punc = input.parse()?;
        let func = input.parse()?;
        Ok(UpdateFn {
            name,
            first_punc,
            delimiter_token,
            fields,
            second_punc,
            func,
        })
    }
}

pub struct ParenthesizedUpdateFn {
    #[allow(dead_code)]
    par: syn::token::Paren,
    pub update_fn: UpdateFn,
}
impl syn::parse::Parse for ParenthesizedUpdateFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;

        let par = syn::parenthesized!(content in input);
        let update_fn = content.parse()?;
        Ok(ParenthesizedUpdateFn { par, update_fn })
    }
}

pub struct UpdateFnName {
    #[allow(dead_code)]
    par: syn::token::Paren,
    pub name: Ident,
}

impl syn::parse::Parse for UpdateFnName {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;

        let par = syn::parenthesized!(content in input);
        let name = content.parse()?;
        Ok(UpdateFnName { par, name })
    }
}
