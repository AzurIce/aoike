use std::path::{Path, PathBuf};

use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;

use crate::stats::FileStats;
use crate::task::TaskIndex;

pub struct FileWatcher {
    _watcher: RecommendedWatcher,
    rx: mpsc::Receiver<notify::Result<Event>>,
    stats: FileStats,
    task_index: TaskIndex,
    vault_path: PathBuf,
    ignore_patterns: Vec<String>,
}

impl FileWatcher {
    pub fn new(
        vault_path: PathBuf,
        stats: FileStats,
        task_index: TaskIndex,
        ignore_patterns: Vec<String>,
    ) -> anyhow::Result<Self> {
        let (tx, rx) = mpsc::channel(100);

        let mut watcher = RecommendedWatcher::new(
            move |res: notify::Result<Event>| {
                let _ = tx.blocking_send(res);
            },
            Config::default(),
        )?;

        watcher.watch(&vault_path, RecursiveMode::Recursive)?;

        Ok(Self {
            _watcher: watcher,
            rx,
            stats,
            task_index,
            vault_path,
            ignore_patterns,
        })
    }

    pub async fn run(mut self) {
        tracing::info!("File watcher started for {:?}", self.vault_path);
        
        while let Some(res) = self.rx.recv().await {
            match res {
                Ok(event) => {
                    self.handle_event(event).await;
                }
                Err(e) => {
                    tracing::error!("Watch error: {}", e);
                }
            }
        }
    }

    async fn handle_event(&self, event: Event) {
        match event.kind {
            EventKind::Create(_) => {
                for path in &event.paths {
                    if !self.should_ignore(path) {
                        tracing::debug!("File created: {:?}", path);
                        self.stats.add_file(path);
                        self.update_tasks_for_file(path).await;
                    }
                }
            }
            EventKind::Modify(_) => {
                for path in &event.paths {
                    if !self.should_ignore(path) {
                        tracing::debug!("File modified: {:?}", path);
                        if path.exists() && path.is_file() {
                            // For stats, we don't need to do anything for modifications
                            // But for tasks, we need to re-parse
                            self.update_tasks_for_file(path).await;
                        }
                    }
                }
            }
            EventKind::Remove(_) => {
                for path in &event.paths {
                    if !self.should_ignore(path) {
                        tracing::debug!("File removed: {:?}", path);
                        self.stats.remove_file(path);
                        self.task_index.remove_file(path);
                    }
                }
            }
            _ => {}
        }
    }

    async fn update_tasks_for_file(&self,
        path: &Path,
    ) {
        // Only parse markdown files
        if let Some(ext) = path.extension() {
            if ext.to_string_lossy().to_lowercase() != "md" {
                return;
            }
        } else {
            return;
        }
        
        // Read file content
        match tokio::fs::read_to_string(path).await {
            Ok(content) => {
                self.task_index.scan_file(path, &content);
                tracing::debug!("Updated tasks for {:?}", path);
            }
            Err(e) => {
                tracing::warn!("Failed to read file {:?}: {}", path, e);
            }
        }
    }

    fn should_ignore(&self, path: &Path) -> bool {
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        for pattern in &self.ignore_patterns {
            if glob_match(pattern, name) {
                return true;
            }
        }

        // Also check if any parent directory matches ignore patterns
        for ancestor in path.ancestors() {
            if let Some(dir_name) = ancestor.file_name().and_then(|n| n.to_str()) {
                for pattern in &self.ignore_patterns {
                    if !pattern.contains('.') && !pattern.contains('*') {
                        if dir_name == pattern {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}

fn glob_match(pattern: &str, name: &str) -> bool {
    if pattern == name {
        return true;
    }
    
    if pattern.starts_with("*.") {
        let suffix = &pattern[1..];
        if name.ends_with(suffix) {
            return true;
        }
    }
    
    if pattern.contains('*') {
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            let prefix = parts[0];
            let suffix = parts[1];
            if name.starts_with(prefix) && name.ends_with(suffix) {
                return true;
            }
        }
    }
    
    false
}
