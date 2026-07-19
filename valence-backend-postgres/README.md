# valence-backend-postgres

Postgres wire [`DatabaseBackend`](../valence-core/src/backend/port.rs) adapter.

## Audience

| Reader | Use this crate for |
|--------|-------------------|
| **App developers** | Enable via `valence` feature `postgres` |
| **Host integrators** | Remote Postgres at boot |
| **Adapter authors** | Wire builder + `from_env_defaults` pattern |

```rust
pub const ENGINE_ID: &str = "postgres";
```

## Environment

| Variable | Meaning |
|----------|---------|
| `DATABASE_URL` | Postgres connection URL (required unless `.url()` is set) |

## Wiring

```rust
use std::sync::Arc;
use valence::{PostgresBackend, Valence};

// Explicit:
let backend = PostgresBackend::connect("postgres://localhost/valence").await?;
// Or env:
let backend = PostgresBackend::from_env().await?;

let valence = Valence::builder()
    .add_backend("default", Arc::new(backend))
    .build()?;
```

Builder: `PostgresBackend::builder().url(...).from_env_defaults().build().await?`

Runnable (skips when unset):

```bash
DATABASE_URL=postgres://localhost/valence \
  cargo run -p uf-valence --example quickstart_postgres --features postgres
```

See `DatabaseBackend` rustdoc (`cargo doc -p uf-valence-core --open`).
