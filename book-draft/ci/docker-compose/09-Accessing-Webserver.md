<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../../README.md)
2. [Chapter 2: Transaction ID Format](../../bitcoin-blockchain/primitives/02-Transaction-ID-Format.md)
3. [Chapter 4: Desktop Admin Interface](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)
4. [Chapter 5: Wallet User Interface](../../bitcoin-wallet-ui/04-Wallet-UI.md)
5. [Chapter 6: Embedded Database & Persistence](../../bitcoin-wallet-ui/05-Embedded-Database.md)
6. [Chapter 7: Web Admin Interface](../../bitcoin-web-ui/06-Web-Admin-UI.md)

### Part II: Deployment & Operations

8. [Chapter 8: Docker Compose Deployment](01-Introduction.md) - Main chapter
   - [Section 2: Architecture](02-Architecture.md)
   - [Section 3: Execution Flow](03-Execution-Flow.md)
   - [Section 4: Network Configuration](04-Network-Configuration.md)
   - [Section 5: Sequential Startup](07-Sequential-Startup.md)
   - [Section 6: Port Mapping](05-Port-Mapping.md)
   - [Section 7: Scaling](06-Scaling.md)
   - [Section 8: Deployment Scenarios](08-Deployment-Scenarios.md)
   - [Section 9: Accessing Webserver](09-Accessing-Webserver.md) ← *You are here*
   - [Section 10: Deployment Guide](10-Deployment-Guide.md)
   - [Section 11: Deployment Execution Walkthrough](11-Deployment-Execution-Walkthrough.md)
   - [Section 12: DNS Resolution Mechanism](12-DNS-Resolution-Mechanism.md)
9. [Chapter 9: Kubernetes Deployment](../../kubernetes/README.md)

</details>

</div>

<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

# Chapter 7, Section 9: Accessing Webserver

**Part II: Deployment & Operations** | **Chapter 8: Docker Compose Deployment**

<div align="center">

**📚 [← Chapter 6: Web Admin UI](../../bitcoin-web-ui/06-Web-Admin-UI.md)** | **Chapter 8: Docker Compose** | **[Chapter 9: Kubernetes →](../../kubernetes/README.md)** 📚

</div>

---

## Quick Access

Once Docker containers are running, access the webserver at:

**Base URL:** `http://localhost:8080`

## 1. Health Check (No Authentication Required)

Check if the webserver is running:

```bash
# Basic health check
curl http://localhost:8080/health

# Liveness probe
curl http://localhost:8080/health/live

# Readiness probe
curl http://localhost:8080/health/ready
```

Or open in your browser:
- http://localhost:8080/health
- http://localhost:8080/health/live
- http://localhost:8080/health/ready

## 2. Web UI (React Application)

The React web UI is automatically built and included in the Docker image. Access it at:

**URL:** http://localhost:8080/

The web UI provides a complete interface for:
- Blockchain exploration
- Wallet management
- Transaction monitoring
- Mining operations
- Health monitoring

**Note:** The React web UI is built during Docker image creation. No manual build step is required.

## 3. Swagger UI (Interactive API Documentation)

Access the interactive API documentation:

**URL:** http://localhost:8080/swagger-ui/

This provides a web interface to explore and test all API endpoints.

## 4. API Endpoints (Require Authentication)

All API endpoints require authentication via the `X-API-Key` header.

### Authentication Keys

Default keys (from docker-compose.yml):
- **Admin API:** `admin-secret` (for `/api/admin/*` endpoints)
- **Wallet API:** `wallet-secret` (for `/api/wallet/*` endpoints)

### Example API Calls

#### Health Check (Admin API)
```bash
curl -H "X-API-Key: admin-secret" http://localhost:8080/api/admin/health
```

#### Get Blockchain Info
```bash
curl -H "X-API-Key: admin-secret" http://localhost:8080/api/admin/blockchain
```

#### Get Latest Blocks
```bash
curl -H "X-API-Key: admin-secret" http://localhost:8080/api/admin/blockchain/blocks/latest
```

#### Get All Blocks
```bash
curl -H "X-API-Key: admin-secret" http://localhost:8080/api/admin/blockchain/blocks
```

#### Create Wallet
```bash
curl -X POST \
  -H "X-API-Key: wallet-secret" \
  -H "Content-Type: application/json" \
  http://localhost:8080/api/wallet/wallet
```

#### Get Mining Info
```bash
curl -H "X-API-Key: admin-secret" http://localhost:8080/api/admin/mining/info
```

## 5. Available API Endpoints

### Public Endpoints (No Auth)
- `GET /health` - Health check
- `GET /health/live` - Liveness probe
- `GET /health/ready` - Readiness probe
- `GET /swagger-ui/` - Swagger UI documentation
- `GET /` - React web UI

### Admin API (`/api/admin/*`) - Requires `X-API-Key: admin-secret`

#### Blockchain
- `GET /api/admin/blockchain` - Get blockchain info
- `GET /api/admin/blockchain/blocks` - Get all blocks
- `GET /api/admin/blockchain/blocks/latest` - Get latest blocks
- `GET /api/admin/blockchain/blocks/{hash}` - Get block by hash

#### Wallet
- `POST /api/admin/wallet` - Create wallet
- `GET /api/admin/wallet/addresses` - Get all addresses
- `GET /api/admin/wallet/{address}` - Get wallet info
- `GET /api/admin/wallet/{address}/balance` - Get balance

#### Transactions
- `GET /api/admin/transactions` - Get all transactions
- `GET /api/admin/transactions/mempool` - Get mempool
- `GET /api/admin/transactions/mempool/{txid}` - Get mempool transaction
- `GET /api/admin/transactions/address/{address}` - Get address transactions
- `POST /api/admin/transactions` - Send transaction

#### Mining
- `GET /api/admin/mining/info` - Get mining info
- `POST /api/admin/mining/generatetoaddress` - Generate blocks

#### Health (Admin)
- `GET /api/admin/health` - Health check
- `GET /api/admin/health/live` - Liveness probe
- `GET /api/admin/health/ready` - Readiness probe

### Wallet API (`/api/wallet/*`) - Requires `X-API-Key: wallet-secret`
- `POST /api/wallet/wallet` - Create wallet
- `POST /api/wallet/transactions` - Send transaction

### Public API (`/api/v1/*`) - No authentication required
- `GET /api/v1/blockchain` - Get blockchain info
- `GET /api/v1/blockchain/blocks` - Get all blocks
- `GET /api/v1/blockchain/blocks/latest` - Get latest blocks
- `GET /api/v1/blockchain/blocks/{hash}` - Get block by hash
- `POST /api/v1/wallet` - Create wallet
- `GET /api/v1/wallet/addresses` - Get all addresses
- `GET /api/v1/wallet/{address}` - Get wallet info
- `GET /api/v1/wallet/{address}/balance` - Get balance
- `GET /api/v1/transactions` - Get all transactions
- `GET /api/v1/transactions/mempool` - Get mempool
- `GET /api/v1/transactions/address/{address}` - Get address transactions
- `POST /api/v1/transactions` - Send transaction
- `GET /api/v1/mining/info` - Get mining info
- `POST /api/v1/mining/generatetoaddress` - Generate blocks

## 6. Using the Web UI

The React web UI is automatically served by the blockchain server. Simply navigate to:

**URL:** http://localhost:8080/

The web UI will automatically connect to the API at the same base URL. If you need to configure API keys, use the "Configure API" option in the navbar.

**Default API Keys:**
- **Admin API Key:** `admin-secret`
- **Wallet API Key:** `wallet-secret`

## 7. Checking Container Status

```bash
# Check if webserver is running
docker compose ps webserver

# View webserver logs
docker compose logs -f webserver-1

# Check webserver health
docker compose exec webserver-1 curl http://localhost:8080/health
```

## 8. Troubleshooting

### Port Already in Use
If port 8080 is already in use, you can change it in `docker-compose.yml`:
```yaml
ports:
  - "8081:8080"  # Map host port 8081 to container port 8080
```

Then access at: `http://localhost:8081`

### Container Not Starting
```bash
# Check logs
docker compose logs webserver-1

# Check if miner is ready (webserver depends on miner)
docker compose logs miner-1

# Restart webserver
docker compose restart webserver-1
```

### API Authentication Failing
Make sure you're using the correct API key:
- Check `docker-compose.yml` for `BITCOIN_API_ADMIN_KEY` and `BITCOIN_API_WALLET_KEY`
- Default values are `admin-secret` and `wallet-secret`

### Testing from Inside Container
```bash
# Execute command inside webserver container
docker compose exec webserver-1 curl http://localhost:8080/health

# Access shell inside container
docker compose exec webserver-1 /bin/bash
```

### Web UI Not Loading
If the web UI shows a "React app not built" message:

1. **Verify the Docker image was built correctly:**
   ```bash
   docker compose exec webserver-1 ls -la /app/bitcoin-web-ui/dist/
   ```

2. **Rebuild the Docker image:**
   ```bash
   docker compose build --no-cache webserver
   docker compose up -d webserver
   ```

3. **Check the build logs** to ensure the React app was built during the Docker build process.

## 9. Quick Reference

| Service | URL | Auth Required |
|---------|-----|---------------|
| Web UI | http://localhost:8080/ | No |
| Health Check | http://localhost:8080/health | No |
| Swagger UI | http://localhost:8080/swagger-ui/ | No |
| Admin API | http://localhost:8080/api/admin/* | Yes (`admin-secret`) |
| Wallet API | http://localhost:8080/api/wallet/* | Yes (`wallet-secret`) |
| Public API | http://localhost:8080/api/v1/* | No |

## 10. Example: Complete Workflow

```bash
# 1. Check health
curl http://localhost:8080/health

# 2. Access web UI in browser
open http://localhost:8080/

# 3. Create a wallet via API
curl -X POST \
  -H "X-API-Key: admin-secret" \
  -H "Content-Type: application/json" \
  http://localhost:8080/api/admin/wallet

# 4. Get blockchain info
curl -H "X-API-Key: admin-secret" \
  http://localhost:8080/api/admin/blockchain

# 5. Get mining info
curl -H "X-API-Key: admin-secret" \
  http://localhost:8080/api/admin/mining/info
```

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Deployment Scenarios](08-Deployment-Scenarios.md) | [↑ Table of Contents](#) | [Next Section: Deployment Guide →](10-Deployment-Guide.md) |
|:---:|:---:|:---:|
| *Section 8* | *Current Section* | *Section 10* |

</div>

---
