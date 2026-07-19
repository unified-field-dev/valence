//! SurrealQL [`QueryCompiler`] for [`crate::query::QueryCore`].

use crate::compiled_query::CompiledQuery;
use crate::error::Result;
use crate::query::QueryCore;
use crate::query_compiler::QueryCompiler;

/// Compiles [`QueryCore`] to parameterized SurrealQL.
#[derive(Debug, Default, Clone, Copy)]
pub struct SurrealQueryCompiler;

impl QueryCompiler for SurrealQueryCompiler {
    fn compile(&self, core: &QueryCore) -> Result<CompiledQuery> {
        let (query_string, params) = core.to_surrealql()?;
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
        let cq = SurrealQueryCompiler.compile(&core).expect("compile");
        assert!(cq.query_string.contains("SELECT * FROM counter"));
    }
}
