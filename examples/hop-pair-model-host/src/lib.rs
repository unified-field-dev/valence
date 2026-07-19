//! Hop-pair model host: Project on hop_a, Task on hop_b (any physical backends).

use valence::{Database, DatabaseFromEngine};

/// Abstract engine id for the primary (Project) table.
pub const HOP_A: &str = "hop_a";
/// Abstract engine id for the secondary (Task) table.
pub const HOP_B: &str = "hop_b";

/// Project table routes to primary hop_a backend.
pub const PROJECT_DB: DatabaseFromEngine = Database::from_engine("primary", HOP_A);

/// Task table routes to secondary hop_b backend.
pub const TASK_DB: DatabaseFromEngine = Database::from_engine("secondary", HOP_B);

valence::include_generated_models!();

pub use generated::{HopPairProject as Project, HopPairTask as Task};
