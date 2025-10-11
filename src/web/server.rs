use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::compression::CompressionLayer;

use crate::node::NodeContext;
use crate::web::middleware::cors;
use crate::web::models::{ApiResponse, ErrorResponse};
use crate::web::routes::{create_all_api_routes, create_web_routes};

/// Web server configuration
#[derive(Debug, Clone)]
pub struct WebServerConfig {
    pub host: String,
    pub port: u16,
    pub enable_cors: bool,
    pub enable_rate_limiting: bool,
    pub rate_limit_requests_per_second: u32,
    pub rate_limit_burst_size: u32,
}

impl Default for WebServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            enable_cors: true,
            enable_rate_limiting: true,
            rate_limit_requests_per_second: 10,
            rate_limit_burst_size: 20,
        }
    }
}

/// Web server implementation
pub struct WebServer {
    config: WebServerConfig,
    node: Arc<NodeContext>,
}

impl WebServer {
    /// Create a new web server instance with NodeContext
    pub fn new(node_context: NodeContext, config: WebServerConfig) -> Self {
        Self {
            config,
            node: Arc::new(node_context),
        }
    }

    /// Create web server from NodeContext directly
    pub fn from_node_context(node: NodeContext, config: WebServerConfig) -> Self {
        Self {
            config,
            node: Arc::new(node),
        }
    }

    /// Create the main application router
    pub fn create_app(&self) -> Router {
        let app = Router::new()
            .merge(create_all_api_routes())
            .merge(create_web_routes())
            .with_state(self.node.clone());

        // Add basic middleware
        let mut app = app;

        // Add CORS middleware
        if self.config.enable_cors {
            app = app.layer(cors::create_cors_layer());
        }

        // Add compression middleware
        app = app.layer(CompressionLayer::new());

        // Add error handling middleware
        app = app.layer(axum::middleware::from_fn(handle_errors));

        app
    }

    /// Start the web server with graceful shutdown
    pub async fn start_with_shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        let app = self.create_app();

        let addr = SocketAddr::from(([0, 0, 0, 0], self.config.port));

        tracing::info!("Starting web server on {} with graceful shutdown", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;

        // Handle shutdown signal
        let shutdown_signal = async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install CTRL+C signal handler");
            tracing::info!("Shutdown signal received");
        };

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal)
            .await?;

        Ok(())
    }
}

/// Error handling middleware
async fn handle_errors(
    request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let response = next.run(request).await;

    if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
        let error_response = ErrorResponse {
            error: "Internal Server Error".to_string(),
            message: "An unexpected error occurred".to_string(),
            status_code: 500,
            timestamp: chrono::Utc::now(),
        };

        return Ok(Json(ApiResponse::<()>::error(
            serde_json::to_string(&error_response).unwrap_or_else(|_| "Unknown error".to_string()),
        ))
        .into_response());
    }

    Ok(response)
}

/// Create a web server with default configuration
pub fn create_web_server(node_context: NodeContext) -> WebServer {
    WebServer::new(node_context, WebServerConfig::default())
}

/// Create a web server with custom configuration
pub fn create_web_server_with_config(
    node_context: NodeContext,
    config: WebServerConfig,
) -> WebServer {
    WebServer::new(node_context, config)
}
