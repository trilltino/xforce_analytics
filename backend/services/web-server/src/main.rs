use axum::Router;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use lib_core::{create_pool, AppConfig, DbConfig};
use lib_web::{cors_dev, cors_production, logger_middleware};
use web_server::{create_routes, AppState};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "web_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Load configuration
    let app_config = match AppConfig::from_env() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load app configuration: {}", e);
            std::process::exit(1);
        }
    };

    let db_config = match DbConfig::from_env() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load database configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Create database pool
    let pool = match create_pool(&db_config).await {
        Ok(pool) => {
            info!(" Database connection established");
            pool
        }
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            error!("Make sure PostgreSQL is running and DATABASE_URL is set correctly");
            std::process::exit(1);
        }
    };

    // Load project data
    let projects = match load_projects_from_json().await {
        Ok(projects) => {
            info!(" Loaded {} projects from JSON", projects.len());
            projects
        }
        Err(e) => {
            error!("Failed to load projects: {}", e);
            vec![] // Continue with empty project list
        }
    };

    // Create app state
    let state = AppState::new(pool.clone(), Arc::new(projects));

    // Create routes
    let routes = create_routes().with_state(state);

    // Apply middleware
    let cors_layer = if app_config.environment.is_production() {
        cors_production(vec![app_config.frontend_url.clone()])
    } else {
        cors_dev()
    };

    let app = Router::new()
        .merge(routes)
        .layer(tower_cookies::CookieManagerLayer::new())
        .layer(cors_layer)
        .layer(axum::middleware::from_fn(logger_middleware));

    // Start server
    let addr = app_config.addr();
    info!("🚀 Server running on http://{}", addr);
    info!("   Environment: {:?}", app_config.environment);
    info!("   Frontend URL: {}", app_config.frontend_url);

    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(e) => {
            error!("Failed to bind to address {}: {}", addr, e);
            std::process::exit(1);
        }
    };

    if let Err(e) = axum::serve(listener, app).await {
        error!("Server failed to start: {}", e);
        std::process::exit(1);
    }
}

/// Load projects from JSON file
async fn load_projects_from_json() -> Result<Vec<shared::Project>, Box<dyn std::error::Error>> {
    // Try workspace root first (for cargo run from workspace root)
    let workspace_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("data/raw/all_projects_structured.json");

    info!("Trying workspace path: {:?}", workspace_path);

    if workspace_path.exists() {
        let contents = tokio::fs::read_to_string(&workspace_path).await?;
        let data: serde_json::Value = serde_json::from_str(&contents)?;

        // Extract projects from nested structure
        let mut all_projects = Vec::new();
        if let Some(by_category) = data.get("by_category").and_then(|v| v.as_object()) {
            for (_category, projects) in by_category {
                if let Some(project_array) = projects.as_array() {
                    for project in project_array {
                        if let Ok(p) = serde_json::from_value::<shared::Project>(project.clone()) {
                            all_projects.push(p);
                        }
                    }
                }
            }
        }
        return Ok(all_projects);
    }

    // Try current directory (for cargo run from backend directory)
    let cwd = std::env::current_dir()?;
    let cwd_path = cwd.join("data/raw/all_projects_structured.json");

    if cwd_path.exists() {
        let contents = tokio::fs::read_to_string(&cwd_path).await?;
        let data: serde_json::Value = serde_json::from_str(&contents)?;

        let mut all_projects = Vec::new();
        if let Some(by_category) = data.get("by_category").and_then(|v| v.as_object()) {
            for (_category, projects) in by_category {
                if let Some(project_array) = projects.as_array() {
                    for project in project_array {
                        if let Ok(p) = serde_json::from_value::<shared::Project>(project.clone()) {
                            all_projects.push(p);
                        }
                    }
                }
            }
        }
        return Ok(all_projects);
    }

    // Try parent of current directory
    let parent_path = cwd.parent()
        .ok_or("No parent directory")?
        .join("data/raw/all_projects_structured.json");

    let contents = tokio::fs::read_to_string(&parent_path).await?;
    let data: serde_json::Value = serde_json::from_str(&contents)?;

    let mut all_projects = Vec::new();
    if let Some(by_category) = data.get("by_category").and_then(|v| v.as_object()) {
        for (_category, projects) in by_category {
            if let Some(project_array) = projects.as_array() {
                for project in project_array {
                    if let Ok(p) = serde_json::from_value::<shared::Project>(project.clone()) {
                        all_projects.push(p);
                    }
                }
            }
        }
    }
    Ok(all_projects)
}
