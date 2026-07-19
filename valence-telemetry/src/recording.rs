//! In-memory [`TelemetrySink`] for tests.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::TelemetrySink;

/// Captured counter increment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedCounter {
    /// Metric name.
    pub name: String,
    /// Label dimensions at emission time.
    pub labels: Vec<(String, String)>,
    /// Increment applied to the counter.
    pub delta: u64,
}

/// Captured gauge sample.
#[derive(Debug, Clone, PartialEq)]
pub struct RecordedGauge {
    /// Metric name.
    pub name: String,
    /// Label dimensions at emission time.
    pub labels: Vec<(String, String)>,
    /// Observed gauge value.
    pub value: f64,
}

/// Captured structured log event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedEvent {
    /// Event schema identifier.
    pub schema: String,
    /// Flattened field payload.
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Default)]
struct Inner {
    counters: Vec<RecordedCounter>,
    gauges: Vec<RecordedGauge>,
    events: Vec<RecordedEvent>,
}

/// Append-only in-memory sink for assertions in unit and integration tests.
#[derive(Debug, Clone)]
pub struct RecordingSink {
    inner: Arc<Mutex<Inner>>,
}

impl RecordingSink {
    /// Create an empty in-memory recording sink.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner::default())),
        }
    }

    fn lock_inner(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Return all recorded counter increments.
    pub fn counters(&self) -> Vec<RecordedCounter> {
        self.lock_inner().counters.clone()
    }

    /// Return all recorded structured events.
    pub fn events(&self) -> Vec<RecordedEvent> {
        self.lock_inner().events.clone()
    }

    /// Return all recorded gauge samples.
    pub fn gauges(&self) -> Vec<RecordedGauge> {
        self.lock_inner().gauges.clone()
    }

    /// Counters matching name and label subset (test helper).
    pub fn recorded_counters_matching(
        &self,
        name: &str,
        labels: &[(&str, &str)],
    ) -> Vec<RecordedCounter> {
        self.counters()
            .into_iter()
            .filter(|c| {
                c.name == name
                    && labels
                        .iter()
                        .all(|(k, v)| c.labels.iter().any(|(lk, lv)| lk == k && lv == v))
            })
            .collect()
    }

    /// Events matching schema name (test helper).
    pub fn recorded_events_for(&self, schema: &str) -> Vec<RecordedEvent> {
        self.events()
            .into_iter()
            .filter(|e| e.schema == schema)
            .collect()
    }
}

impl TelemetrySink for RecordingSink {
    fn record_counter(&self, name: &str, labels: &[(&str, &str)], delta: u64) {
        let labels: Vec<(String, String)> = labels
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let mut inner = self.lock_inner();
        inner.counters.push(RecordedCounter {
            name: name.to_string(),
            labels,
            delta,
        });
    }

    fn record_gauge(&self, name: &str, labels: &[(&str, &str)], value: f64) {
        let labels: Vec<(String, String)> = labels
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let mut inner = self.lock_inner();
        inner.gauges.push(RecordedGauge {
            name: name.to_string(),
            labels,
            value,
        });
    }

    fn log_event(&self, schema: &str, fields: &[(&str, &str)]) {
        let fields: HashMap<String, String> = fields
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let mut inner = self.lock_inner();
        inner.events.push(RecordedEvent {
            schema: schema.to_string(),
            fields,
        });
    }
}

impl Default for RecordingSink {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_counters_and_events() {
        let sink = RecordingSink::new();
        sink.record_counter("valence_queries", &[("table", "user")], 1);
        sink.log_event("valence.record.created", &[("table", "user")]);
        assert_eq!(sink.counters().len(), 1);
        assert_eq!(sink.events().len(), 1);
    }
}
