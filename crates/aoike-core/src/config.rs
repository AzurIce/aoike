use std::path::{Path, PathBuf};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub control: ControlConfig,
    #[serde(default)]
    pub watch: WatchConfig,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_server_bind")]
    pub bind: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ControlConfig {
    #[serde(default = "default_control_bind")]
    pub bind: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WatchConfig {
    #[serde(default = "default_ignore")]
    pub ignore: Vec<String>,
}

fn default_server_bind() -> String {
    "127.0.0.1:8080".to_string()
}

fn default_control_bind() -> String {
    "127.0.0.1:8081".to_string()
}

fn default_ignore() -> Vec<String> {
    vec![
        ".git".to_string(),
        ".aoike".to_string(),
        "node_modules".to_string(),
        "*.tmp".to_string(),
    ]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            control: ControlConfig::default(),
            watch: WatchConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind: default_server_bind(),
        }
    }
}

impl Default for ControlConfig {
    fn default() -> Self {
        Self {
            bind: default_control_bind(),
        }
    }
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            ignore: default_ignore(),
        }
    }
}

impl Config {
    pub fn default_toml() -> String {
        r#"[server]
# Frontend HTTP API bind address
bind = "127.0.0.1:8080"

[control]
# Control port for CLI to daemon communication
bind = "127.0.0.1:8081"

[watch]
# Files/directories to ignore (glob patterns)
ignore = [".git", ".aoike", "node_modules", "*.tmp"]
"#
        .to_string()
    }

    pub fn config_path(vault_path: &Path) -> PathBuf {
        vault_path.join(".aoike").join("aoike.toml")
    }

    pub fn ensure_config_exists(vault_path: &Path) -> anyhow::Result<PathBuf> {
        let aoike_dir = vault_path.join(".aoike");
        let config_path = aoike_dir.join("aoike.toml");

        if !config_path.exists() {
            std::fs::create_dir_all(&aoike_dir)?;
            std::fs::write(&config_path, Self::default_toml())?;
            tracing::info!("Created default config at {:?}", config_path);
        }

        Ok(config_path)
    }
}

pub fn load_config(vault_path: &Path) -> anyhow::Result<Config> {
    let config_path = Config::ensure_config_exists(vault_path)?;
    
    let content = std::fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&content)?;
    
    Ok(config)
}

pub fn load_config_with_overrides(
    vault_path: &Path,
    server_bind: Option<String>,
    control_bind: Option<String>,
    ignore: Option<Vec<String>>,
) -> anyhow::Result<Config> {
    let mut config = load_config(vault_path)?;
    
    if let Some(bind) = server_bind {
        config.server.bind = bind;
    }
    
    if let Some(bind) = control_bind {
        config.control.bind = bind;
    }
    
    if let Some(ignore_list) = ignore {
        config.watch.ignore = ignore_list;
    }
    
    Ok(config)
}
