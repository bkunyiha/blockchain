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

## Chapter 22, Section 4: Deployment Scenarios & Operations

**Part II: Deployment & Operations** | **Chapter 31: Docker Compose Deployment**

<div align="center">

**[← Section 3: Deployment Topology](03-Deployment-Topology.md)** | **Section 4: Deployment Scenarios & Operations** | **[Section 5: Accessing Webserver →](05-Accessing-Webserver.md)**

</div>

---

## Prerequisites

Before reading this section, you should have:
- Completed [Section 3: Deployment Topology](03-Deployment-Topology.md)
- Understanding of deployment procedures
- Basic familiarity with Docker Compose operations

## Learning Objectives

After reading this section, you will understand:
- Common deployment scenarios and when to use each
- How to deploy script changes effectively
- Complete step-by-step execution walkthroughs
- Operational best practices and troubleshooting
- Production deployment procedures

---

This section combines practical deployment scenarios, operational procedures, and detailed execution walkthroughs to give you complete understanding of how to deploy and operate the blockchain network.

> **Methods involved:**
> - Compose topology: `docker-compose.yml` ([Listing 22A.1](01A-Docker-Compose-Code-Listings.md#listing-22a1-cidocker-composeconfigsdocker-composeyml))
> - Instance wiring + port math + connect-node resolution: `docker-entrypoint.sh` ([Listing 22A.2](01A-Docker-Compose-Code-Listings.md#listing-22a2-cidocker-composeconfigsdocker-entrypointsh))
> - Scaling helpers (override generation): [Listings 8.4–8.7](01A-Docker-Compose-Code-Listings.md)
> - `docker-entrypoint.sh` orchestration logic ([Listing 22A.2](01A-Docker-Compose-Code-Listings.md#listing-22a2-cidocker-composeconfigsdocker-entrypointsh))
> - `wait-for-node.sh` readiness gating ([Listing 22A.3](01A-Docker-Compose-Code-Listings.md#listing-22a3-cidocker-composeconfigswait-for-nodesh))
> - Docker image build: `Dockerfile` ([Listing 22A.11](01A-Docker-Compose-Code-Listings.md#listing-22a11-cidocker-composeconfigsdockerfile))

---

## Deployment Scenarios

### Scenario 1: Single Miner + Single Webserver

#### Configuration

```bash
cd configs
docker compose up -d
```

#### Network Topology

```text
redis:6379 (rate limiting backend)
    ↑
miner_1:2001 (seed node, "local")
    ↑
webserver_1:2101 → connects to miner_1:2001
```

#### Details

- **Redis**: Provides shared state for the webserver's Redis-backed API rate limiting.
- **Miner 1**: Acts as seed node, P2P port 2001
- **Webserver 1**: Connects to miner_1:2001, Web port 8080, P2P port 2101
- **Ports accessible**: 2001, 8080, 2101
- **Use case**: Development, testing, small deployments

#### Access Points

- Miner P2P: `localhost:2001`
- Webserver API: `http://localhost:8080`
- Webserver P2P: `localhost:2101`

### Scenario 2: Single Miner + Multiple Webservers

#### Configuration

```bash
cd configs
./docker-compose.scale.sh 1 3
```

#### Network Topology

```text
miner_1:2001 (seed node, "local")
    ↑
    ├── webserver_1:2101 → connects to miner_1:2001
    ├── webserver_2:2102 → connects to miner_1:2001
    └── webserver_3:2103 → connects to miner_1:2001
```

#### Details

- **Miner 1**: Acts as seed node, P2P port 2001
- **Webserver instances**: Web ports 8080-8082, P2P ports 2101-2103
- **All webservers connect to miner_1:2001** (star topology)
- **Use case**: High availability web services, load balancing

### Scenario 3: Multiple Miners + Single Webserver

#### Configuration

```bash
cd configs
./docker-compose.scale.sh 3 1
```

#### Network Topology

```text
miner_1:2001 (seed, "local")
    ↑
miner_2:2002 → connects to miner_1:2001
    ↑
miner_3:2003 → connects to miner_2:2002
    ↑
webserver_1:2101 → connects to miner_1:2001
```

#### Details

- **Miners form a chain**: miner_1 → miner_2 → miner_3
- **Webserver 1**: Connects to miner_1:2001
- **Use case**: Distributed mining, consensus testing

### Scenario 4: Multiple Miners + Multiple Webservers (Production)

#### Configuration

```bash
cd configs
./docker-compose.scale.sh 3 2
```

#### Network Topology

```text
miner_1:2001 (seed, "local")
    ↑
    ├── miner_2:2002 → connects to miner_1:2001
    │       ↑
    │   miner_3:2003 → connects to miner_2:2002
    │
    ├── webserver_1:2101 → connects to miner_1:2001
    └── webserver_2:2102 → connects to miner_1:2001
```

#### Details

- **Miners form a chain**: miner_1 → miner_2 → miner_3
- **All webservers connect to miner_1**: Star topology around first miner
- **Use case**: Production deployment, high availability

---

## Deploying Script Changes

The `docker-entrypoint.sh` and `wait-for-node.sh` scripts are copied into the Docker image during build. To deploy changes to these scripts, you **MUST rebuild the Docker image** - Docker will use cached layers otherwise and your changes won't be applied.

### IMPORTANT: Always Rebuild

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
   Note: `--no-cache` ensures a fresh build with the latest scripts.

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

### Production Deployment

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

### Environment Variables

#### Required Variables

- **`NODE_MINING_ADDRESS`**: Must be set for miners (or use `WALLET_ADDRESS_POOL`)

#### Optional Variables

- **`DEBUG`**: Set to `1` to enable verbose debug logging
- **`SEQUENTIAL_STARTUP`**: Set to `yes` (default) or `no`
- **`BITCOIN_API_ADMIN_KEY`**: Admin API key (default: `admin-secret`)
- **`BITCOIN_API_WALLET_KEY`**: Wallet API key (default: `wallet-secret`)

#### Setting Environment Variables

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

---

## Detailed Execution Walkthrough

### Overview

This section provides a step-by-step walkthrough of how the Docker Compose deployment system operates. Through practical examples, you will learn how containers initialize, how nodes discover and connect to each other, and how the system ensures reliable startup sequencing.

### Scenario 1: Webserver Container Startup

This scenario demonstrates the most common deployment case: a webserver container starting and establishing a connection to the first miner node.

#### Initial Container State

**Container:** `configs-webserver-1`
**Environment Variables:**
```bash
NODE_IS_MINER=no
NODE_IS_WEB_SERVER=yes
NODE_CONNECT_NODES=miner_1:2001
INSTANCE_NUMBER=1
HOSTNAME=configs-webserver-1
```

#### Step 1: Container Identity Extraction

The entrypoint script extracts identifying information from the container's hostname.

```bash
CONTAINER_NAME="${HOSTNAME:-}"  # CONTAINER_NAME="configs-webserver-1"

# Extract instance number
if [[ "${CONTAINER_NAME}" =~ _([0-23]+)$ ]]; then
    INSTANCE_NUMBER="${BASH_REMATCH[1]}"  # INSTANCE_NUMBER=1
fi
```

**Result:**
- `CONTAINER_NAME="configs-webserver-1"`
- `INSTANCE_NUMBER=1`

#### Step 2: Service Type Detection

Based on the container name, the system determines the service type and calculates appropriate port numbers.

```bash
# Determine service type from container name
if [[ "${CONTAINER_NAME}" =~ webserver ]]; then
    SERVICE_TYPE="webserver"
    WEB_PORT=$((8080 + INSTANCE_NUMBER - 1))  # WEB_PORT=8080
    P2P_PORT=$((2101 + INSTANCE_NUMBER - 1))  # P2P_PORT=2101
fi
```

**Result:**
- `SERVICE_TYPE="webserver"`
- `WEB_PORT=8080`
- `P2P_PORT=2101`

#### Step 3: Sequential Startup Coordination

When sequential startup is enabled, the webserver waits for the miner to be ready before proceeding.

#### Step 4: DNS Resolution and Connection Configuration

The system resolves hostnames to IP addresses before passing them to the Rust binary.

**Result:**
- `NODE_CONNECT_NODES` is an IP address (Rust can parse it)
- Webserver will connect to miner successfully

### Scenario 2: Additional Miner Instance Startup

This scenario demonstrates how additional miner instances connect to the existing network. When `miner_2` starts, it must discover and connect to `miner_1`.

#### Initial Container State

**Container:** `configs-miner-2`
**Environment Variables:**
```bash
NODE_IS_MINER=yes
NODE_IS_WEB_SERVER=no
NODE_CONNECT_NODES=local
INSTANCE_NUMBER=2
HOSTNAME=configs-miner-2
```

#### Instance Number Extraction

The system extracts the instance number from the container name:

```bash
CONTAINER_NAME="configs-miner-2"
if [[ "${CONTAINER_NAME}" =~ _([0-23]+)$ ]]; then
    INSTANCE_NUMBER="${BASH_REMATCH[1]}"  # INSTANCE_NUMBER=2
fi
```

#### Port Calculation

Ports are calculated based on the instance number to avoid conflicts:

```bash
if [[ "${CONTAINER_NAME}" =~ miner ]]; then
    SERVICE_TYPE="miner"
    P2P_PORT=$((2001 + INSTANCE_NUMBER - 1))  # P2P_PORT=2002
fi
```

#### Sequential Startup Coordination

The miner waits for the previous miner instance to be ready.

```bash
if [ "${SEQUENTIAL_STARTUP}" = "yes" ]; then
    # For miners, wait for previous instance
    WAIT_INSTANCE_NUMBER=2

    WAIT_OUTPUT=$(
        /app/wait-for-node.sh "miner" "2" "2001" "no"
    )
```

#### Node Discovery

The wait script discovers the previous miner instance and outputs its address.

**Result:**
- `PREV_NODE_ADDRESS=miner_1:2001`

#### Address Resolution

The system converts and resolves the address to an IP.

```bash
PREV_ADDR="miner_1:2001"

# Convert miner_1 to miner for DNS resolution
if [[ "${PREV_ADDR}" =~ ^miner_([0-23]+): ]]; then
    RESOLVE_ADDR="miner:2001"  # Converted for DNS
fi

# Resolve to IP
PREV_ADDR_RESOLVED=$(resolve_hostname_to_ip "miner:2001")
# PREV_ADDR_RESOLVED="172.19.0.2:2001"
```

#### Miner Connection Configuration

The miner is configured to connect to the previous miner.

**Result:**
- `NODE_CONNECT_NODES="172.19.0.2:2001"`
- `miner_2` will connect to `miner_1`

### Complete Startup Workflow

```bash
cd ci/docker-compose/configs
NODE_MINING_ADDRESS="1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa" docker compose up -d

# Check webserver logs
docker compose logs webserver | grep "Auto-configured"

# Verify webserver connection
curl http://localhost:8080/api/health/ready
```

All these steps should succeed without errors, demonstrating that the deployment system is working correctly.

---

## Best Practices

### 1. Always Use Helper Script for Scaling

```bash
# ✅ Good
./docker-compose.scale.sh 3 2

# ❌ Bad (only first instance gets ports)
docker compose up -d --scale miner=3 --scale webserver=2
```

### 2. Verify Ports After Scaling

```bash
./docker-compose.scale.sh 3 2
docker compose ps  # Verify all containers running
curl http://localhost:2001  # Test miner_1
curl http://localhost:2002  # Test miner_2
curl http://localhost:2003  # Test miner_3
```

### 3. Monitor Startup Sequence

```bash
# Watch logs during startup
docker compose logs -f

# Look for sequential startup messages
docker compose logs miner_2 | grep -i wait
```

### 4. Use Health Checks

```bash
# Check webserver health
curl http://localhost:8080/api/health/ready

# Check miner port
nc -zv localhost 2001
```

### 5. Data Persistence

```bash
# Data persists in volumes
docker compose down  # Stops containers, keeps volumes
docker compose down -v  # Stops containers, removes volumes (deletes data)
```

---

## Common Patterns

### Pattern 1: Horizontal Scaling

Scale webservers for increased web traffic:

```bash
# Start with 1 miner, 1 webserver
./docker-compose.scale.sh 1 1

# Scale webservers to 5
./docker-compose.scale.sh 1 5
```

### Pattern 2: Vertical Mining

Scale miners for increased mining power:

```bash
# Start with 1 miner, 1 webserver
./docker-compose.scale.sh 1 1

# Scale miners to 5
./docker-compose.scale.sh 5 1
```

### Pattern 3: Balanced Deployment

Equal miners and webservers:

```bash
# 3 miners, 3 webservers
./docker-compose.scale.sh 3 3
```

---

## Troubleshooting

### Scenario: Ports Not Accessible

**Problem**: After scaling, only first instance ports are accessible.

**Solution**: Use helper script to generate port mappings:
```bash
./docker-compose.scale.sh 3 2
```

### Scenario: Nodes Not Connecting

**Problem**: Nodes fail to connect to each other.

**Solution**: Check sequential startup is enabled and previous nodes are ready:
```bash
docker compose logs miner_2 | grep -i wait
docker compose logs miner_2 | grep -i connect
```

### Scenario: Webservers Not Connecting to Miner

**Problem**: Webservers can't connect to miner.

**Solution**: Verify miner is healthy and webserver NODE_CONNECT_NODES:
```bash
docker compose exec webserver_1 env | grep NODE_CONNECT_NODES
docker compose exec miner_1 nc -zv localhost 2001
```

### Scenario: Containers Fail to Start

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
   docker compose down -v
   docker compose build --no-cache
   docker compose up -d
   ```

### Scenario: Script Changes Not Applied

If script changes aren't being applied:

1. **Ensure you are using `--no-cache`:**
   ```bash
   docker compose build --no-cache
   ```

2. **Verify scripts are copied in Dockerfile:**
   ```bash
   grep -E "COPY.*docker-entrypoint|COPY.*wait-for-node" Dockerfile
   ```

3. **Force recreate containers:**
   ```bash
   docker compose up -d --force-recreate
   ```

---

## Summary Table

| Scenario | Command | Result |
|----------|---------|--------|
| Development | `docker compose up -d` | 1 miner + 1 webserver |
| Scale webservers | `./docker-compose.scale.sh 1 3` | 1 miner + 3 webservers |
| Scale miners | `./docker-compose.scale.sh 3 1` | 3 miners + 1 webserver |
| Production | `./docker-compose.scale.sh 5 3` | 5 miners + 3 webservers |
| Deploy changes | `docker compose build --no-cache && docker compose up -d` | Rebuild image + restart |

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Deployment Topology](03-Deployment-Topology.md) | [↑ Table of Contents](#) | [Next Section: Accessing Webserver →](05-Accessing-Webserver.md) |
|:---:|:---:|:---:|
| *Section 3* | *Current Section* | *Section 5* |

</div>

---
