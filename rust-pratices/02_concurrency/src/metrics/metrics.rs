use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    fmt,
};
#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn inc(&mut self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.write().map_err(|e| anyhow!(e.to_string()))?;

        let c = data.entry(key.into()).or_insert(0);
        *c += 1;

        Ok(())
    }

    pub fn dec(&mut self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.write().map_err(|e| anyhow!(e.to_string()))?;

        let c = data.entry(key.into()).or_insert(0);
        *c -= 1;

        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self
            .data
            .read()
            .map_err(|e| anyhow!(e.to_string()))?
            .clone())
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.data.read().map_err(|_| fmt::Error)?;
        for (k, v) in data.iter() {
            write!(f, "{}: {}\n", k, v)?;
        }
        Ok(())
    }
}
