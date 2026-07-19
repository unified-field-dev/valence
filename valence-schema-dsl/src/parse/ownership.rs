//! `ownership: { system, resolve }` grammar.

use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    token, Ident, LitBool, Path, Result, Token,
};

/// Parsed `ownership: { system: bool, resolve: path::To::Resolver }`.
#[derive(Debug, Clone, Default)]
pub struct ParsedOwnershipConfig {
    pub system_owned: bool,
    pub resolve: Option<String>,
}

/// `ownership: { system: true, resolve: path::Type }`
pub struct OwnershipConfigBlock {
    _brace: token::Brace,
    pub items: Punctuated<OwnershipItem, Token![,]>,
}

pub enum OwnershipItem {
    System(LitBool),
    Resolve(Path),
}

impl Parse for OwnershipConfigBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(OwnershipConfigBlock {
            _brace: braced!(content in input),
            items: content.parse_terminated(OwnershipItem::parse, Token![,])?,
        })
    }
}

impl Parse for OwnershipItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        match key.to_string().as_str() {
            "system" => Ok(OwnershipItem::System(input.parse()?)),
            "resolve" => Ok(OwnershipItem::Resolve(input.parse()?)),
            _ => Err(syn::Error::new(
                key.span(),
                format!("Unknown ownership key: {key}"),
            )),
        }
    }
}

pub(super) fn parse_ownership_block(
    block: &OwnershipConfigBlock,
    span: proc_macro2::Span,
) -> Result<ParsedOwnershipConfig> {
    let mut system_owned = false;
    let mut resolve: Option<String> = None;
    for oi in &block.items {
        match oi {
            OwnershipItem::System(b) => system_owned = b.value,
            OwnershipItem::Resolve(p) => {
                resolve = Some(p.to_token_stream().to_string().replace(' ', ""));
            }
        }
    }
    if system_owned && resolve.is_some() {
        return Err(syn::Error::new(
            span,
            "ownership: use only one of `system: true` or `resolve:`",
        ));
    }
    Ok(ParsedOwnershipConfig {
        system_owned,
        resolve,
    })
}
