//! Process-wide async lock for harness tests that share global telemetry / deletion state.

use tokio::sync::{Mutex, MutexGuard};

static HARNESS_LOCK: Mutex<()> = Mutex::const_new(());

/// Serialize integration harness entry points that mutate process-global sinks.
pub async fn lock_harness() -> MutexGuard<'static, ()> {
    HARNESS_LOCK.lock().await
}
