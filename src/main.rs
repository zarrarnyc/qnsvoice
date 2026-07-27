use resightings_core::{App, Config, PluginLoader};
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting qnsvoice v0.1.0");

    let config = Config::load("config/app.toml")?;
    info!("Configuration loaded for {}", config.app.name);

    let mut app = App::new(config.clone())?;

    app.load_module("auth").await?;
    app.load_module("users").await?;
    app.load_module("tenants").await?;

    if config.features.enable_cms {
        app.load_module("cms").await?;
    }
    if config.features.enable_notifications {
        app.load_module("notifications").await?;
    }

    for plugin_name in &config.plugins.active {
        info!("Loading plugin: {}", plugin_name);
        app.load_plugin(plugin_name).await?;
    }

    info!("Running database migrations...");
    app.run_migrations().await?;

    let addr = format!("0.0.0.0:{}", config.server.port);
    info!("qnsvoice listening on {}", addr);

    app.listen(&addr).await?;

    Ok(())
}
