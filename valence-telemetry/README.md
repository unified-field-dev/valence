# valence-telemetry

`TelemetrySink` trait with `NoOpSink`, `ConsoleSink`, and `RecordingSink`.

## Audience

- **Application developers** — use `NoOpSink` default via builder.
- **Host integrators** — inject custom sinks from separate adapter crates.
- **Maintainers** — keep product telemetry out of this crate.
