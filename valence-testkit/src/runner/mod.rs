//! Shared scenario executor for e2e (correctness) and bench (timings).

mod steps;

use std::time::Instant;

use crate::bootstrap::BootstrapSession;
use crate::scenario::ScenarioSpec;

use steps::{run_step, step_label};

/// Driver mode: assert on outcomes vs collect timings only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunMode {
    /// Run assertion steps and fail on mismatches.
    Correctness,
    /// Skip assertions; record per-step timings for bench reports.
    Benchmark,
}

/// Per-step timing samples (milliseconds).
#[derive(Debug, Clone)]
pub struct StepTiming {
    /// Index of the step within the scenario.
    pub step_index: usize,
    /// Short operation label.
    pub op: String,
    /// Elapsed samples in milliseconds for this step.
    pub samples_ms: Vec<f64>,
}

/// Outcome of running one [`ScenarioSpec`].
#[derive(Debug, Clone)]
pub struct ScenarioResult {
    /// Scenario identifier from the spec.
    pub scenario_id: String,
    /// Matrix slug from the bootstrapped session.
    pub matrix_slug: String,
    /// Benchmark-mode timing samples per step.
    pub step_timings: Vec<StepTiming>,
    /// First step error message, if any.
    pub error: Option<String>,
}

/// Executes declarative scenarios against a bootstrapped session.
pub struct ScenarioRunner<'a> {
    pub(crate) session: &'a mut BootstrapSession,
}

impl<'a> ScenarioRunner<'a> {
    /// Bind a runner to an installed [`BootstrapSession`].
    pub fn new(session: &'a mut BootstrapSession) -> Self {
        Self { session }
    }

    /// Run all steps in `spec`, honoring `mode` for assertions vs timings.
    pub async fn run(
        &mut self,
        spec: &ScenarioSpec,
        mode: RunMode,
    ) -> Result<ScenarioResult, String> {
        if !self.session.is_ready() {
            return Err("BootstrapSession::spawn must succeed before running scenarios".into());
        }

        let matrix_slug = self.session.matrix().slug();
        let mut step_timings = Vec::new();
        let mut result = ScenarioResult {
            scenario_id: spec.id.clone(),
            matrix_slug,
            step_timings: Vec::new(),
            error: None,
        };

        for (step_index, step) in spec.steps.iter().enumerate() {
            let start = Instant::now();
            let step_result = run_step(self.session, step, mode).await;
            if mode == RunMode::Benchmark {
                step_timings.push(StepTiming {
                    step_index,
                    op: step_label(step),
                    samples_ms: vec![start.elapsed().as_secs_f64() * 1000.0],
                });
            }
            if let Err(e) = step_result {
                result.error = Some(e);
                result.step_timings = step_timings;
                return Ok(result);
            }
        }

        result.step_timings = step_timings;
        Ok(result)
    }
}
