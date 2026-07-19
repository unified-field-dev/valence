# valence-backend-redis

Redis wire [`DatabaseBackend`](../valence-core/src/backend/port.rs) adapter.

## Audience

| Reader | Use this crate for |
|--------|-------------------|
| **App developers** | Enable via `valence` feature `redis` |
| **Host integrators** | Remote Redis / fleet at boot |
| **Adapter authors** | Wire builder + fleet routing pattern |

```rust
pub const ENGINE_ID: &str = "redis";
```

## Environment

| Variable | Meaning |
|----------|---------|
| `VALENCE_REDIS_URL` | Redis URL (`redis://…`) |
| `VALENCE_TEST_REDIS_URL` | Fallback test URL |
| `VALENCE_REDIS_KEY_PREFIX` | Key prefix (default `valence`) |
| `VALENCE_REDIS_URLS` | Comma-separated fleet URLs |

## Wiring

```rust
use std::sync::Arc;
use valence::{RedisBackend, Valence};

let backend = RedisBackend::from_env().await?;
// Or: RedisBackend::connect("redis://127.0.0.1:6379").await?;

let valence = Valence::builder()
    .add_backend("cache", Arc::new(backend))
    .build()?;
```

Runnable (skips when unset):

```bash
VALENCE_REDIS_URL=redis://127.0.0.1:6379 \
  cargo run -p uf-valence --example quickstart_redis --features redis
```

See `DatabaseBackend` rustdoc (`cargo doc -p uf-valence-core --open`).
