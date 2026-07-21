//! Normalize `FieldType::…` and `Validator::…` token trees into the compact strings
//! [`crate::ParsedField`] and validations use (e.g. `record<user>`, `min_length:10`).
//!
//! These helpers intentionally use [`TokenStream::to_string()`] and light string parsing
//! because the DSL accepts arbitrary `syn::Expr` fragments inside the macro; a fully
//! symbolic parser would duplicate `syn` AST matching for little gain here.

use proc_macro2::TokenStream;
use syn::Result;

/// Strip surrounding quotes for string literal defaults; otherwise return the raw token text.
pub fn extract_default_string(tokens: &TokenStream) -> String {
    let s = tokens.to_string();
    if s.starts_with('"') && s.ends_with('"') && s.len() > 1 {
        s[1..s.len() - 1].to_string()
    } else {
        s
    }
}

/// Lower a `r#type: FieldType::…` expression to Valence's internal type name.
///
/// Order matters: `ExternalEnum("crate::Foo::Bar")` must run before naive `rfind("::")`
/// splitting so paths with `::` inside the string stay intact.
pub fn extract_field_type_string(tokens: &TokenStream) -> Result<String> {
    let s = tokens.to_string();

    if let Some(path) = parse_external_enum(&s) {
        return Ok(format!("ext_enum:{path}"));
    }
    if let Some(variants) = parse_braced_enum_variants(&s) {
        return Ok(format!("enum:{}", variants.join(",")));
    }

    let Some(idx) = s.rfind("::") else {
        return Ok(s.to_lowercase());
    };

    let ftype = s[idx + 2..].trim();
    if let Some(table) = parse_record_table_name(ftype) {
        return Ok(format!("record<{table}>"));
    }
    Ok(ftype.to_lowercase())
}

fn parse_external_enum(s: &str) -> Option<String> {
    let ext_pos = s.find("ExternalEnum")?;
    let after = &s[ext_pos..];
    let paren_start = after.find('(')?;
    let paren_end = after.rfind(')')?;
    let param = after[paren_start + 1..paren_end].trim();
    Some(param.trim_matches('"').trim().replace(' ', ""))
}

/// `FieldType :: Enum (& ["A", "B"])` after `to_string()` normalization.
fn parse_braced_enum_variants(s: &str) -> Option<Vec<String>> {
    let enum_pos = s.find(":: Enum")?;
    let after = s[enum_pos + 2..].trim_start();
    if !after.starts_with("Enum") {
        return None;
    }
    let paren_start = after.find('(')?;
    let paren_end = after.rfind(')')?;
    let param = after[paren_start + 1..paren_end].trim();
    let array_content = param
        .trim_start_matches('&')
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .trim();
    let values: Vec<String> = array_content
        .split(',')
        .map(|v| v.trim().trim_matches('"').to_string())
        .filter(|v| !v.is_empty())
        .collect();
    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}

fn parse_record_table_name(ftype: &str) -> Option<String> {
    if !ftype.starts_with("Record") {
        return None;
    }
    let paren_start = ftype.find('(')?;
    let paren_end = ftype.rfind(')')?;
    let param = ftype[paren_start + 1..paren_end].trim();
    Some(param.trim_matches('"').trim().to_string())
}

/// Lower `Validator::…` tokens to validation DSL strings (`email`, `min_length:5`, …).
pub fn extract_validator_string(tokens: &TokenStream) -> Result<String> {
    let s = tokens.to_string();

    if let Some(idx) = s.find("::") {
        let rest = s[idx + 2..].trim();
        if let Some(k) = simple_validator_keyword(rest) {
            return Ok(k);
        }
        if let Some(v) = parameterized_validator(rest) {
            return Ok(v);
        }
    }

    Ok(if s.starts_with('"') && s.ends_with('"') && s.len() > 1 {
        s[1..s.len() - 1].to_string()
    } else {
        s
    })
}

fn simple_validator_keyword(rest: &str) -> Option<String> {
    const MAP: &[(&str, &str)] = &[
        ("Email", "email"),
        ("Phone", "phone"),
        ("Url", "url"),
        ("NonEmpty", "non_empty"),
        ("Positive", "positive"),
        ("NonNegative", "non_negative"),
    ];
    for (prefix, out) in MAP {
        if rest.starts_with(prefix) && !rest[prefix.len()..].starts_with('(') {
            return Some((*out).to_string());
        }
    }
    None
}

fn parameterized_validator(rest: &str) -> Option<String> {
    let paren_idx = rest.find('(')?;
    let end_paren = rest.rfind(')')?;
    let validator_name = rest[..paren_idx].trim();
    let param = rest[paren_idx + 1..end_paren].trim();

    match validator_name {
        "MinLength" => Some(format!("min_length:{param}")),
        "MaxLength" => Some(format!("max_length:{param}")),
        "Min" => Some(format!("min:{param}")),
        "Max" => Some(format!("max:{param}")),
        "Enum" => Some(format!("enum:{}", split_comma_list(param).join(","))),
        "Pattern" => Some(format!("pattern:{}", param.trim_matches('"'))),
        "Custom" => Some(format!("fn:{}", param.trim_matches('"'))),
        _ => None,
    }
}

fn split_comma_list(param: &str) -> Vec<String> {
    let array_content = param
        .trim_start_matches('&')
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .trim();
    array_content
        .split(',')
        .map(|v| v.trim().trim_matches('"').to_string())
        .collect()
}
