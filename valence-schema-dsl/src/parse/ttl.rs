//! `ttl: { seconds, mode }` grammar.

use syn::punctuated::Punctuated;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    token, Ident, LitInt, LitStr, Result, Token,
};

/// Parsed TTL policy after lowering.
#[derive(Debug, Clone)]
pub struct ParsedTtlPolicy {
    pub seconds: u64,
    pub mode: String,
}

/// `ttl: { seconds: N, mode: "..." }`
pub struct TtlConfig {
    _brace: token::Brace,
    pub items: Punctuated<TtlItem, Token![,]>,
}

impl Parse for TtlConfig {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(TtlConfig {
            _brace: braced!(content in input),
            items: content.parse_terminated(TtlItem::parse, Token![,])?,
        })
    }
}

pub enum TtlItem {
    Seconds(LitInt),
    Mode(LitStr),
}

impl Parse for TtlItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        match key.to_string().as_str() {
            "seconds" => Ok(TtlItem::Seconds(input.parse()?)),
            "mode" => Ok(TtlItem::Mode(input.parse()?)),
            _ => Err(syn::Error::new(
                key.span(),
                format!("Unknown ttl key: {key}"),
            )),
        }
    }
}

pub(super) fn parse_ttl_config(config: &TtlConfig) -> Result<ParsedTtlPolicy> {
    let mut seconds: Option<u64> = None;
    let mut mode = "backend_capability".to_string();
    for item in &config.items {
        match item {
            TtlItem::Seconds(v) => {
                let parsed = v.base10_parse::<u64>()?;
                seconds = Some(parsed);
            }
            TtlItem::Mode(v) => mode = v.value(),
        }
    }

    let seconds = seconds.ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "ttl.seconds is required and must be an unsigned integer",
        )
    })?;

    Ok(ParsedTtlPolicy { seconds, mode })
}
