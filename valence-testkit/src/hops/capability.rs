//! Explicit hop capability matrix for release honesty.
//!
//! Combinations listed as unsupported are **skipped** with a documented reason.
//! They must not soft-pass broken semantics via ignored empty/false-positive results.

use crate::hops::layout::{HopPair, HopQuad, HopTriple};
use crate::matrix::StorageAdapter;

/// Why a hop assertion was skipped (backend missing vs capability gap).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HopSkip {
    /// Required storage adapter is not available in this environment.
    BackendUnavailable,
    /// Nested `EXISTS` / connection predicates are not asserted for this layout yet.
    NestedWhereUnsupported,
}

impl HopSkip {
    pub fn label(self) -> &'static str {
        match self {
            Self::BackendUnavailable => "backend_unavailable",
            Self::NestedWhereUnsupported => "nested_where_unsupported",
        }
    }
}

/// Cross-backend nested `EXISTS` is not part of the asserted 0.1.x contract.
///
/// Navigation (BelongsTo / HasMany) is still asserted when backends are available.
/// Same-backend nested where is out of scope for the directed-pair matrix (pairs are
/// always `primary ≠ secondary`).
pub fn pair_nested_where_skip(pair: HopPair) -> Option<&'static str> {
    let _ = pair;
    Some("cross-backend nested EXISTS not asserted until capability matrix expands (pre-1.0)")
}

/// Multi-engine depth-3 chains do not assert nested `EXISTS` in 0.1.x.
pub fn triple_nested_where_skip(triple: HopTriple) -> Option<&'static str> {
    let _ = triple;
    Some("multi-engine nested EXISTS not asserted until capability matrix expands (pre-1.0)")
}

/// Multi-engine depth-4 chains do not assert nested `EXISTS` in 0.1.x.
pub fn quad_nested_where_skip(quad: HopQuad) -> Option<&'static str> {
    let _ = quad;
    Some("multi-engine nested EXISTS not asserted until capability matrix expands (pre-1.0)")
}

/// Adapters excluded from hop layouts (stub / non-relational).
pub fn hop_adapter_excluded(adapter: StorageAdapter) -> bool {
    matches!(adapter, StorageAdapter::AcmeStub)
}
