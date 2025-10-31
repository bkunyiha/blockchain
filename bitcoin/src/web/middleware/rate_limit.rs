/// Rate limiting configuration
pub struct RateLimitConfig {
    pub requests_per_second: u32,
    pub burst_size: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 10,
            burst_size: 20,
        }
    }
}

/// Create rate limiting middleware (placeholder - not implemented yet)
pub fn create_rate_limit_layer(
    _config: RateLimitConfig,
) -> impl tower::Layer<axum::Router> + Clone {
    // For now, return a no-op layer
    tower::layer::util::Identity::new()
}

/// Create rate limiting middleware with custom key extractor (placeholder)
pub fn create_rate_limit_layer_with_key_extractor<K>(
    _config: RateLimitConfig,
    _key_extractor: K,
) -> impl tower::Layer<axum::Router> + Clone
where
    K: Send + Sync + 'static,
{
    // For now, return a no-op layer
    tower::layer::util::Identity::new()
}
