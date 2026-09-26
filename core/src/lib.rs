//! Resightings Core Framework
//!
//! Provides the foundational architecture for multi-tenant applications:
//! - Configuration management (TOML-based)
//! - Application lifecycle (modules, plugins, migrations, server)
//! - Plugin loading system
//! - Module registration and dependency injection

mod config;
mod app;
mod plugin;
mod error;

pub use config::Config;
pub use app::App;
pub use plugin::PluginLoader;
pub use error::{CoreError, CoreResult};

/// Module trait for core functionality
#[async_trait::async_trait]
pub trait Module: Send + Sync {
    /// Module name identifier
    fn name(&self) -> &str;
    
    /// Initialize the module
    async fn init(&mut self, config: &Config) -> CoreResult<()>;
    
    /// Register routes and middleware
    async fn register(&self, app: &mut App) -> CoreResult<()>;
}

/// Plugin trait for project-specific extensions
#[async_trait::async_trait]
pub trait Plugin: Send + Sync {
    /// Plugin name identifier
    fn name(&self) -> &str;
    
    /// Initialize the plugin
    async fn init(&mut self, config: &Config) -> CoreResult<()>;
    
    /// Register routes and middleware
    async fn register(&self, app: &mut App) -> CoreResult<()>;
}
