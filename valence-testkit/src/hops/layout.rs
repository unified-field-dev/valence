//! Data-driven hop layout descriptors (no hand-written Cartesian tests).

use crate::hops::engines::hop_storage_engines;
use crate::matrix::StorageAdapter;

/// Directed pair of distinct storage adapters (depth-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HopPair {
    pub primary: StorageAdapter,
    pub secondary: StorageAdapter,
}

impl HopPair {
    pub fn slug(self) -> String {
        format!("{}->{}", self.primary.slug(), self.secondary.slug())
    }
}

/// Ordered distinct triple (depth-3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HopTriple {
    pub a: StorageAdapter,
    pub b: StorageAdapter,
    pub c: StorageAdapter,
}

impl HopTriple {
    pub fn slug(self) -> String {
        format!("{}->{}->{}", self.a.slug(), self.b.slug(), self.c.slug())
    }
}

/// Ordered distinct quad (depth-4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HopQuad {
    pub a: StorageAdapter,
    pub b: StorageAdapter,
    pub c: StorageAdapter,
    pub d: StorageAdapter,
}

impl HopQuad {
    pub fn slug(self) -> String {
        format!(
            "{}->{}->{}->{}",
            self.a.slug(),
            self.b.slug(),
            self.c.slug(),
            self.d.slug()
        )
    }
}

/// All directed pairs `E1 ≠ E2` (full Cartesian).
pub fn directed_pairs() -> Vec<HopPair> {
    let engines = hop_storage_engines();
    let mut out = Vec::new();
    for &primary in &engines {
        for &secondary in &engines {
            if primary != secondary {
                out.push(HopPair { primary, secondary });
            }
        }
    }
    out
}

/// Representative depth-3 chains for AWS (avoids full combinatorial explosion).
pub fn hop_triples_representative() -> Vec<HopTriple> {
    use StorageAdapter::*;
    let candidates = [
        [Mem, Sqlite, SurrealMem],
        [Sqlite, Mem, IndraDb],
        [SurrealMem, Sqlite, Mem],
        [Mem, IndraDb, Sqlite],
        [Postgres, Sqlite, Mem],
        [MongoDb, Mem, Sqlite],
        [Redis, Mem, Sqlite],
        [SurrealRocksdb, Sqlite, Mem],
        [IndraDb, SurrealMem, Sqlite],
        [Sqlite, Postgres, Mem],
        [Mem, MongoDb, Redis],
        [SurrealMem, Mem, Postgres],
    ];
    candidates
        .into_iter()
        .filter(|[a, b, c]| a != b && b != c && a != c)
        .map(|[a, b, c]| HopTriple { a, b, c })
        .collect()
}

/// Representative depth-4 chains.
pub fn hop_quads_representative() -> Vec<HopQuad> {
    use StorageAdapter::*;
    let candidates = [
        [Mem, Sqlite, SurrealMem, IndraDb],
        [Sqlite, Mem, IndraDb, SurrealMem],
        [SurrealMem, Sqlite, Mem, IndraDb],
        [Mem, Sqlite, Postgres, Redis],
        [Mem, Sqlite, MongoDb, Redis],
        [Postgres, Sqlite, Mem, SurrealMem],
        [Redis, Mem, Sqlite, IndraDb],
        [SurrealRocksdb, Sqlite, Mem, IndraDb],
    ];
    candidates
        .into_iter()
        .filter(|t| {
            let s = [t[0], t[1], t[2], t[3]];
            (0..4).all(|i| (i + 1..4).all(|j| s[i] != s[j]))
        })
        .map(|[a, b, c, d]| HopQuad { a, b, c, d })
        .collect()
}
