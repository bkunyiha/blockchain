<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../01-Introduction.md) - Book introduction, project structure, technical stack
2. [Chapter 1.2: Introduction to Bitcoin & Blockchain](../../bitcoin-blockchain/README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 1.3: Bitcoin Whitepaper](../../bitcoin-blockchain/00-Bitcoin-Whitepaper-Summary.md) - Bitcoin Whitepaper
4. [Chapter 1.4: Bitcoin Whitepaper In Rust](../../bitcoin-blockchain/whitepaper-rust/README.md) - Bitcoin Whitepaper In Rust
5. [Chapter 2.0: Rust Blockchain Project](../../bitcoin-blockchain/Rust-Project-Index.md) - Blockchain Project
6. [Chapter 2.1: Primitives](../../bitcoin-blockchain/primitives/README.md) - Core data structures
7. [Chapter 2.2: Utilities](../../bitcoin-blockchain/util/README.md) - Utility functions and helpers
8. [Chapter 2.3: Cryptography](../../bitcoin-blockchain/crypto/README.md) - Cryptographic primitives and libraries
9. [Chapter 2.4: Blockchain(POW & Block Acceptance)](../../bitcoin-blockchain/chain/01-Technical-Foundations.md) - Proof Of Work
10. [Chapter 2.5: Storage Layer](../../bitcoin-blockchain/store/README.md) - Persistent storage implementation
11. [Chapter 2.6: Blockchain(POW & Block Acceptance)](../../bitcoin-blockchain/chain/02-Block-Acceptance-Whitepaper-Step-5.md) - Proof Of Work
12. [Chapter 2.7: Network Layer](../../bitcoin-blockchain/net/README.md) - Peer-to-peer networking and protocol
13. [Chapter 2.8: Node Orchestration](../../bitcoin-blockchain/node/README.md) - Node context and coordination
14. [Chapter 2.9: Wallet System](../../bitcoin-blockchain/wallet/README.md) - Wallet implementation and key management
15. [Chapter 3: Web API Architecture](../../bitcoin-blockchain/web/README.md) - REST API implementation
16. [Chapter 4: Desktop Admin Interface](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md) - Iced framework architecture
17. [Chapter 5: Wallet User Interface](../../bitcoin-wallet-ui/04-Wallet-UI.md) - Wallet UI implementation
18. [Chapter 6: Embedded Database & Persistence](../../bitcoin-wallet-ui/05-Embedded-Database.md) - SQLCipher integration
19. [Chapter 7: Web Admin Interface](../../bitcoin-web-ui/06-Web-Admin-UI.md) - React/TypeScript web UI

### Part II: Deployment & Operations

20. **Chapter 8: Docker Compose Deployment** ← *You are here*
21. [Chapter 9: Kubernetes Deployment](../kubernetes/README.md) - Kubernetes production guide
22. [Chapter 10: Rust Language Guide](../../rust/README.md) - Rust programming language reference

</details>

</div>

---
<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

# Chapter 7, Section 10: Deployment Guide

**Part II: Deployment & Operations** | **Chapter 8: Docker Compose Deployment**

<div align="center">

**📚 [← Section 9: Accessing Webserver](09-Accessing-Webserver.md)** | **Section 10: Deployment Guide** | **[Section 11: Deployment Execution Walkthrough →](11-Deployment-Execution-Walkthrough.md)** 📚

</div>

---

## Deploying Script Changes

The `docker-entrypoint.sh` and `wait-for-node.sh` scripts are copied into the Docker image during build. To deploy changes to these scripts, you **MUST rebuild the Docker image** - Docker will use cached layers otherwise and your changes won't be applied.

### ⚠️ IMPORTANT: Always Rebuild

**You MUST use `--no-cache` or `--build` when deploying script changes**, otherwise Docker will use cached layers and your changes won't be applied.

### Quick Deploy (Recommended)

From the `ci/docker-compose/configs/` directory:

```bash
# Stop and remove existing containers
docker compose down

# Rebuild images with the updated scripts (--no-cache ensures fresh build)
docker compose build --no-cache

# Start containers with the new image
docker compose up -d
```

**Rate limiting note (webserver):**
- The Compose configs start a `redis` service and mount `configs/Settings.toml` into the webserver container.
- The webserver reads it via `RL_SETTINGS_PATH=/app/Settings.toml`.
- If you change `Settings.toml`, restart just the webserver to apply:
  - `docker compose restart webserver`

### Alternative: Rebuild and Restart in One Command

```bash
# Stop, rebuild (with --no-cache), and restart
docker compose build --no-cache && docker compose up -d --force-recreate
```

**Note:** The `--build` flag alone may use cached layers. For script changes, use `build --no-cache` first.

### Step-by-Step Deployment

1. **Navigate to the configs directory:**
   ```bash
   cd ci/docker-compose/configs/
   ```

2. **Stop running containers:**
   ```bash
   docker compose down
   ```

3. **Rebuild the Docker image:**
   ```bash
   docker compose build --no-cache
   ```
   Note: `--no-cache` ensures a fresh build with the latest scripts. You can omit it for faster builds if you're confident the scripts are updated.

4. **Start the containers:**
   ```bash
   docker compose up -d
   ```

5. **Verify the deployment:**
   ```bash
   # Check container logs
   docker compose logs -f webserver-1
   ```

### Verify Changes Are Deployed

Check the logs to confirm the new fixes are active:

```bash
# Watch webserver logs
docker compose logs -f webserver-1 | grep -E "ERROR|miner_"

# Check miner logs
docker compose logs -f miner-1
```

You should see:
- No more `miner_0` resolution errors
- Proper connection to `miner_1:2001` for webservers
- Clean startup without hostname resolution failures

**Note:** Debug logging is disabled by default. To enable verbose debug logs, set `DEBUG=1` in your environment:

```bash
# In docker-compose.yml, add to environment section:
environment:
  - DEBUG=1
```

## Building the React Web UI

The React web UI is automatically built during the Docker image build process. The Dockerfile uses a multi-stage build:

1. **Stage 1 (rust-builder):** Builds the Rust blockchain binary
2. **Stage 2 (web-ui-builder):** Builds the React web UI using Node.js
3. **Stage 3 (runtime):** Combines the binary and built web UI

**No manual build step is required** - the React app is built automatically when you run `docker compose build`.

### Verifying Web UI Build

To verify the web UI was built correctly:

```bash
# Check if dist directory exists in container
docker compose exec webserver-1 ls -la /app/bitcoin-web-ui/dist/

# Should show:
# - index.html
# - assets/ directory with JS and CSS files
```

### Rebuilding Web UI

If you need to rebuild the web UI (e.g., after making changes to React source code):

```bash
# Rebuild the webserver image (includes React build)
docker compose build --no-cache webserver

# Restart the webserver container
docker compose up -d webserver
```

## Troubleshooting

### Containers Fail to Start

If containers fail to start:

1. **Check logs:**
   ```bash
   docker compose logs
   ```

2. **Verify script syntax:**
   ```bash
   bash -n ci/docker-compose/configs/docker-entrypoint.sh
   bash -n ci/docker-compose/configs/wait-for-node.sh
   ```

3. **Rebuild from scratch:**
   ```bash
   docker compose down -v  # Remove volumes too (WARNING: deletes data)
   docker compose build --no-cache
   docker compose up -d
   ```

### Web UI Not Loading

If the web UI shows "React app not built" message:

1. **Verify the build completed:**
   ```bash
   docker compose exec webserver-1 test -d /app/bitcoin-web-ui/dist && echo "dist exists" || echo "dist missing"
   ```

2. **Rebuild the webserver image:**
   ```bash
   docker compose build --no-cache webserver
   docker compose up -d webserver
   ```

3. **Check build logs** for React build errors:
   ```bash
   docker compose build --no-cache webserver 2>&1 | grep -i "npm\|react\|build"
   ```

### Script Changes Not Applied

If script changes aren't being applied:

1. **Ensure you're using `--no-cache`:**
   ```bash
   docker compose build --no-cache
   ```

2. **Verify scripts are copied in Dockerfile:**
   ```bash
   # Check Dockerfile includes script copy commands
   grep -E "COPY.*docker-entrypoint|COPY.*wait-for-node" Dockerfile
   ```

3. **Force recreate containers:**
   ```bash
   docker compose up -d --force-recreate
   ```

## Production Deployment

For production environments:

1. **Tag the image:**
   ```bash
   docker compose build
   docker tag <image-id> blockchain:latest
   docker tag <image-id> blockchain:v<version>
   ```

2. **Push to registry (if using one):**
   ```bash
   docker push blockchain:latest
   ```

3. **Deploy with zero downtime:**
   ```bash
   # Use rolling updates or blue-green deployment strategy
   docker compose up -d --scale webserver=2 --no-recreate
   # Then scale down old instances
   ```

## Environment Variables

### Required Variables

- **`NODE_MINING_ADDRESS`**: Must be set for miners (or use `WALLET_ADDRESS_POOL`)

### Optional Variables

- **`DEBUG`**: Set to `1` to enable verbose debug logging
- **`SEQUENTIAL_STARTUP`**: Set to `yes` (default) or `no`
- **`BITCOIN_API_ADMIN_KEY`**: Admin API key (default: `admin-secret`)
- **`BITCOIN_API_WALLET_KEY`**: Wallet API key (default: `wallet-secret`)

### Setting Environment Variables

**Method 1: In docker-compose.yml**
```yaml
environment:
  - NODE_MINING_ADDRESS=1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa
  - DEBUG=1
```

**Method 2: Via command line**
```bash
NODE_MINING_ADDRESS="1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa" docker compose up -d
```

**Method 3: Via .env file**
```bash
# Create .env file in configs/ directory
echo "NODE_MINING_ADDRESS=1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa" > .env
docker compose up -d
```

## Best Practices

1. **Always use `--no-cache` for script changes** to ensure fresh builds
2. **Verify changes in logs** after deployment
3. **Test health endpoints** after deployment
4. **Use version tags** for production deployments
5. **Keep data volumes** separate from code deployments
6. **Monitor container logs** during and after deployment

## Summary

- Script changes require Docker image rebuild with `--no-cache`
- React web UI is built automatically during Docker build
- Always verify deployment by checking logs and health endpoints
- Use environment variables for configuration
- Follow production deployment practices for production environments

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Accessing Webserver](09-Accessing-Webserver.md) | [↑ Table of Contents](#) | [Next Section: Deployment Execution Walkthrough →](11-Deployment-Execution-Walkthrough.md) |
|:---:|:---:|:---:|
| *Section 9* | *Current Section* | *Section 11* |

</div>

---
