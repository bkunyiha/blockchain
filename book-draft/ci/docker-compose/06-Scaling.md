<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a> - Book introduction, project structure, technical stack
2. <a href="../../bitcoin-blockchain/README.md">Chapter 1.2: Introduction to Bitcoin & Blockchain</a> - Bitcoin and blockchain fundamentals
3. <a href="../../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 1.3: Bitcoin Whitepaper</a> - Bitcoin Whitepaper
4. <a href="../../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 1.4: Bitcoin Whitepaper In Rust</a> - Bitcoin Whitepaper In Rust
5. <a href="../../bitcoin-blockchain/Rust-Project-Index.md">Chapter 2.0: Rust Blockchain Project</a> - Blockchain Project
6. <a href="../../bitcoin-blockchain/primitives/README.md">Chapter 2.1: Primitives</a> - Core data structures
7. <a href="../../bitcoin-blockchain/util/README.md">Chapter 2.2: Utilities</a> - Utility functions and helpers
8. <a href="../../bitcoin-blockchain/crypto/README.md">Chapter 2.3: Cryptography</a> - Cryptographic primitives and libraries
9. <a href="../../bitcoin-blockchain/chain/README.md">Chapter 2.4: Blockchain (Technical Foundations)</a> - Proof Of Work
10. <a href="../../bitcoin-blockchain/store/README.md">Chapter 2.5: Storage Layer</a> - Persistent storage implementation
11. <a href="../../bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. <a href="../../bitcoin-blockchain/net/README.md">Chapter 2.7: Network Layer</a> - Peer-to-peer networking and protocol
13. <a href="../../bitcoin-blockchain/node/README.md">Chapter 2.8: Node Orchestration</a> - Node context and coordination
14. <a href="../../bitcoin-blockchain/wallet/README.md">Chapter 2.9: Wallet System</a> - Wallet implementation and key management
15. <a href="../../bitcoin-blockchain/web/README.md">Chapter 3: Web API Architecture</a> - REST API implementation
16. <a href="../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md">Chapter 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="../../bitcoin-wallet-ui/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../../bitcoin-wallet-ui/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. **Chapter 8: Docker Compose Deployment** ← *You are here*
21. <a href="../kubernetes/README.md">Chapter 9: Kubernetes Deployment</a> - Kubernetes production guide
22. <a href="../../rust/README.md">Chapter 10: Rust Language Guide</a> - Rust programming language reference

</details>

</div>

---
<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

## Chapter 8, Section 6: Scaling & Deployment

**Part II: Deployment & Operations** | **Chapter 8: Docker Compose Deployment**

<div align="center">

**📚 [← Chapter 6: Web Admin UI](../../bitcoin-web-ui/06-Web-Admin-UI.md)** | **Chapter 8: Docker Compose** | **[Chapter 9: Kubernetes →](../kubernetes/README.md)** 📚

</div>

---

This section explains different scaling methods, how to scale running containers, and best practices for deployment.

> **Methods involved**
> - `docker-compose.scale.sh` (recommended scaling entrypoint, [Listing 8.4](01A-Docker-Compose-Code-Listings.md#listing-84-cidocker-composeconfigsdocker-composescalesh))
> - `generate-compose-ports.sh` (manual override generation, [Listing 8.5](01A-Docker-Compose-Code-Listings.md#listing-85-cidocker-composeconfigsgenerate-compose-portssh))
> - `scale-up.sh`, `scale-down.sh` (incremental scaling, [Listings 8.6–8.7](01A-Docker-Compose-Code-Listings.md))

## Scaling Methods Comparison

### Quick Answer

**For blockchain nodes (ports must be accessible externally):**
- ✅ **Use**: `cd configs && ./docker-compose.scale.sh 3 2` (recommended)
- ❌ **Don't use**: `docker compose up -d --scale miner=3 --scale webserver=2` (only first instance gets ports)

## Detailed Comparison

### Method 1: Helper Script (Recommended for Blockchain)

```bash
cd configs
./docker-compose.scale.sh 3 2
```

**What it does:**
1. ✅ Automatically generates `docker-compose.override.yml` with port mappings for ALL instances
2. ✅ Scales services with all ports accessible externally
3. ✅ All instances have ports mapped:
   - `miner_1`: Port 2001 → `localhost:2001` ✅
   - `miner_2`: Port 2002 → `localhost:2002` ✅
   - `miner_3`: Port 2003 → `localhost:2003` ✅
   - `webserver_1`: Ports 8080, 2101 ✅
   - `webserver_2`: Ports 8081, 2102 ✅

**Result**: ✅ All ports accessible externally (required for blockchain P2P networking)

### Method 2: Direct Docker Compose Command (Not Recommended for Blockchain)

```bash
cd configs
docker compose up -d --scale miner=3 --scale webserver=2
```

**What it does:**
1. ❌ Does NOT generate port override file
2. ❌ Only maps ports for the first instance of each service
3. ❌ Additional instances do NOT have ports mapped:
   - `miner_1`: Port 2001 → `localhost:2001` ✅
   - `miner_2`: Port 2002 → NOT mapped ❌ (only accessible via Docker network)
   - `miner_3`: Port 2003 → NOT mapped ❌ (only accessible via Docker network)
   - `webserver_1`: Ports 8080, 2101 ✅
   - `webserver_2`: Ports 8081, 2102 → NOT mapped ❌

**Result**: ❌ Only first instance ports accessible (insufficient for blockchain)

### Method 3: Manual Override + Direct Command (Works, but Manual)

```bash
cd configs
# Step 1: Generate port override file manually
./generate-compose-ports.sh 3 2

# Step 2: Scale with direct command
docker compose up -d --scale miner=3 --scale webserver=2
```

**What it does:**
1. ✅ Generates `docker-compose.override.yml` with port mappings
2. ✅ Scales services
3. ✅ All instances have ports mapped (same as Method 1)

**Result**: ✅ All ports accessible externally (same as helper script, but requires 2 steps)

## When to Use Each Method

### Use Helper Script (`./docker-compose.scale.sh`) When:
- ✅ **Blockchain nodes** (ports must be accessible for P2P networking)
- ✅ You want automatic port mapping
- ✅ You want simplicity (one command)
- ✅ You're scaling multiple instances

### Use Direct Command (`docker compose up -d --scale`) When:
- ❌ **NOT recommended for blockchain** (only first instance gets ports)
- ✅ You're testing/developing and don't need external access to all instances
- ✅ You only need the first instance accessible externally
- ✅ You're using a load balancer/reverse proxy (only need one entry point)

### Use Manual Override + Direct Command When:
- ✅ You want more control over the process
- ✅ You're integrating into a larger automation script
- ✅ You need to inspect/modify the override file before scaling

## Scaling Running Containers

Docker Compose **can scale running containers** without stopping them. When you run `docker compose up -d --scale`, it:

1. **Keeps existing containers running** - No downtime
2. **Adds new containers** if scaling up
3. **Stops and removes containers** if scaling down (starting from highest instance numbers)

### Scaling Up

```bash
cd configs
# Current: 1 miner, 1 webserver
# Scale to: 3 miners, 2 webservers
./docker-compose.scale.sh 3 2
```

**What happens:**
- `miner_1` continues running (no change)
- `miner_2` container starts (new)
- `miner_3` container starts (new)
- `webserver_1` continues running (no change)
- `webserver_2` container starts (new)

### Scaling Down

```bash
cd configs
# Current: 3 miners, 2 webservers
# Scale to: 2 miners, 1 webserver
./docker-compose.scale.sh 2 1
```

**What happens:**
- `miner_1` continues running
- `miner_2` continues running
- `miner_3` stops and is removed (highest instance number)
- `webserver_1` continues running
- `webserver_2` stops and is removed (highest instance number)

### Incremental Scaling

```bash
# Check current running containers
docker compose ps

# Scale up incrementally
./docker-compose.scale.sh 2 1  # Add 1 more miner
./docker-compose.scale.sh 2 3  # Add 2 more webservers

# Scale down incrementally
./docker-compose.scale.sh 1 1  # Remove extra miners
./docker-compose.scale.sh 1 1  # Remove extra webservers
```

## How New Containers Connect

When you scale up, new containers automatically:

1. **Detect their instance number** from container name (e.g., `blockchain_miner_2`)
2. **Wait for previous node** (if sequential startup enabled)
3. **Connect to the network**:
   - **Miners**: Connect to previous miner (e.g., `miner_2` connects to `miner_1`)
   - **Webservers**: Always connect to first miner (`miner_1:2001`)

### Example: Scaling from 1 to 3 Miners

```bash
# Start with 1 miner
./docker-compose.scale.sh 1 1

# Scale to 3 miners (containers are running)
./docker-compose.scale.sh 3 1
```

**What happens:**
1. `miner_1` continues running (no change)
2. `miner_2` container starts:
   - Detects instance number 2
   - Waits for `miner_1` to be ready
   - Connects to `miner_1:2001`
3. `miner_3` container starts:
   - Detects instance number 3
   - Waits for `miner_2` to be ready
   - Connects to `miner_2:2002`

## Monitoring Scaling

### Check Current Containers

```bash
# List all running containers
docker compose ps

# Check specific service
docker compose ps miner
docker compose ps webserver

# View logs of new containers
docker compose logs -f miner_2
docker compose logs -f webserver_2
```

### Verify Connections

```bash
# Check if new miner connected
docker compose logs miner_2 | grep "connected"

# Check webserver health
curl http://localhost:8080/api/health/ready  # First webserver
curl http://localhost:8081/api/health/ready  # Second webserver (if ports mapped)
```

### Verify Port Mappings

```bash
# Check port mappings
docker compose ps --format "table {{.Name}}\t{{.Ports}}"

# Test ports
curl http://localhost:2001  # miner_1
curl http://localhost:2002  # miner_2 (if mapped)
curl http://localhost:8080/api/health/ready  # webserver_1
```

## Important Notes

### Port Mapping Limitation

**Important**: When using `--scale` directly (without helper script), Docker Compose only maps ports for the **first instance** of each service:
- `miner_1`: Port 2001 mapped to host ✅
- `miner_2`, `miner_3`: Use internal ports only (accessible via Docker network) ❌

**Note**: If you run `docker compose up -d` **without** `--scale`, you get 1 instance and its ports ARE mapped. The limitation only applies when scaling to multiple instances.

To access additional instances externally, you need to:
1. Use the helper script (recommended)
2. Use Docker network directly
3. Set up a reverse proxy/load balancer
4. Use port ranges (requires docker-compose.yml modification)

### Data Persistence

Each instance maintains its own data directory:
- `miner_1`: `/app/data/data1` (in volume `miner-data`)
- `miner_2`: `/app/data/data2` (in volume `miner-data`)
- `webserver_1`: `/app/data/data1` (in volume `webserver-data`)
- `webserver_2`: `/app/data/data2` (in volume `webserver-data`)

When scaling down, the data directories remain in the volume (not deleted). This means:
- If you scale back up, the data will still be there
- To start fresh, you need to remove volumes: `docker compose down -v`

### Sequential Startup

When scaling up, new containers wait for previous nodes:
- `miner_2` waits for `miner_1` to be ready
- `miner_3` waits for `miner_2` to be ready
- `webserver_2` waits for `miner_1` to be ready (webservers connect to miners)

This ensures proper network connectivity. See [Chapter 7: Sequential Startup](07-Sequential-Startup.md) for details.

## Troubleshooting

### Containers Not Starting

```bash
# Check logs
docker compose logs miner_2

# Common issues:
# - Previous node not ready (check wait script output)
# - Port conflicts (unlikely with dynamic ports)
# - Volume mount issues
```

### Scaling Down Issues

```bash
# If containers won't stop
docker compose stop miner_3
docker compose rm -f miner_3

# Then scale again
./docker-compose.scale.sh 2 1
```

### Verify Scaling Worked

```bash
# Count running containers
docker compose ps -q miner | wc -l  # Should match target count
docker compose ps -q webserver | wc -l  # Should match target count

# Check all containers are healthy
docker compose ps --format "table {{.Name}}\t{{.Status}}"
```

## Example: Complete Scaling Workflow

```bash
# 1. Start with default (1 miner, 1 webserver)
cd configs
./docker-compose.scale.sh 1 1

# 2. Wait for services to be healthy
docker compose ps

# 3. Scale up to 3 miners, 2 webservers
./docker-compose.scale.sh 3 2

# 4. Monitor new containers starting
docker compose logs -f miner_2 miner_3 webserver_2

# 5. Verify all containers are running
docker compose ps

# 6. Verify ports are accessible
curl http://localhost:2001  # miner_1
curl http://localhost:2002  # miner_2
curl http://localhost:2003  # miner_3
curl http://localhost:8080/api/health/ready  # webserver_1
curl http://localhost:8081/api/health/ready  # webserver_2

# 7. Scale down to 2 miners, 1 webserver
./docker-compose.scale.sh 2 1

# 8. Verify scaling down
docker compose ps
```

## Summary Table

| Method | Command | Port Mapping | Recommended for Blockchain? |
|--------|---------|--------------|------------------------------|
| Helper Script | `./docker-compose.scale.sh 3 2` | ✅ All instances | ✅ **YES** |
| Direct Command | `docker compose up -d --scale miner=3 --scale webserver=2` | ❌ First instance only | ❌ **NO** |
| Manual Override + Direct | `./generate-compose-ports.sh 3 2` then `docker compose up -d --scale miner=3 --scale webserver=2` | ✅ All instances | ✅ **YES** (but manual) |

## Recommendation

**For blockchain nodes, always use the helper script:**
```bash
./docker-compose.scale.sh 3 2
```

This ensures all instances have ports accessible externally, which is required for proper P2P networking in a blockchain.

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Port Mapping](05-Port-Mapping.md) | [↑ Table of Contents](#) | [Next Section: Deployment Scenarios →](08-Deployment-Scenarios.md) |
|:---:|:---:|:---:|
| *Section 6* | *Current Section* | *Section 8* |

</div>

---
