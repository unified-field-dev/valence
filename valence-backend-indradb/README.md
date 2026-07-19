# valence-backend-indradb

IndraDB embedded graph [`DatabaseBackend`](../valence-core/src/backend/port.rs) adapter.

## Audience

| Reader | Use this crate for |
|--------|-------------------|
| **App developers** | Enable via `valence` feature `indradb` |
| **Host integrators** | In-process graph store |
| **Adapter authors** | Graph-edge capability pattern |

```rust
pub const ENGINE_ID: &str = "indradb";
```

## Wiring

```rust
use std::sync::Arc;
use valence::{IndradbBackend, Valence};

let valence = Valence::builder()
    .add_backend("default", Arc::new(IndradbBackend::new()))
    .build()?;
```

Runnable: `cargo run -p uf-valence --example quickstart_indradb --features indradb`

See `DatabaseBackend` rustdoc (`cargo doc -p uf-valence-core --open`).
