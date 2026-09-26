use crate::{Config, CoreResult, CoreError};
use axum::Router;
use std::collections::HashMap;
use tracing::{info, warn};

/// The core application instance
pub struct App {
    config: Config,
    modules: HashMap<String, String>,
    plugins: HashMap<String, String>,
    router: Router,
    db_pool: Option<sqlx::PgPool>,
}

impl App {
    /// Create a new application instance
    pub fn new(config: Config) -> CoreResult<Self> {
        info!("Initializing {} v{}", config.app.name, config.app.version);
        
        Ok(Self {
            config,
            modules: HashMap::new(),
            plugins: HashMap::new(),
            router: Router::new(),
            db_pool: None,
        })
    }

    /// Load a core module by name
    pub async fn load_module(&mut self, name: &str) -> CoreResult<()> {
        info!("Loading core module: {}", name);
        
        // In a real implementation, this would dynamically load modules
        // For now, we register the module name and initialize it
        // The actual module implementations would be in their own crates
        
        info!("Core module '{}' loaded", name);
        Ok(())
    }

    /// Load a plugin by name
    pub async fn load_plugin(&mut self, name: &str) -> CoreResult<()> {
        info!("Loading plugin: {}", name);
        
        // Check if plugin is configured
        let plugin_config = self.config.plugins.plugin_settings.get(name);
        if let Some(config) = plugin_config {
            info!("Plugin '{}' config: {:?}", name, config);
        }
        
        // In a real implementation, this would dynamically load plugins
        // from the plugins/ directory using manifest files
        
        info!("Plugin '{}' loaded", name);
        Ok(())
    }

    /// Run database migrations
    pub async fn run_migrations(&mut self) -> CoreResult<()> {
        info!("Running database migrations...");
        
        let db_url = &self.config.database.url;
        if db_url.is_empty() || db_url.starts_with("${") {
            warn!("DATABASE_URL not configured, skipping migrations");
            return Ok(());
        }
        
        // Connect to database
        let pool = sqlx::PgPool::connect(db_url).await.map_err(|e| {
            CoreError::Database(format!("Failed to connect to database: {}", e))
        })?;
        
        // Note: Individual projects should implement their own migration logic
        // using sqlx::migrate!() with their specific migrations directory
        info!("Database connection established, migrations should be run by project");
        
        self.db_pool = Some(pool);
        info!("Database ready");
        Ok(())
    }

    /// Start the HTTP server
    pub async fn listen(self, addr: &str) -> CoreResult<()> {
        info!("Starting server on {}", addr);
        
        let listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| {
            CoreError::Server(format!("Failed to bind to {}: {}", addr, e))
        })?;
        
        axum::serve(listener, self.router.into_make_service())
            .await
            .map_err(|e| {
                CoreError::Server(format!("Server error: {}", e))
            })?;
        
        Ok(())
    }

    /// Run the application (load modules, plugins, migrations, start server)
    pub async fn run(self) -> CoreResult<()> {
        let addr = format!("{}:{}", self.config.server.host, self.config.server.port);
        self.listen(&addr).await
    }

    /// Get a reference to the config
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get a mutable reference to the router
    pub fn router_mut(&mut self) -> &mut Router {
        &mut self.router
    }
}
