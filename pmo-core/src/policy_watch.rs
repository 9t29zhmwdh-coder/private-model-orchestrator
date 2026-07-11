//! Hot-reloads an `MdmPolicy` from a local JSON file whenever it changes on
//! disk, so an MDM-pushed Configuration Profile payload can be picked up
//! without restarting `pmo-cli` or `pmo-macos`.

use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, TryRecvError};

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::policy::MdmPolicy;

#[derive(Debug, thiserror::Error)]
pub enum PolicyWatchError {
    #[error("failed to watch policy file: {0}")]
    Watch(#[from] notify::Error),
    #[error("failed to read policy file: {0}")]
    Io(#[from] std::io::Error),
    #[error("policy file is not valid JSON: {0}")]
    Json(#[from] serde_json::Error),
}

pub struct PolicyWatcher {
    _watcher: RecommendedWatcher,
    rx: Receiver<notify::Result<Event>>,
    path: PathBuf,
}

impl PolicyWatcher {
    /// Starts watching `path` for changes. Call [`Self::load`] once up front
    /// to get the initial policy, then poll [`Self::poll_reload`] periodically
    /// (e.g. once per CLI invocation, or on a timer in a long-running app).
    pub fn new(path: impl AsRef<Path>) -> Result<Self, PolicyWatchError> {
        let path = path.as_ref().to_path_buf();
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(tx)?;
        watcher.watch(&path, RecursiveMode::NonRecursive)?;
        Ok(Self { _watcher: watcher, rx, path })
    }

    /// Reads and parses the policy file as it is right now, regardless of
    /// whether it has changed.
    pub fn load(&self) -> Result<MdmPolicy, PolicyWatchError> {
        let contents = std::fs::read_to_string(&self.path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    /// Drains pending filesystem events without blocking. Returns the
    /// reloaded policy if the file was modified since the last call, or
    /// `None` if nothing changed (or the reload failed, e.g. the file was
    /// mid-write and contained invalid JSON: the caller keeps the last
    /// good policy and can retry on the next poll).
    pub fn poll_reload(&self) -> Option<MdmPolicy> {
        let mut changed = false;
        loop {
            match self.rx.try_recv() {
                Ok(Ok(event)) if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) => {
                    changed = true;
                }
                Ok(_) => continue,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }
        if changed { self.load().ok() } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::time::Duration;

    fn write_policy(path: &Path, policy: &MdmPolicy) {
        let mut f = std::fs::File::create(path).unwrap();
        f.write_all(serde_json::to_string(policy).unwrap().as_bytes()).unwrap();
    }

    #[test]
    fn loads_the_policy_file_as_written() {
        let dir = std::env::temp_dir().join(format!("pmo-policy-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("policy.json");
        write_policy(&path, &MdmPolicy { inference_allowed: true, ..Default::default() });

        let watcher = PolicyWatcher::new(&path).unwrap();
        let loaded = watcher.load().unwrap();
        assert!(loaded.inference_allowed);

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn poll_reload_picks_up_a_file_change() {
        let dir = std::env::temp_dir().join(format!("pmo-policy-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("policy.json");
        write_policy(&path, &MdmPolicy { inference_allowed: false, ..Default::default() });

        let watcher = PolicyWatcher::new(&path).unwrap();
        // Drain whatever the watcher setup itself may have surfaced (e.g. a
        // Create event for the file that already existed); only the change
        // written below is what this test actually asserts on.
        watcher.poll_reload();

        std::thread::sleep(Duration::from_millis(50));
        write_policy(&path, &MdmPolicy { inference_allowed: true, ..Default::default() });

        // Poll with a generous timeout instead of a single fixed-delay check:
        // filesystem event latency varies a lot between a local machine and
        // a loaded CI runner, so a single sleep-then-check is inherently flaky.
        let mut reloaded = None;
        for _ in 0..50 {
            if let Some(policy) = watcher.poll_reload() {
                reloaded = Some(policy);
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        let reloaded = reloaded.expect("file change should be picked up within 5s");
        assert!(reloaded.inference_allowed);

        std::fs::remove_dir_all(&dir).ok();
    }
}
