use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct QuotaLimit {
    /// Maximum inferences per 24-hour window (None = unlimited)
    pub daily_max: Option<u64>,
    /// Maximum inferences per clock hour (None = unlimited)
    pub hourly_max: Option<u64>,
}

#[derive(Debug, Default, Clone)]
pub struct QuotaUsage {
    pub daily_count: u64,
    pub hourly_count: u64,
}

#[derive(Debug, Default)]
pub struct QuotaEngine {
    limits: HashMap<Uuid, QuotaLimit>,
    usage: HashMap<Uuid, QuotaUsage>,
}

impl QuotaEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_limit(&mut self, device_id: Uuid, limit: QuotaLimit) {
        self.limits.insert(device_id, limit);
    }

    pub fn record_inference(&mut self, device_id: Uuid) {
        let u = self.usage.entry(device_id).or_default();
        u.daily_count += 1;
        u.hourly_count += 1;
    }

    pub fn reset_hourly(&mut self) {
        for u in self.usage.values_mut() {
            u.hourly_count = 0;
        }
    }

    pub fn reset_daily(&mut self) {
        for u in self.usage.values_mut() {
            u.daily_count = 0;
            u.hourly_count = 0;
        }
    }

    pub fn is_allowed(&self, device_id: &Uuid) -> bool {
        let Some(limit) = self.limits.get(device_id) else {
            return true;
        };
        let usage = self.usage.get(device_id).cloned().unwrap_or_default();
        if let Some(d) = limit.daily_max {
            if usage.daily_count >= d {
                return false;
            }
        }
        if let Some(h) = limit.hourly_max {
            if usage.hourly_count >= h {
                return false;
            }
        }
        true
    }

    pub fn usage_for(&self, device_id: &Uuid) -> QuotaUsage {
        self.usage.get(device_id).cloned().unwrap_or_default()
    }
}
