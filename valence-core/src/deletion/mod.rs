//! Cascading delete orchestration — dispatch hooks and DAG planning.
//!
//! Hosts register a deletion dispatcher at boot; [`DeletionService`] coordinates graph expansion
//! via the [`dag`] submodule.
mod dispatch;
mod service;

pub mod dag;

pub use dispatch::{
    dispatch, is_deletion_dispatcher_registered, register_deletion_dispatcher,
    register_noop_deletion_dispatcher_for_tests, DeletionRequest,
};
pub use service::DeletionService;
