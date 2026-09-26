use axum::{response::Html, routing::get, Json};
use resightings_core::{App, Config};
use serde_json::json;
use tracing::{info, Level};

const COMING_SOON_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>QNS Voice — Coming Soon</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            font-family: Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
            color: #f8fafc;
            text-align: center;
            padding: 2rem;
        }
        .container { max-width: 640px; }
        h1 { font-size: clamp(2.5rem, 8vw, 4.5rem); font-weight: 800; letter-spacing: -0.04em; margin-bottom: 1rem; }
        h1 span { color: #2563EB; }
        p { font-size: 1.25rem; line-height: 1.6; color: #94a3b8; margin-bottom: 2rem; }
        .badge {
            display: inline-block;
            padding: 0.5rem 1rem;
            border-radius: 9999px;
            background: rgba(37, 99, 235, 0.15);
            color: #60a5fa;
            font-size: 0.875rem;
            font-weight: 600;
            letter-spacing: 0.05em;
            text-transform: uppercase;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="badge">Coming Soon</div>
        <h1>QNS <span>Voice</span></h1>
        <p>The community-driven news and voice platform for Queens is on its way. Stay tuned.</p>
    </div>
</body>
</html>"#;

async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

async fn index_handler() -> Html<&'static str> {
    Html(COMING_SOON_HTML)
}

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

    app.router_mut()
        .route("/", get(index_handler))
        .route("/health", get(health_handler));

    info!("Running database migrations...");
    app.run_migrations().await?;

    let addr = format!("0.0.0.0:{}", config.server.port);
    info!("qnsvoice listening on {}", addr);

    app.listen(&addr).await?;

    Ok(())
}
