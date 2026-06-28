use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Token, braced};

/// Top-level item inside `tmpl! { }` or after the template ident in `tmpl!(tmpl, ...)`
enum TopItem {
    /// `key = value` — simple parameter
    Param { key: Ident, value: Expr },
    /// `name { arm, arm, ... }` — implementation block
    ImplBlock { name: Ident, arms: Vec<Arm> },
}

/// An arm inside an implementation block
enum Arm {
    /// `key = value` — single-param arm
    Single { key: Ident, value: Expr },
    /// `{ key = value, key = value, ... }` — multi-param arm
    Multi(Vec<(Ident, Expr)>),
}

impl Parse for TopItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let ahead = input.lookahead1();
        if ahead.peek(Token![=]) {
            let _eq: Token![=] = input.parse()?;
            let value: Expr = input.parse()?;
            Ok(TopItem::Param { key: ident, value })
        } else if ahead.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            let arms: Vec<Arm> = content
                .parse_terminated(Arm::parse, Token![,])?
                .into_iter()
                .collect();
            Ok(TopItem::ImplBlock { name: ident, arms })
        } else {
            Err(ahead.error())
        }
    }
}

impl Parse for Arm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            let params: Vec<(Ident, Expr)> = content
                .parse_terminated(Self::parse_single, Token![,])?
                .into_iter()
                .collect();
            Ok(Arm::Multi(params))
        } else {
            let (key, value) = Self::parse_single(input)?;
            Ok(Arm::Single { key, value })
        }
    }
}

impl Arm {
    fn parse_single(input: ParseStream) -> syn::Result<(Ident, Expr)> {
        let key: Ident = input.parse()?;
        let _eq: Token![=] = input.parse()?;
        let value: Expr = input.parse()?;
        Ok((key, value))
    }
}

fn split_input(input: ParseStream) -> syn::Result<(Ident, Vec<TopItem>)> {
    // Try to parse: `ident , items` (explicit template variable)
    // If the first token is an ident followed by `,`, it's explicit.
    // Otherwise, default to `tmpl` and parse as items.

    let first: Ident = input.parse()?;
    if input.peek(Token![,]) {
        let _comma: Token![,] = input.parse()?;
        let items: Vec<TopItem> = input
            .parse_terminated(TopItem::parse, Token![,])?
            .into_iter()
            .collect();
        Ok((first, items))
    } else {
        // The first ident is actually the start of the first item.
        // It must be re-parsed. Since `first` has already been consumed,
        // the first item is parsed manually using `first` as the starting ident,
        // and then any remaining items are parsed from the input stream.
        let items = {
            // Re-build: first ident + rest
            let mut items: Vec<TopItem> = Vec::new();

            // Parse the first item starting with `first`
            let ahead = input.lookahead1();
            if ahead.peek(Token![=]) {
                let _eq: Token![=] = input.parse()?;
                let value: Expr = input.parse()?;
                items.push(TopItem::Param { key: first, value });
            } else if ahead.peek(syn::token::Brace) {
                let content;
                braced!(content in input);
                let arms: Vec<Arm> = content
                    .parse_terminated(Arm::parse, Token![,])?
                    .into_iter()
                    .collect();
                items.push(TopItem::ImplBlock { name: first, arms });
            } else {
                return Err(ahead.error());
            }

            // Parse remaining items
            while !input.is_empty() {
                let _comma: Token![,] = input.parse()?;
                if input.is_empty() {
                    break; // trailing comma
                }
                items.push(input.parse()?);
            }

            items
        };

        Ok((Ident::new("tmpl", proc_macro2::Span::call_site()), items))
    }
}

#[proc_macro]
pub fn tmpl(input: TokenStream) -> TokenStream {
    match syn::parse::Parser::parse(split_input, input) {
        Ok((template_var, items)) => {
            let stmts = generate(&template_var, &items);
            let expanded = quote! { { #(#stmts)* } };
            expanded.into()
        }
        Err(e) => e.to_compile_error().into(),
    }
}

fn generate(template_var: &Ident, items: &[TopItem]) -> Vec<proc_macro2::TokenStream> {
    let mut stmts: Vec<proc_macro2::TokenStream> = Vec::new();

    for item in items {
        match item {
            TopItem::Param { key, value } => {
                let key_str = key.to_string();
                stmts.push(quote! {
                    #template_var.insert_param(
                        #key_str.to_string(),
                        ::std::string::ToString::to_string(&#value),
                    );
                });
            }
            TopItem::ImplBlock { name, arms } => {
                let name_str = name.to_string();
                // Generate push statements for each arm
                let push_stmts: Vec<proc_macro2::TokenStream> =
                    arms.iter().map(|arm| gen_arm_push(name, arm)).collect();
                stmts.push(quote! {
                    let #name = #template_var.add_impl(#name_str.to_string());
                    #(#push_stmts)*
                });
            }
        }
    }

    stmts
}

fn gen_arm_push(name: &Ident, arm: &Arm) -> proc_macro2::TokenStream {
    match arm {
        Arm::Single { key, value } => {
            let key_str = key.to_string();
            quote! {
                #name.push(::std::collections::HashMap::from([
                    (#key_str.to_string(), ::std::string::ToString::to_string(&#value)),
                ]));
            }
        }
        Arm::Multi(params) => {
            let entries: Vec<_> = params
                .iter()
                .map(|(key, value)| {
                    let k = key.to_string();
                    quote! {
                        (#k.to_string(), ::std::string::ToString::to_string(&#value))
                    }
                })
                .collect();
            quote! {
                #name.push(::std::collections::HashMap::from([
                    #(#entries),*
                ]));
            }
        }
    }
}
