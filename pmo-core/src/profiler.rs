use std::time::{Duration, Instant};

/// A lightweight profiling session stub.
///
/// In production, replace with real Core ML Profiler FFI calls or
/// instruments-based measurements. See docs/aot_conversion.md.
#[derive(Debug)]
pub struct ProfilingSession {
    pub label: String,
    start: Option<Instant>,
    elapsed: Option<Duration>,
}

impl ProfilingSession {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), start: None, elapsed: None }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        if let Some(s) = self.start.take() {
            self.elapsed = Some(s.elapsed());
        }
    }

    pub fn elapsed_ms(&self) -> Option<f64> {
        self.elapsed.map(|d| d.as_secs_f64() * 1000.0)
    }

    pub fn is_complete(&self) -> bool {
        self.elapsed.is_some()
    }
}

/// Factory for profiling sessions. No-op when profiling is disabled.
pub struct ProfilingStub {
    enabled: bool,
}

impl ProfilingStub {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    pub fn session(&self, label: &str) -> Option<ProfilingSession> {
        if self.enabled {
            Some(ProfilingSession::new(label))
        } else {
            None
        }
    }
}
