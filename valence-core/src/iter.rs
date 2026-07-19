//! Row-level iter descriptor types for `valence_schema! { iters: [...] }`.

use crate::runtime::Valence;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

/// Result of an iter `should_run` hook.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterEvaluation {
    pub should_run: bool,
    pub reason: String,
}

impl IterEvaluation {
    pub fn run(reason: impl Into<String>) -> Self {
        Self {
            should_run: true,
            reason: reason.into(),
        }
    }

    pub fn skip(reason: impl Into<String>) -> Self {
        Self {
            should_run: false,
            reason: reason.into(),
        }
    }
}

/// Type-erased `should_run` for orchestration (row as JSON).
pub type IterShouldRunFn =
    fn(
        Valence,
        serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<IterEvaluation>> + Send + 'static>>;

/// Type-erased `execute` for orchestration (row as JSON).
pub type IterExecuteFn = fn(
    Valence,
    serde_json::Value,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send + 'static>>;

/// One registered iter implementation for a table (submitted by `valence_schema!`).
#[derive(Copy, Clone)]
pub struct IterDescriptor {
    pub iter_type_name: &'static str,
    pub table_name: &'static str,
    pub should_run: IterShouldRunFn,
    pub execute: IterExecuteFn,
}

inventory::collect!(IterDescriptor);
