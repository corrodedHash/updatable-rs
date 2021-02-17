pub struct ParName {
    #[allow(dead_code)]
    par: syn::token::Paren,
    pub name: syn::Ident,
}

impl syn::parse::Parse for ParName {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;

        let par = syn::parenthesized!(content in input);
        let name = content.parse()?;
        Ok(Self { par, name })
    }
}
