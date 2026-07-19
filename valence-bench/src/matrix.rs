//! CLI matrix flags → [`MatrixSpec`].

use anyhow::{bail, Result};
use valence_testkit::{MatrixSpec, StorageAdapter, TelemetryAdapter, Topology};

pub fn matrix_from_cli(storage: &str, telemetry: &str, topology: &str) -> Result<MatrixSpec> {
    let storage = StorageAdapter::parse_cli(storage)
        .ok_or_else(|| anyhow::anyhow!("unknown storage adapter: {storage}"))?;
    let telemetry = TelemetryAdapter::parse_cli(telemetry)
        .ok_or_else(|| anyhow::anyhow!("unknown telemetry adapter: {telemetry}"))?;
    let topology = Topology::parse_cli(topology)
        .ok_or_else(|| anyhow::anyhow!("unknown topology: {topology}"))?;

    if matches!(topology, Topology::RemoteStub) {
        bail!("remote-stub topology is not runnable in upstream bench");
    }

    Ok(MatrixSpec {
        storage,
        telemetry,
        topology,
    })
}
