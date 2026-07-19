# valence-backend-mem

In-memory [`DatabaseBackend`](../valence-core/src/backend/port.rs) reference adapter.

## Audience

| Reader | Use this crate for |
|--------|-------------------|
| **App developers** | Enable via `valence` feature `mem` (default) |
| **Host integrators** | Default embedded storage; `install_default_mem_router()` for tests |
| **Adapter authors** | Template for third-party `ENGINE_ID` + port impl |

```rust
pub const ENGINE_ID: &str = "inmemory_mem";
```

## Wiring

```rust
use std::sync::Arc;
use valence::{InMemoryBackend, Valence};

let valence = Valence::builder()
    .add_backend("default", Arc::new(InMemoryBackend::new()))
    .build()?;
```

Runnable: `cargo run -p uf-valence --example quickstart --features mem`
