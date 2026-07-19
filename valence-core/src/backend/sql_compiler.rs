//! SQL [`QueryCompiler`] for [`crate::query::QueryCore`].

use crate::compiled_query::CompiledQuery;
use crate::error::Result;
use crate::query::QueryCore;
use crate::query_compiler::QueryCompiler;

/// Compiles [`QueryCore`] to parameterized SQL.
#[derive(Debug, Default, Clone, Copy)]
pub struct SqlQueryCompiler;

impl QueryCompiler for SqlQueryCompiler {
    fn compile(&self, core: &QueryCore) -> Result<CompiledQuery> {
        let (query_string, params) = core.to_sql()?;
        Ok(CompiledQuery::new(query_string, params))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::QueryCore;

    #[test]
    fn compiles_simple_select() {
        let core = QueryCore::new("counter".to_string());
        let cq = SqlQueryCompiler.compile(&core).expect("compile");
        assert!(
            cq.query_string.contains("FROM counter"),
            "unexpected SQL: {}",
            cq.query_string
        );
        assert!(
            cq.query_string.starts_with("SELECT "),
            "unexpected SQL: {}",
            cq.query_string
        );
    }
}
