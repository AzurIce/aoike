mod monitor;
mod server;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aoike")]
#[command(about = "Aoike - Agentic personal knowledge management")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the server to watch a vault
    Serve {
        /// Path to the vault directory (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        
        /// Server bind address (overrides config)
        #[arg(long)]
        bind: Option<String>,
        
        /// Control port bind address (overrides config)
        #[arg(long)]
        control_bind: Option<String>,
        
        /// Comma-separated list of patterns to ignore (overrides config)
        #[arg(long)]
        ignore: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Serve { path, bind, control_bind, ignore } => {
            run_serve(path, bind, control_bind, ignore).await?;
        }
    }
    
    Ok(())
}

async fn run_serve(
    path: PathBuf,
    bind: Option<String>,
    control_bind: Option<String>,
    ignore: Option<String>,
) -> anyhow::Result<()> {
    // Resolve absolute path
    let path = std::fs::canonicalize(&path)
        .unwrap_or_else(|_| path.clone());
    
    tracing::info!("Starting Aoike server for vault: {:?}", path);
    
    // Parse ignore list
    let ignore_list = ignore.map(|s| {
        s.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>()
    });
    
    // Load config (creates default if not exists)
    let config = aoike_core::load_config_with_overrides(
        &path,
        bind,
        control_bind,
        ignore_list,
    )?;
    
    tracing::info!("Configuration loaded: {:?}", config);
    tracing::info!("Server will bind to: {}", config.server.bind);
    tracing::info!("Control port will bind to: {}", config.control.bind);
    
    // Create stats tracker
    let stats = aoike_core::FileStats::new();
    
    // Create task index
    let task_index = aoike_core::TaskIndex::new();
    
    // Create system monitor
    let monitor = monitor::SystemMonitor::new();
    
    // Initial scan
    tracing::info!("Performing initial directory scan...");
    stats.scan_directory(&path, &config.watch.ignore)?;
    stats.print_stats();
    
    // Initial task scan - scan all markdown files
    tracing::info!("Scanning for tasks in markdown files...");
    scan_markdown_tasks(&path, &config.watch.ignore, &task_index).await?;
    let (todo_count, done_count) = task_index.get_stats();
    tracing::info!("Found {} todo tasks, {} done tasks", todo_count, done_count);
    
    // Create file watcher
    let watcher = aoike_core::FileWatcher::new(
        path.clone(),
        stats.clone(),
        task_index.clone(),
        config.watch.ignore.clone(),
    )?;
    
    // Start HTTP server
    let server_bind = config.server.bind.clone();
    let stats_for_server = stats.clone();
    let task_index_for_server = task_index.clone();
    let monitor_for_server = monitor.clone();
    let server_task = tokio::spawn(async move {
        if let Err(e) = server::run_server(
            &server_bind,
            stats_for_server,
            task_index_for_server,
            monitor_for_server,
        ).await {
            tracing::error!("Server error: {}", e);
        }
    });
    
    tracing::info!("Server is running. Press Ctrl+C to stop.");
    tracing::info!("Open http://{} in your browser to view dashboard", config.server.bind);
    
    // Set up interval to print stats periodically
    let stats_clone = stats.clone();
    let task_index_clone = task_index.clone();
    let monitor_clone = monitor.clone();
    let stats_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            let (counts, total) = stats_clone.get_stats();
            let mut items: Vec<_> = counts.iter().collect();
            items.sort_by(|a, b| b.1.cmp(a.1));
            let ext_summary = items.iter()
                .map(|(ext, count)| format!("{}: {}", ext, count))
                .collect::<Vec<_>>()
                .join(", ");
            
            let (todo, done) = task_index_clone.get_stats();
            let total_tasks = todo + done;
            
            if let Some(info) = monitor_clone.get_info(total_tasks, todo, done) {
                tracing::info!(
                    "Files: {} ({}) | Tasks: {} todo, {} done | Memory: {:.1} MB ({:.1}%) | CPU: {:.1}%",
                    total, ext_summary, todo, done, info.memory_mb, info.memory_percent, info.cpu_percent
                );
            } else {
                tracing::info!(
                    "Files: {} ({}) | Tasks: {} todo, {} done",
                    total, ext_summary, todo, done
                );
            }
        }
    });
    
    // Run watcher
    let watcher_task = tokio::spawn(async move {
        watcher.run().await;
    });
    
    // Wait for shutdown signal
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Received shutdown signal");
        }
        _ = server_task => {}
        _ = stats_task => {}
        _ = watcher_task => {}
    }
    
    // Final stats
    tracing::info!("Final statistics:");
    stats.print_stats();
    let (todo, done) = task_index.get_stats();
    tracing::info!("Final tasks: {} todo, {} done", todo, done);
    
    Ok(())
}

async fn scan_markdown_tasks(
    path: &std::path::Path,
    ignore_patterns: &[String],
    task_index: &aoike_core::TaskIndex,
) -> anyhow::Result<()> {
    scan_markdown_tasks_recursive(path, ignore_patterns, task_index).await?;
    Ok(())
}

fn should_ignore(name: &str, patterns: &[String]) -> bool {
    for pattern in patterns {
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
    }
    false
}

async fn scan_markdown_tasks_recursive(
    path: &std::path::Path,
    ignore_patterns: &[String],
    task_index: &aoike_core::TaskIndex,
) -> anyhow::Result<()> {
    if !path.is_dir() {
        return Ok(());
    }

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        // Check if ignored
        if should_ignore(&name, ignore_patterns) {
            continue;
        }

        if path.is_dir() {
            Box::pin(scan_markdown_tasks_recursive(
                &path,
                ignore_patterns,
                task_index,
            )).await?;
        } else if path.is_file() {
            // Only scan markdown files
            if let Some(ext) = path.extension() {
                if ext.to_string_lossy().to_lowercase() == "md" {
                    match tokio::fs::read_to_string(&path).await {
                        Ok(content) => {
                            task_index.scan_file(&path, &content);
                        }
                        Err(e) => {
                            tracing::warn!("Failed to read file {:?}: {}", path, e);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
