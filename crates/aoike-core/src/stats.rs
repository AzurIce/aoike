use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct FileStats {
    pub counts: Arc<Mutex<HashMap<String, usize>>>,
    pub total: Arc<Mutex<usize>>,
}

impl FileStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scan_directory(&self, path: &Path, ignore_patterns: &[String]) -> anyhow::Result<()> {
        let mut counts = self.counts.lock().unwrap();
        let mut total = self.total.lock().unwrap();
        
        counts.clear();
        *total = 0;
        
        self.scan_recursive(path, ignore_patterns, &mut counts, &mut total)?;
        
        Ok(())
    }

    fn scan_recursive(
        &self,
        path: &Path,
        ignore_patterns: &[String],
        counts: &mut HashMap<String, usize>,
        total: &mut usize,
    ) -> anyhow::Result<()> {
        if !path.is_dir() {
            return Ok(());
        }

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // 检查是否忽略
            if should_ignore(&name, ignore_patterns) {
                continue;
            }

            if path.is_dir() {
                self.scan_recursive(&path, ignore_patterns, counts, total)?;
            } else if path.is_file() {
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("no_extension")
                    .to_lowercase();
                
                *counts.entry(ext).or_insert(0) += 1;
                *total += 1;
            }
        }

        Ok(())
    }

    pub fn add_file(&self, path: &Path) {
        if !path.is_file() {
            return;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("no_extension")
            .to_lowercase();

        let mut counts = self.counts.lock().unwrap();
        let mut total = self.total.lock().unwrap();
        
        *counts.entry(ext).or_insert(0) += 1;
        *total += 1;
    }

    pub fn remove_file(&self, path: &Path) {
        if !path.is_file() {
            return;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("no_extension")
            .to_lowercase();

        let mut counts = self.counts.lock().unwrap();
        let mut total = self.total.lock().unwrap();
        
        if let Some(count) = counts.get_mut(&ext) {
            if *count > 0 {
                *count -= 1;
                *total -= 1;
            }
            if *count == 0 {
                counts.remove(&ext);
            }
        }
    }

    pub fn get_stats(&self) -> (HashMap<String, usize>, usize) {
        let counts = self.counts.lock().unwrap().clone();
        let total = *self.total.lock().unwrap();
        (counts, total)
    }

    pub fn print_stats(&self) {
        let (counts, total) = self.get_stats();
        
        println!("\n=== File Statistics ===");
        println!("Total files: {}", total);
        println!("\nBy extension:");
        
        let mut items: Vec<_> = counts.iter().collect();
        items.sort_by(|a, b| b.1.cmp(a.1));
        
        for (ext, count) in items {
            println!("  {:20} {}", format!(".{}", ext), count);
        }
        println!("======================\n");
    }
}

fn should_ignore(name: &str, patterns: &[String]) -> bool {
    for pattern in patterns {
        if glob_match(pattern, name) {
            return true;
        }
    }
    false
}

fn glob_match(pattern: &str, name: &str) -> bool {
    // 简单实现：支持 * 通配符和精确匹配
    if pattern == name {
        return true;
    }
    
    if pattern.starts_with("*.") {
        let suffix = &pattern[1..]; // .ext
        if name.ends_with(suffix) {
            return true;
        }
    }
    
    if pattern.contains('*') {
        // 简单的 glob 匹配，这里只做前缀/后缀匹配
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
