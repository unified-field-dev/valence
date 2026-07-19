use std::sync::Arc;

use valence_backend_mem::InMemoryBackend;
use valence_testkit::run_backend_contract;

#[tokio::test]
async fn mem_backend_passes_port_contract() {
    let backend = Arc::new(InMemoryBackend::new());
    run_backend_contract(backend)
        .await
        .expect("backend contract");
}
