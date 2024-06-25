use core::fmt;
use std::{collections::HashMap, sync::{atomic::{self, AtomicU16}, Arc}, thread, time::Duration};

use anyhow::{anyhow, Result};


#[derive(Debug, Clone)]
pub struct AmapMetrics {
    data: Arc<HashMap<String, atomic::AtomicU16>>
}

impl AmapMetrics {
    pub fn new(metri_names: &[&'static str]) -> Self {
        let map = metri_names.iter().map(|name| {
            (name.to_string(), AtomicU16::new(0))
        }).collect();
        Self { data: Arc::new(map) }
    }

    pub fn inc(&self, key: &str) -> Result<()> {
        let tom = self.data.get(key).ok_or_else(|| anyhow!("key not found"))?;
        tom.fetch_add(1, atomic::Ordering::Relaxed);
        Ok(())
    }

    pub fn dec(&self, key: &str) -> Result<()> {
        let tom = self.data.get(key).ok_or_else(|| anyhow!("key not found"))?;
        tom.fetch_sub(1, atomic::Ordering::Relaxed);
        Ok(())
    }
}

impl fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, v) in self.data.iter() {
            write!(f, "{}: {}\n", k, v.load(atomic::Ordering::Relaxed))?;
        }
        write!(f, "\n")
    }
}

pub fn create_worker(idx: usize, m: AmapMetrics) -> Result<()> {
    thread::spawn(move || -> Result<()> {
        loop {
            thread::sleep(Duration::from_secs(1));
            m.inc(format!("create/api/v1/{}", idx).as_str())
            .map_err(|e| anyhow!("{}", e.to_string()))?;
        }

        #[allow(unreachable_code)]
        Ok(())
    });
    Ok(())
}

pub fn move_worker(m: AmapMetrics) -> Result<()> {
    thread::spawn(move || -> Result<()> {
        loop {
            thread::sleep(Duration::from_secs(2));
            m.inc("move/api/v1").map_err(|e| anyhow!("{}", e.to_string()))?;
        }

        #[allow(unreachable_code)]
        Ok(())
    });
    Ok(())
}


#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::{create_worker, move_worker, AmapMetrics};

    #[test]
    fn test_amap_metrics() {
        let m = AmapMetrics::new(&[
            "create/api/v1/0",
            "create/api/v1/1",
            "move/api/v1",
        ]);

        // for create
        for idx in 0..2 {
            let _ = create_worker(idx, m.clone());
        }

        // for move
        for _ in 0..4 {
            let _ = move_worker(m.clone());
        }

        // print
        loop {
            println!("{}", m);
            thread::sleep(Duration::from_secs(5));
        }
    }
}
