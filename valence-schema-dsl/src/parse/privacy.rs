use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream},
    token, Expr, Ident, LitBool, Result, Token,
};

/// Table- and field-level policy bundles lowered to token streams for codegen.
#[derive(Debug, Clone, Default)]
pub struct ParsedPolicies {
    pub read: Option<ParsedPolicyRules>,
    pub create: Option<ParsedPolicyRules>,
    pub update: Option<ParsedPolicyRules>,
    pub delete: Option<ParsedPolicyRules>,
}

#[derive(Debug, Clone, Default)]
pub struct ParsedPolicyRules {
    pub always_allow: Vec<TokenStream>,
    pub allow: Vec<TokenStream>,
    pub block: Vec<TokenStream>,
    pub always_block: Vec<TokenStream>,
}

/// Parsed `privacy: { ... }` container (values are not carried into [`super::schema::ParsedSchema`]).
#[allow(dead_code)]
pub struct PrivacyConfig {
    _brace: token::Brace,
    pub items: Punctuated<PrivacyItem, Token![,]>,
}

impl Parse for PrivacyConfig {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(PrivacyConfig {
            _brace: braced!(content in input),
            items: content.parse_terminated(PrivacyItem::parse, Token![,])?,
        })
    }
}

pub enum PrivacyItem {
    /// `gdpr_compliant:` literal accepted for source compatibility.
    GdprCompliant,
}

impl Parse for PrivacyItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        input.parse::<Token![:]>()?;

        match key.to_string().as_str() {
            "gdpr_compliant" => {
                let _: LitBool = input.parse()?;
                Ok(PrivacyItem::GdprCompliant)
            }
            _ => Err(syn::Error::new(
                key.span(),
                format!("Unknown privacy key: {key}"),
            )),
        }
    }
}

pub struct PoliciesConfig {
    _brace: token::Brace,
    pub items: Punctuated<PolicyItem, Token![,]>,
}

impl Parse for PoliciesConfig {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(PoliciesConfig {
            _brace: braced!(content in input),
            items: content.parse_terminated(PolicyItem::parse, Token![,])?,
        })
    }
}

pub enum PolicyItem {
    Read(PolicySpec),
    Create(PolicySpec),
    Update(PolicySpec),
    Delete(PolicySpec),
}

impl Parse for PolicyItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        input.parse::<Token![:]>()?;

        match key.to_string().as_str() {
            "read" => Ok(PolicyItem::Read(input.parse()?)),
            "create" => Ok(PolicyItem::Create(input.parse()?)),
            "update" => Ok(PolicyItem::Update(input.parse()?)),
            "delete" => Ok(PolicyItem::Delete(input.parse()?)),
            _ => Err(syn::Error::new(
                key.span(),
                format!("Unknown policy key: {key}"),
            )),
        }
    }
}

/// A policy specification (PrivacyPolicy with rule arrays)
pub struct PolicySpec {
    _brace: token::Brace,
    pub items: Punctuated<PolicyRuleList, Token![,]>,
}

impl Parse for PolicySpec {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(PolicySpec {
            _brace: braced!(content in input),
            items: content.parse_terminated(PolicyRuleList::parse, Token![,])?,
        })
    }
}

/// A list of policy rules (e.g., `allow: [PUBLIC_READ, AUTHENTICATED]`)
pub struct PolicyRuleList {
    pub name: Ident,
    _colon: Token![:],
    _bracket: token::Bracket,
    pub rules: Punctuated<Expr, Token![,]>,
}

impl Parse for PolicyRuleList {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let colon = input.parse::<Token![:]>()?;
        let content;
        let bracket = bracketed!(content in input);
        let mut rules = Punctuated::new();

        while !content.is_empty() {
            let expr: Expr = content.parse()?;
            rules.push_value(expr);

            if !content.is_empty() {
                rules.push_punct(content.parse()?);
            }
        }

        Ok(PolicyRuleList {
            name,
            _colon: colon,
            _bracket: bracket,
            rules,
        })
    }
}

pub fn parse_policies_config(config: &PoliciesConfig) -> Result<ParsedPolicies> {
    let mut parsed = ParsedPolicies::default();

    for item in &config.items {
        match item {
            PolicyItem::Read(spec) => parsed.read = Some(parse_policy_spec(spec)?),
            PolicyItem::Create(spec) => parsed.create = Some(parse_policy_spec(spec)?),
            PolicyItem::Update(spec) => parsed.update = Some(parse_policy_spec(spec)?),
            PolicyItem::Delete(spec) => parsed.delete = Some(parse_policy_spec(spec)?),
        }
    }

    Ok(parsed)
}

fn parse_policy_spec(spec: &PolicySpec) -> Result<ParsedPolicyRules> {
    let mut rules = ParsedPolicyRules::default();

    for list in &spec.items {
        let rule_tokens: Vec<TokenStream> = list
            .rules
            .iter()
            .map(|expr| expr.to_token_stream())
            .collect();

        match list.name.to_string().as_str() {
            "always_allow" => rules.always_allow.extend(rule_tokens),
            "allow" => rules.allow.extend(rule_tokens),
            "block" => rules.block.extend(rule_tokens),
            "always_block" => rules.always_block.extend(rule_tokens),
            _ => {}
        }
    }

    Ok(rules)
}
