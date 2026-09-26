use serde::{Deserialize, Serialize};
use std::path::Path;

/// Application configuration loaded from TOML
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub app: AppConfig,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub features: FeaturesConfig,
    pub plugins: PluginsConfig,
    pub security: SecurityConfig,
    pub branding: BrandingConfig,
    pub jwt: JwtConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app: AppConfig::default(),
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            redis: RedisConfig::default(),
            features: FeaturesConfig::default(),
            plugins: PluginsConfig::default(),
            security: SecurityConfig::default(),
            branding: BrandingConfig::default(),
            jwt: JwtConfig::default(),
        }
    }
}

impl Config {
    /// Load configuration from a TOML file path
    pub fn load<P: AsRef<Path>>(path: P) -> crate::CoreResult<Self> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            crate::CoreError::Config(format!(
                "Failed to read config file '{}': {}",
                path.as_ref().display(),
                e
            ))
        })?;

        // Expand environment variables in the form ${VAR_NAME}
        let expanded = Self::expand_env_vars(&content);

        let config: Config = toml::from_str(&expanded).map_err(|e| {
            crate::CoreError::Config(format!(
                "Failed to parse config file '{}': {}",
                path.as_ref().display(),
                e
            ))
        })?;

        Ok(config)
    }

    /// Expand ${VAR} references in config strings
    fn expand_env_vars(input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '$' && chars.peek() == Some(&'{') {
                chars.next(); // consume '{'
                let mut var_name = String::new();
                for c in chars.by_ref() {
                    if c == '}' {
                        break;
                    }
                    var_name.push(c);
                }
                match std::env::var(&var_name) {
                    Ok(val) => result.push_str(&val),
                    Err(_) => result.push_str(&format!("${{{}}}", var_name)),
                }
            } else {
                result.push(ch);
            }
        }

        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub name: String,
    pub description: String,
    pub domain: String,
    pub version: String,
    pub secondary_domain: Option<String>,
    pub brand: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: String::from("app"),
            description: String::new(),
            domain: String::from("localhost"),
            version: String::from("0.1.0"),
            secondary_domain: None,
            brand: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: String::from("0.0.0.0"),
            port: 3000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
    pub max_connections: Option<u32>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: String::from("postgres://localhost:5432/app"),
            pool_size: 10,
            max_connections: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RedisConfig {
    pub url: String,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: String::from("redis://localhost:6379"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FeaturesConfig {
    pub enable_cms: bool,
    pub enable_crm: bool,
    pub enable_social: bool,
    pub enable_billing: bool,
    pub enable_search: bool,
    pub enable_notifications: bool,
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            enable_cms: true,
            enable_crm: false,
            enable_social: false,
            enable_billing: false,
            enable_search: false,
            enable_notifications: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PluginsConfig {
    pub active: Vec<String>,
    /// Capture plugin-specific config sections as arbitrary tables
    #[serde(flatten)]
    pub plugin_settings: std::collections::HashMap<String, toml::Value>,
}

impl Default for PluginsConfig {
    fn default() -> Self {
        Self {
            active: Vec::new(),
            plugin_settings: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub session_timeout_hours: u64,
    pub handshake_secret: Option<String>,
    pub handshake_ttl_seconds: Option<u64>,
    pub allow_custom_scripts: Option<bool>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            jwt_secret: String::from("change-me-in-production"),
            session_timeout_hours: 24,
            handshake_secret: None,
            handshake_ttl_seconds: None,
            allow_custom_scripts: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BrandingConfig {
    pub logo: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub accent_color: Option<String>,
    pub font_family: String,
}

impl Default for BrandingConfig {
    fn default() -> Self {
        Self {
            logo: String::new(),
            primary_color: String::from("#000000"),
            secondary_color: String::from("#FFFFFF"),
            accent_color: None,
            font_family: String::from("sans-serif"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct JwtConfig {
    pub secret: String,
    pub expiry_hours: u64,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: String::from("change-me-in-production"),
            expiry_hours: 24,
        }
    }
}
