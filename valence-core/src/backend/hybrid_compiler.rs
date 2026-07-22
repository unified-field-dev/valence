//! Hybrid query [`QueryCompiler`] — M2M hop plans or SQL passthrough.

use serde_json::json;

use crate::compiled_query::CompiledQuery;
use crate::error::Result;
use crate::query::{HopType, QueryCore, WhereClause};
use crate::query_compiler::QueryCompiler;

#[cfg(feature = "compiler-sql")]
use crate::backend::SqlQueryCompiler;

/// Compiles [`QueryCore`] for `hybrid_indra_sql`.
///
/// Many-to-many hops become a JSON hop plan executed by the hybrid adapter; all other
/// queries delegate to [`SqlQueryCompiler`].
///
/// # Examples
///
/// ```
/// # #[cfg(all(feature = "compiler-hybrid", feature = "compiler-sql"))]
/// # {
/// use valence_core::backend::HybridQueryCompiler;
/// use valence_core::query::{HopSource, HopType, QueryCore};
/// use valence_core::query_compiler::QueryCompiler;
///
/// let source = QueryCore::new("org".into());
/// let mut target = QueryCore::new("project".into());
/// target.hop_source = Some(HopSource {
///     source_query: Box::new(source),
///     hop_type: HopType::ManyToManyForward {
///         edge_table: "org_projects".into(),
///     },
/// });
/// let compiled = HybridQueryCompiler.compile(&target).expect("compile");
/// assert!(compiled.query_string.contains("hybrid_hop"));
/// # }
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct HybridQueryCompiler;

impl QueryCompiler for HybridQueryCompiler {
    fn compile(&self, core: &QueryCore) -> Result<CompiledQuery> {
        if let Some(plan) = try_m2m_hop_plan(core)? {
            let query_string = serde_json::to_string(&plan).map_err(|e| {
                crate::error::Error::Internal(format!("serialize hybrid hop plan: {e}"))
            })?;
            return Ok(CompiledQuery::new(query_string, vec![]));
        }
        #[cfg(feature = "compiler-sql")]
        {
            SqlQueryCompiler.compile(core)
        }
        #[cfg(not(feature = "compiler-sql"))]
        {
            Err(crate::error::Error::Internal(
                "hybrid compiler requires compiler-sql for non-hop queries".into(),
            ))
        }
    }
}

/// Attempt to build a hybrid hop JSON value for M2M navigation.
fn try_m2m_hop_plan(core: &QueryCore) -> Result<Option<serde_json::Value>> {
    let Some(hop) = core.hop_source.as_ref() else {
        return Ok(None);
    };
    let HopType::ManyToManyForward { edge_table } = &hop.hop_type else {
        return Ok(None);
    };
    if !residual_is_equality_only(core) {
        return Ok(None);
    }

    let mut source = (*hop.source_query).clone();
    source.projection = Some(vec!["id".into()]);
    let (source_sql, source_params) = source.to_sql()?;

    let mut residual = core.clone();
    residual.hop_source = None;
    let (residual_sql, residual_params) = residual.to_sql()?;

    Ok(Some(json!({
        "hybrid_hop": {
            "source_sql": source_sql,
            "source_params": source_params,
            "source_table": hop.source_query.table,
            "edge_table": edge_table,
            "target_table": core.table,
            "residual_sql": residual_sql,
            "residual_params": residual_params,
        }
    })))
}

/// Only accelerate when residual filters are equality-friendly (or empty besides hop).
fn residual_is_equality_only(core: &QueryCore) -> bool {
    core.where_clauses.iter().all(|clause| {
        matches!(
            clause,
            WhereClause::Int(..)
                | WhereClause::String(..)
                | WhereClause::DateTime(..)
                | WhereClause::Record(..)
                | WhereClause::Null(..)
                | WhereClause::Hop(..)
        )
    }) && core.or_groups.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::{HopSource, HopType, QueryCore};

    #[test]
    fn emits_hop_plan_for_m2m() {
        let source = QueryCore::new("org".into());
        let mut target = QueryCore::new("project".into());
        target.hop_source = Some(HopSource {
            source_query: Box::new(source),
            hop_type: HopType::ManyToManyForward {
                edge_table: "org_projects".into(),
            },
        });
        let cq = HybridQueryCompiler.compile(&target).expect("compile");
        assert!(cq.query_string.contains("hybrid_hop"));
        assert!(cq.query_string.contains("org_projects"));
    }

    #[test]
    fn passthrough_without_hop() {
        let core = QueryCore::new("counter".into());
        let cq = HybridQueryCompiler.compile(&core).expect("compile");
        assert!(cq.query_string.contains("FROM counter"));
        assert!(!cq.query_string.contains("hybrid_hop"));
    }
}
