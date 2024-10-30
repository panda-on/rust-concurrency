use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

#[derive(Debug, Clone)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        Self {
            data: Arc::new(map),
        }
    }

    pub fn incr(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow!("key: {} not found", key))?;
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (&key, counter) in self.data.iter() {
            writeln!(f, "{}: {}", key, counter.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}
