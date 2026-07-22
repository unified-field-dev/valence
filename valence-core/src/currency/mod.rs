//! Monetary amount with an ISO-4217 [`CurrencyCode`].
//!
//! # Storage
//!
//! Persisted as a single JSON object:
//! `{ "code": "USD", "amount_minor": 12345 }`
//! where `code` is the alphabetic ISO string and `amount_minor` is signed minor units.

mod code;

pub use code::{CurrencyCode, ParseCurrencyCodeError};

use serde::{Deserialize, Serialize};

/// Same-currency arithmetic / conversion failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurrencyError {
    /// Operands used different [`CurrencyCode`] values.
    Mismatch {
        left: CurrencyCode,
        right: CurrencyCode,
    },
    /// Checked arithmetic overflowed.
    Overflow,
}

impl std::fmt::Display for CurrencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrencyError::Mismatch { left, right } => {
                write!(f, "currency mismatch: {left} vs {right}")
            }
            CurrencyError::Overflow => f.write_str("currency arithmetic overflow"),
        }
    }
}

impl std::error::Error for CurrencyError {}

/// Composite money value: ISO code + signed minor units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Currency {
    code: CurrencyCode,
    amount_minor: i64,
}

impl Currency {
    /// Construct a monetary value.
    #[must_use]
    pub const fn new(code: CurrencyCode, amount_minor: i64) -> Self {
        Self { code, amount_minor }
    }

    /// Zero amount in `code`.
    #[must_use]
    pub const fn zero(code: CurrencyCode) -> Self {
        Self::new(code, 0)
    }

    #[must_use]
    pub const fn code(self) -> CurrencyCode {
        self.code
    }

    #[must_use]
    pub const fn amount_minor(self) -> i64 {
        self.amount_minor
    }

    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.amount_minor == 0
    }

    /// Absolute minor amount (overflow on `i64::MIN` → `None`).
    #[must_use]
    pub const fn checked_abs(self) -> Option<Self> {
        match self.amount_minor.checked_abs() {
            Some(v) => Some(Self::new(self.code, v)),
            None => None,
        }
    }

    /// Negate minor amount (overflow on `i64::MIN` → `None`).
    #[must_use]
    pub const fn checked_negate(self) -> Option<Self> {
        match self.amount_minor.checked_neg() {
            Some(v) => Some(Self::new(self.code, v)),
            None => None,
        }
    }

    /// Same-currency checked addition.
    pub fn checked_add(self, other: Self) -> Result<Self, CurrencyError> {
        if self.code != other.code {
            return Err(CurrencyError::Mismatch {
                left: self.code,
                right: other.code,
            });
        }
        self.amount_minor
            .checked_add(other.amount_minor)
            .map(|v| Self::new(self.code, v))
            .ok_or(CurrencyError::Overflow)
    }

    /// Compare minor amounts when currencies match.
    pub fn partial_cmp_amount(self, other: Self) -> Result<std::cmp::Ordering, CurrencyError> {
        if self.code != other.code {
            return Err(CurrencyError::Mismatch {
                left: self.code,
                right: other.code,
            });
        }
        Ok(self.amount_minor.cmp(&other.amount_minor))
    }

    /// Build from major units using the ISO exponent for `code`.
    ///
    /// `major` is scaled by `10^exponent` into minor units.
    pub fn from_major_units(code: CurrencyCode, major: i64) -> Result<Self, CurrencyError> {
        let factor = 10_i64
            .checked_pow(code.exponent())
            .ok_or(CurrencyError::Overflow)?;
        major
            .checked_mul(factor)
            .map(|minor| Self::new(code, minor))
            .ok_or(CurrencyError::Overflow)
    }

    /// Convert minor units to truncated major units using the ISO exponent.
    pub fn to_major_units(self) -> Result<i64, CurrencyError> {
        let factor = 10_i64
            .checked_pow(self.code.exponent())
            .ok_or(CurrencyError::Overflow)?;
        if factor == 0 {
            return Err(CurrencyError::Overflow);
        }
        Ok(self.amount_minor / factor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_shape() {
        let c = Currency::new(CurrencyCode::Usd, 12345);
        let v = serde_json::to_value(c).unwrap();
        assert_eq!(
            v,
            serde_json::json!({ "code": "USD", "amount_minor": 12345 })
        );
        let back: Currency = serde_json::from_value(v).unwrap();
        assert_eq!(back, c);
    }

    #[test]
    fn rejects_unknown_code_on_deserialize() {
        let v = serde_json::json!({ "code": "ZZZ", "amount_minor": 1 });
        assert!(serde_json::from_value::<Currency>(v).is_err());
    }

    #[test]
    fn add_mismatch_and_major() {
        let a = Currency::new(CurrencyCode::Usd, 100);
        let b = Currency::new(CurrencyCode::Eur, 100);
        assert!(matches!(
            a.checked_add(b),
            Err(CurrencyError::Mismatch { .. })
        ));
        let from_major = Currency::from_major_units(CurrencyCode::Usd, 12).unwrap();
        assert_eq!(from_major.amount_minor(), 1200);
        assert_eq!(from_major.to_major_units().unwrap(), 12);
        let jpy = Currency::from_major_units(CurrencyCode::Jpy, 100).unwrap();
        assert_eq!(jpy.amount_minor(), 100);
    }
}
