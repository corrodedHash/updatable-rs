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
        Ok(UpdateFn {
            name: input.parse()?,
            first_punc: input.parse()?,
            delimiter_token: {
                if input.lookahead1().peek(syn::token::Brace) {
                    braced = true;
                    UpdateDelimiter::Brace(syn::braced!(content in input))
                } else {
                    UpdateDelimiter::Par(syn::parenthesized!(content in input))
                }
            },
            fields: {
                if braced {
                    UpdateFields::Braced(content.parse_terminated(syn::Field::parse_named)?)
                } else {
                    UpdateFields::Parenthesized(syn::Field::parse_named(&content)?)
                }
            },
            second_punc: input.parse()?,
            func: input.parse()?,
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
        Ok(ParenthesizedUpdateFn {
            par: syn::parenthesized!(content in input),
            update_fn: content.parse()?,
        })
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
        Ok(UpdateFnName {
            par: syn::parenthesized!(content in input),
            name: content.parse()?,
        })
    }
}
