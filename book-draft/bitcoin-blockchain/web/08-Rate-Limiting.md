<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../00-Quick-Start.md">Chapter 1: Quick Start</a>
2. <a href="../../01-Introduction.md">Chapter 2: Introduction & Overview</a>
3. <a href="../README.md">Chapter 3: Introduction to Blockchain</a>
4. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 4: Bitcoin Whitepaper</a>
5. <a href="../whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 5: Bitcoin Whitepaper in Rust</a>
6. <a href="../Rust-Project-Index.md">Chapter 6: Rust Blockchain Project</a>
7. <a href="../primitives/README.md">Chapter 7: Primitives</a>
8. <a href="../util/README.md">Chapter 8: Utilities</a>
9. <a href="../crypto/README.md">Chapter 9: Cryptography</a>
10. <a href="../chain/01-Domain-Model.md">Chapter 10: Domain Model</a>
11. <a href="../chain/02-Blockchain-State-Management.md">Chapter 11: Blockchain State Management</a>
12. <a href="../chain/03-Chain-State-and-Storage.md">Chapter 12: Chain State and Storage</a>
13. <a href="../chain/04-UTXO-Set.md">Chapter 13: UTXO Set</a>
14. <a href="../chain/05-Transaction-Lifecycle.md">Chapter 14: Transaction Lifecycle</a>
15. <a href="../chain/06-Block-Lifecycle-and-Mining.md">Chapter 15: Block Lifecycle and Mining</a>
16. <a href="../chain/07-Consensus-and-Validation.md">Chapter 16: Consensus and Validation</a>
17. <a href="../chain/08-Node-Orchestration.md">Chapter 17: Node Orchestration</a>
18. <a href="../chain/09-Transaction-To-Block.md">Chapter 18: Transaction to Block</a>
19. <a href="../chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 19: Block Acceptance</a>
20. <a href="../store/README.md">Chapter 20: Storage Layer</a>
21. <a href="../net/README.md">Chapter 21: Network Layer</a>
22. <a href="../node/README.md">Chapter 22: Node Orchestration</a>
23. <a href="../wallet/README.md">Chapter 23: Wallet System</a>
24. <a href="README.md">Chapter 24: Web API Architecture</a>
25. <a href="../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 25: Desktop Admin (Iced)</a>
26. <a href="../../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">25A: Code Walkthrough</a>
27. <a href="../../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">25B: Update Loop</a>
28. <a href="../../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">25C: View Layer</a>
29. <a href="../../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 26: Desktop Admin (Tauri)</a>
30. <a href="../../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">26A: Rust Backend</a>
31. <a href="../../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">26B: Frontend Infrastructure</a>
32. <a href="../../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">26C: Frontend Pages</a>
33. <a href="../../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 27: Wallet UI (Iced)</a>
34. <a href="../../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">27A: Code Listings</a>
35. <a href="../../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 28: Wallet UI (Tauri)</a>
36. <a href="../../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">28A: Rust Backend</a>
37. <a href="../../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">28B: Frontend Infrastructure</a>
38. <a href="../../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">28C: Frontend Pages</a>
39. <a href="../../embedded-database/06-Embedded-Database.md">Chapter 29: Embedded Database</a>
40. <a href="../../embedded-database/06A-Embedded-Database-Code-Listings.md">29A: Code Listings</a>
41. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 30: Web Admin Interface</a>
42. <a href="../../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">30A: Code Listings</a>
### Part II: Deployment & Operations

43. <a href="../../ci/docker-compose/01-Introduction.md">Chapter 31: Docker Compose Deployment</a>
44. <a href="../../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">31A: Code Listings</a>
45. <a href="../../ci/kubernetes/README.md">Chapter 32: Kubernetes Deployment</a>
46. <a href="../../ci/kubernetes/01A-Kubernetes-Code-Listings.md">32A: Code Listings</a>
### Part III: Language Reference

47. <a href="../../rust/README.md">Chapter 33: Rust Language Guide</a>
### Appendices

48. <a href="../../Glossary.md">Glossary</a>
49. <a href="../../Bibliography.md">Bibliography</a>
50. <a href="../../Appendix-Source-Reference.md">Source Reference</a>

</details>

</div>

---
# Chapter 24.8: Rate Limiting

**Part I: Foundations & Core Implementation** | **Chapter 24.8: Rate Limiting**

<div align="center">

**[← Error Handling](07-Error-Handling.md)** | **[Rate Limiting](08-Rate-Limiting.md)** | **[OpenAPI →](09-OpenAPI.md)**

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

# Per-IP limit: allow bursts of 20, refill 1 token every 6 seconds
[[rate_limiter.limiter]]
strategy = "ip"
global_bucket = { tokens_count = 20, add_tokens_every = 6 }

# Tighten a specific expensive endpoint by URL
[[rate_limiter.limiter]]
strategy = "url"
global_bucket = { tokens_count = 60, add_tokens_every = 1 }
buckets_per_value = [
  { value = "/api/mining/generate", tokens_count = 2,
    add_tokens_every = 60 },
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

**[← Error Handling](07-Error-Handling.md)** | **[Rate Limiting](08-Rate-Limiting.md)** | **[OpenAPI →](09-OpenAPI.md)**

</div>


