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
    
    // Initial scan
    tracing::info!("Performing initial directory scan...");
    stats.scan_directory(&path, &config.watch.ignore)?;
    stats.print_stats();
    
    // Create file watcher
    let watcher = aoike_core::FileWatcher::new(
        path.clone(),
        stats.clone(),
        config.watch.ignore.clone(),
    )?;
    
    // Start watching
    tracing::info!("Server is running. Press Ctrl+C to stop.");
    
    // Set up interval to print stats periodically
    let stats_clone = stats.clone();
    let stats_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            stats_clone.print_stats();
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
        _ = stats_task => {}
        _ = watcher_task => {}
    }
    
    // Final stats
    tracing::info!("Final statistics:");
    stats.print_stats();
    
    Ok(())
}
