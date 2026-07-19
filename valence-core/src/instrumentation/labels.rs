//! Stable label value helpers for Valence telemetry.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadOp {
    Get,
    Query,
}

impl ReadOp {
    pub fn as_str(self) -> &'static str {
        match self {
            ReadOp::Get => "get",
            ReadOp::Query => "query",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteOp {
    Create,
    Update,
    Upsert,
    Merge,
    Delete,
}

impl WriteOp {
    pub fn as_str(self) -> &'static str {
        match self {
            WriteOp::Create => "create",
            WriteOp::Update => "update",
            WriteOp::Upsert => "upsert",
            WriteOp::Merge => "merge",
            WriteOp::Delete => "delete",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeWriteOp {
    Relate,
    Unrelate,
}

impl EdgeWriteOp {
    pub fn as_str(self) -> &'static str {
        match self {
            EdgeWriteOp::Relate => "relate",
            EdgeWriteOp::Unrelate => "unrelate",
        }
    }
}
