<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../00-Quick-Start.md">Chapter 1: Quick Start</a>
2. <a href="../../01-Introduction.md">Chapter 2: Introduction & Overview</a>
3. <a href="../../bitcoin-blockchain/README.md">Chapter 3: Introduction to Blockchain</a>
4. <a href="../../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 4: Bitcoin Whitepaper</a>
5. <a href="../../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 5: Bitcoin Whitepaper in Rust</a>
6. <a href="../../bitcoin-blockchain/Rust-Project-Index.md">Chapter 6: Rust Blockchain Project</a>
7. <a href="../../bitcoin-blockchain/primitives/README.md">Chapter 7: Primitives</a>
8. <a href="../../bitcoin-blockchain/util/README.md">Chapter 8: Utilities</a>
9. <a href="../../bitcoin-blockchain/crypto/README.md">Chapter 9: Cryptography</a>
10. <a href="../../bitcoin-blockchain/chain/01-Domain-Model.md">Chapter 10: Domain Model</a>
11. <a href="../../bitcoin-blockchain/chain/02-Blockchain-State-Management.md">Chapter 11: Blockchain State Management</a>
12. <a href="../../bitcoin-blockchain/chain/03-Chain-State-and-Storage.md">Chapter 12: Chain State and Storage</a>
13. <a href="../../bitcoin-blockchain/chain/04-UTXO-Set.md">Chapter 13: UTXO Set</a>
14. <a href="../../bitcoin-blockchain/chain/05-Transaction-Lifecycle.md">Chapter 14: Transaction Lifecycle</a>
15. <a href="../../bitcoin-blockchain/chain/06-Block-Lifecycle-and-Mining.md">Chapter 15: Block Lifecycle and Mining</a>
16. <a href="../../bitcoin-blockchain/chain/07-Consensus-and-Validation.md">Chapter 16: Consensus and Validation</a>
17. <a href="../../bitcoin-blockchain/chain/08-Node-Orchestration.md">Chapter 17: Node Orchestration</a>
18. <a href="../../bitcoin-blockchain/chain/09-Transaction-To-Block.md">Chapter 18: Transaction to Block</a>
19. <a href="../../bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 19: Block Acceptance</a>
20. <a href="../../bitcoin-blockchain/store/README.md">Chapter 20: Storage Layer</a>
21. <a href="../../bitcoin-blockchain/net/README.md">Chapter 21: Network Layer</a>
22. <a href="../../bitcoin-blockchain/node/README.md">Chapter 22: Node Orchestration</a>
23. <a href="../../bitcoin-blockchain/wallet/README.md">Chapter 23: Wallet System</a>
24. <a href="../../bitcoin-blockchain/web/README.md">Chapter 24: Web API Architecture</a>
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

43. <a href="01-Introduction.md">Chapter 31: Docker Compose Deployment</a>
44. <a href="01A-Docker-Compose-Code-Listings.md">31A: Code Listings</a>
45. <a href="../kubernetes/README.md">Chapter 32: Kubernetes Deployment</a>
46. <a href="../kubernetes/01A-Kubernetes-Code-Listings.md">32A: Code Listings</a>
### Part III: Language Reference

47. <a href="../../rust/README.md">Chapter 33: Rust Language Guide</a>
### Appendices

48. <a href="../../Glossary.md">Glossary</a>
49. <a href="../../Bibliography.md">Bibliography</a>
50. <a href="../../Appendix-Source-Reference.md">Source Reference</a>

</details>

</div>

---
<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

## Chapter 22, Section 9: Accessing Webserver

**Part II: Deployment & Operations** | **Chapter 31: Docker Compose Deployment**

<div align="center">

**[← Chapter 30: Web Admin UI](../../bitcoin-web-ui/06-Web-Admin-UI.md)** | **Chapter 31: Docker Compose** | **[Chapter 32: Kubernetes →](../kubernetes/README.md)**

</div>

---

## Quick Access

Once Docker containers are running, access the webserver at:

**Base URL:** `http://localhost:8080`

> **Methods involved:**
> - Webserver service definition + health check endpoints: `docker-compose.yml` ([Listing 22A.1](01A-Docker-Compose-Code-Listings.md#listing-22a1-cidocker-composeconfigsdocker-composeyml))
> - Rate limiting settings: `Settings.toml` ([Listing 22A.10](01A-Docker-Compose-Code-Listings.md#listing-22a10-cidocker-composeconfigssettingstoml))

## 1. Health Check (No Authentication Required)

Check if the webserver is running:

```bash
# Liveness probe
curl -f http://localhost:8080/api/health/live

# Readiness probe
curl -f http://localhost:8080/api/health/ready
```

Or open in your browser:
- http://localhost:8080/api/health/live
- http://localhost:8080/api/health/ready

## 2. Web Admin Interface (separate application)

This Docker Compose deployment starts the **node webserver** (REST API + Swagger UI) and miners. The **Web Admin Interface** is a separate application (`bitcoin-web-ui`) described in Chapter 21:

- [Chapter 30: Web Admin Interface](../../bitcoin-web-ui/06-Web-Admin-UI.md)

In other words:

- use this section to reach the **API** and operational endpoints served by the node webserver,
- use Chapter 21 to run and operate the dedicated React admin UI.

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
curl -H "X-API-Key: admin-secret" \
  http://localhost:8080/api/admin/blockchain/blocks/latest
```

#### Get All Blocks
```bash
curl -H "X-API-Key: admin-secret" \
  http://localhost:8080/api/admin/blockchain/blocks
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
- `GET /api/health/live` - Liveness probe
- `GET /api/health/ready` - Readiness probe
- `GET /swagger-ui/` - Swagger UI documentation

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

## 6. Swagger UI and API Clients

For interactive exploration of the API surface, use Swagger UI:

- `http://localhost:8080/swagger-ui/`

For scripts and automation, prefer `curl` (examples above) and supply `X-API-Key` for authenticated routes.

## 7. Checking Container Status

```bash
# Check if webserver is running
docker compose ps webserver

# View webserver logs
docker compose logs -f webserver-1

# Check webserver health
docker compose exec webserver-1 curl -f http://localhost:8080/api/health/ready
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

### Rate Limiting Verification (Optional)

The webserver uses Redis-backed rate limiting. You can verify headers are present on successful responses:

```bash
curl -i http://localhost:8080/api/health/liveness | grep -i "x-ratelimit"
```

### API Authentication Failing
Make sure you are using the correct API key:
- Check `docker-compose.yml` for `BITCOIN_API_ADMIN_KEY` and `BITCOIN_API_WALLET_KEY`
- Default values are `admin-secret` and `wallet-secret`

### Testing from Inside Container
```bash
# Execute command inside webserver container
docker compose exec webserver-1 curl -f http://localhost:8080/api/health/ready

# Access shell inside container
docker compose exec webserver-1 /bin/bash
```

## 9. Quick Reference

| Service | URL | Auth Required |
|---------|-----|---------------|
| Liveness | http://localhost:8080/api/health/live | No |
| Readiness | http://localhost:8080/api/health/ready | No |
| Swagger UI | http://localhost:8080/swagger-ui/ | No |
| Admin API | http://localhost:8080/api/admin/* | Yes (`admin-secret`) |
| Wallet API | http://localhost:8080/api/wallet/* | Yes (`wallet-secret`) |
| Public API | http://localhost:8080/api/v1/* | No |

## 10. Example: Complete Workflow

```bash
# 1. Check readiness
curl -f http://localhost:8080/api/health/ready

# 2. Open Swagger UI (interactive docs)
open http://localhost:8080/swagger-ui/

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

| [← Previous Section: Deployment Scenarios & Operations](04-Deployment-Scenarios-and-Operations.md) | [↑ Table of Contents](#) | [Next Section: DNS Resolution Mechanism →](06-DNS-Resolution-Mechanism.md) |
|:---:|:---:|:---:|
| *Section 8* | *Current Section* | *Section 10* |

</div>

---
