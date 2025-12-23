# Rate Limiting

**Part I: Core Blockchain Implementation** | **Chapter 3.8: Rate Limiting**

<div align="center">

**📚 [← Error Handling](07-Error-Handling.md)** | **Rate Limiting** | **[OpenAPI →](09-OpenAPI.md)** 📚

</div>

---

## Table of Contents

1. [Why Rate Limiting Matters](#why-rate-limiting-matters)
2. [What We Implemented (High Level)](#what-we-implemented-high-level)
3. [How the Rate Limiter Works](#how-the-rate-limiter-works)
4. [Settings File (TOML) Schema](#settings-file-toml-schema)
5. [Enabling Rate Limiting](#enabling-rate-limiting)
6. [Operating Notes](#operating-notes)
7. [Quick Manual Test](#quick-manual-test)

---

## Why Rate Limiting Matters

Blockchains are resource-heavy systems: even “simple” endpoints might touch storage, validate objects, or query UTXO state. A web API without protections is easy to overload—intentionally (DoS) or accidentally (misconfigured clients, dashboards with aggressive polling).

Rate limiting gives us a **predictable upper bound** on request volume per client and per endpoint. That helps:

- **Protect availability**: keep the node responsive under load.
- **Prevent abuse**: brute-force or spammy clients get throttled.
- **Keep latency stable**: fewer thundering herds, fewer spikes.

---

## What We Implemented (High Level)

We implemented rate limiting as **Axum middleware** using the [`axum_rate_limiter`](https://crates.io/crates/axum_rate_limiter) crate. The middleware:

- Uses a **Redis-backed token bucket** to track limits (so it works across multiple server instances).
- Supports multiple strategies (IP, URL, header, query, body).
- Allows an **IP whitelist** (bypass rate limiting for trusted sources).
- Adds response headers:
  - `X-RateLimit-Limit`
  - `X-RateLimit-Remaining`
- Rejects requests with **HTTP 429** when a limit is exceeded.

In this repository, integration happens in:

- `bitcoin/src/web/middleware/rate_limit.rs` (initialization / settings loading)
- `bitcoin/src/web/server.rs` (middleware wiring + enabling `ConnectInfo`)

**Why `ConnectInfo` matters:** `axum_rate_limiter` determines the client IP via Axum’s
`ConnectInfo<SocketAddr>` extractor. That extractor only works if the server is started with
`into_make_service_with_connect_info::<SocketAddr>()` (otherwise the client address is not attached
to the request extensions, and the middleware can’t read it).

---

## How the Rate Limiter Works

### Token bucket (the mental model)

Think of every “bucket” as a small wallet of tokens:

- **Capacity** (`tokens_count`): maximum tokens the bucket can hold (burst allowance).
- **Refill interval** (`add_tokens_every` seconds): how often a token is added back.
- Every request **spends 1 token**.
- If the bucket has **0 tokens**, the request is rejected (429).

This gives you the best of both worlds:

- A client can burst briefly (up to `tokens_count`).
- Over time, the client is held to a stable average rate.

### Request flow in middleware

For each incoming request:

1. **Extract client IP** from `ConnectInfo<SocketAddr>`.
2. **Whitelist check**: if the IP is in `ip_whitelist`, the request is immediately allowed.
3. **Build a safe request snapshot**:
   - the middleware reads the whole body into bytes and reconstructs the request
   - this allows request inspection (for “body strategy”) without holding a non-`Send` request across awaits
4. **Evaluate configured strategies**:
   - “user” strategies: `ip`, `header`
   - “request” strategies: `url`, `query`, `body`
5. If any strategy reports the limit exceeded, return **429 Too Many Requests**.
6. Otherwise, forward to the handler and add `X-RateLimit-*` headers based on the most restrictive matched limit.

### Important detail: Redis failures

If Redis cannot be reached, the limiter’s check returns “no decision” for that limiter. The middleware treats that as “allow” (it continues checking others, and if none produce a limit it proceeds). This is a deliberate trade-off: **fail-open** keeps the API available, but reduces protection during Redis outages.

---

## Settings File (TOML) Schema

The limiter reads configuration from a TOML file. The path is controlled by the environment variable `RL_SETTINGS_PATH`. If it is not set, the default is `./Settings.toml` (relative to the process working directory).

### Top-level settings

```toml
[rate_limiter]
redis_addr = "127.0.0.1:6379"
ip_whitelist = ["127.0.0.1"]
```

- `redis_addr`: Redis host/port **without** the `redis://` scheme (the crate builds the URL internally).
- `ip_whitelist`: list of IPs that bypass rate limiting.

### Strategy blocks

Each limiter is configured as:

```toml
[[rate_limiter.limiter]]
strategy = "ip" # ip | url | header | query | body
global_bucket = { tokens_count = 10, add_tokens_every = 60 }
```

Some strategies can also use per-value buckets:

```toml
[[rate_limiter.limiter]]
strategy = "url"
global_bucket = { tokens_count = 50, add_tokens_every = 60 }
buckets_per_value = [
  { value = "/api/mining/generate", tokens_count = 2, add_tokens_every = 60 },
  { value = "/api/health", tokens_count = 60, add_tokens_every = 10 },
]
```

### Full example `Settings.toml`

```toml
[rate_limiter]
redis_addr = "127.0.0.1:6379"
ip_whitelist = ["127.0.0.1"]

# Per-IP limit: allow bursts up to 20, refill 1 token every 6 seconds (~10/min average)
[[rate_limiter.limiter]]
strategy = "ip"
global_bucket = { tokens_count = 20, add_tokens_every = 6 }

# Tighten a specific expensive endpoint by URL
[[rate_limiter.limiter]]
strategy = "url"
global_bucket = { tokens_count = 60, add_tokens_every = 1 } # default for other URLs
buckets_per_value = [
  { value = "/api/mining/generate-to-address", tokens_count = 2, add_tokens_every = 60 },
]
```

---

## Enabling Rate Limiting

### 1) Run Redis

For local development:

```bash
docker run -d --name redis -p 6379:6379 redis:7-alpine
```

### 2) Create a settings file

Create `Settings.toml` somewhere on disk (example above).

### 3) Point the server to it

Set `RL_SETTINGS_PATH` before running the node:

```bash
export RL_SETTINGS_PATH=/absolute/path/to/Settings.toml
```

### 4) Ensure it’s enabled in server config

Rate limiting is controlled by `WebServerConfig.enable_rate_limiting` (default: `true`). The server wires the limiter in only when enabled.

---

## Operating Notes

- **Client IP correctness**: the middleware uses the socket address (`ConnectInfo`). If you deploy behind a reverse proxy, you’ll likely want to terminate TLS/proxy at a layer that preserves the real client IP (or extend the implementation to honor `X-Forwarded-For` safely).
- **Headers provided**: the current middleware adds `X-RateLimit-Limit` and `X-RateLimit-Remaining`. It does not currently add `Retry-After`.
- **Multi-strategy behavior**: if you configure multiple limiters, the most restrictive matched limit wins, and any exceeded limit returns 429.

---

## Quick Manual Test

Assuming you configured a very small bucket (e.g. `tokens_count = 3` and `add_tokens_every = 60`) for a health endpoint:

```bash
for i in {1..10}; do
  curl -s -o /dev/null -w "%{http_code} " http://localhost:8080/api/health
done
echo
```

You should see a few `200` responses followed by `429`. When successful, responses should include:

- `X-RateLimit-Limit`
- `X-RateLimit-Remaining`

---

<div align="center">

**📚 [← Error Handling](07-Error-Handling.md)** | **Rate Limiting** | **[OpenAPI →](09-OpenAPI.md)** 📚

</div>


