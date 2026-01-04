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

# Chapter 8: Docker Compose Deployment - Complete Guide

**Part II: Deployment & Operations**

<div align="center">

**📚 [← Chapter 6: Web Admin UI](../../bitcoin-web-ui/06-Web-Admin-UI.md)** | **Chapter 8: Docker Compose** | **[Chapter 9: Kubernetes →](../kubernetes/README.md)** 📚

</div>

---

**Important**: All commands in this guide should be run from the `configs/` directory where `docker-compose.yml` and the helper scripts are located.

```bash
cd ci/docker-compose/configs  # Navigate to configs directory first
```

## Table of Contents

**Learning Path - Read in Order:**

1. [Introduction & Quick Start](#introduction--quick-start) - *Start here*
2. [Architecture & Container System](02-Architecture.md) - *Understand the system structure*
3. [Execution Flow & Startup Process](03-Execution-Flow.md) - *Learn how the system operates*
4. [Network Configuration & Node Connections](04-Network-Configuration.md) - *Understand networking fundamentals*
5. [Sequential Startup](07-Sequential-Startup.md) - *Learn startup coordination*
6. [Port Mapping & External Access](05-Port-Mapping.md) - *Configure external connectivity*
7. [Scaling & Deployment](06-Scaling.md) - *Scale your deployment*
8. [Deployment Scenarios & Examples](08-Deployment-Scenarios.md) - *Practical deployment patterns*
9. [Accessing Webserver](09-Accessing-Webserver.md) - *Use the web interface*
10. [Deployment Guide](10-Deployment-Guide.md) - *Production deployment procedures*
11. [Deployment Execution Walkthrough](11-Deployment-Execution-Walkthrough.md) - *Deep dive into execution details*
12. [DNS Resolution Mechanism](12-DNS-Resolution-Mechanism.md) - *Advanced networking internals*

---

## Introduction & Quick Start

### Overview

In this chapter, we'll learn how to deploy and manage our blockchain network using Docker Compose. We'll discover how the system supports multiple miners and webservers, with automatic port configuration, sequential startup, and flexible scaling options. By the end of this chapter, you'll be able to deploy a fully functional blockchain network and understand how all the pieces work together.

### Our Learning Journey

We've organized this chapter to guide you from basic concepts to advanced implementation details. As we progress, you'll build a complete understanding of how the deployment system works:

1. **We'll Start Here**: Introduction & Quick Start (this section) - We'll get you up and running quickly so you can see the system in action.

2. **We'll Build the Foundation**: Architecture → Execution Flow → Network Configuration - We'll explore how the system works under the hood, understanding the architecture and execution flow.

3. **We'll Learn Operations**: Sequential Startup → Port Mapping → Scaling - We'll discover operational concepts that make the system reliable and scalable.

4. **We'll Practice**: Deployment Scenarios → Accessing Webserver - We'll work through practical examples that show real-world deployment patterns.

5. **We'll Go Deep**: Deployment Guide → Execution Walkthrough → DNS Resolution - We'll dive into the internals, understanding every detail of how the system operates.

**What You'll Need:**
- Basic understanding of Docker and Docker Compose (we'll explain as we go)
- Familiarity with command-line interfaces
- Knowledge of blockchain concepts is helpful but not required—we'll explain what you need to know

### Quick Start

Let's get started! We'll begin with the simplest deployment: one miner and one webserver. Before we start, we need to set up wallet addresses. We have two methods to choose from:

**Method 1: Address Pool (Recommended)**

This method is more flexible and scales better. We set up a pool of addresses, and each node automatically selects one:

```bash
cd ci/docker-compose/configs
# Set up address pool (at least 2 addresses for 1 miner + 1 webserver)
export WALLET_ADDRESS_POOL="addr1,addr2"
docker compose up -d
# miner_1 → selects addr1 (index 0)
# webserver_1 → selects addr2 (index 1)
```

**What starts with `docker compose up -d`:**
- `redis` (required by the webserver’s Redis-backed API rate limiting)
- `miner` (P2P + mining)
- `webserver` (REST API + Swagger UI + rate limiting)

**Rate limiting configuration:**
- The webserver loads `axum_rate_limiter` settings from `ci/docker-compose/configs/Settings.toml`.
- Compose mounts it into the container as `/app/Settings.toml` and sets `RL_SETTINGS_PATH=/app/Settings.toml`.
- To change limits/strategies, edit `configs/Settings.toml` and restart the webserver.

**Method 2: Direct Assignment**

If you prefer to assign addresses directly, you can use this simpler approach:

```bash
cd ci/docker-compose/configs
# Use same address for both (or set different addresses per service)
export NODE_MINING_ADDRESS=3npBNyKSEwhCQWTXHFjwR8Rb66kjq6khfZSdmLPm8Gde9XoTwW
docker compose up -d
```

When we run `docker compose up -d`, it uses the main `docker-compose.yml` which includes both `miner` and `webserver` services. Once started, the webserver will be accessible at `http://localhost:8080`.

**A Note on Sequential Startup:** By default, sequential startup is enabled. This means each node waits for the previous node to be ready before starting. This ensures a stable network formation. We'll explore this mechanism in detail in [Section 5: Sequential Startup](07-Sequential-Startup.md).

For detailed information on accessing the webserver, see [Section 9: Accessing Webserver](09-Accessing-Webserver.md). For deployment instructions, see [Section 10: Deployment Guide](10-Deployment-Guide.md).

We'll explore wallet address distribution in more detail in the [Wallet Address Distribution](#wallet-address-distribution) section below.

#### Scaling to Multiple Instances

Once you're comfortable with the basic setup, you'll likely want to scale to multiple instances. Before scaling, we need to set up wallet addresses. We'll use `WALLET_ADDRESS_POOL` with enough addresses for all our nodes:

```bash
cd ci/docker-compose/configs
# Set up address pool (need at least 5 addresses for 3 miners + 2 webservers)
export WALLET_ADDRESS_POOL="addr1,addr2,addr3,addr4,addr5"
# Scale to 3 miners and 2 webservers
./docker-compose.scale.sh 3 2
# miner_1 → addr1, miner_2 → addr2, miner_3 → addr3
# webserver_1 → addr4, webserver_2 → addr5
```

**<span style="color:red;">Important</span>**: When using `--scale` directly with `docker compose up -d --scale`, only the first instance gets ports mapped. To ensure **ALL instances have ports accessible externally**, always use the helper script `./docker-compose.scale.sh`.

The helper script:
- ✅ Automatically generates `docker-compose.override.yml` with port mappings for all instances
- ✅ Ensures ALL instances have ports accessible externally
- ✅ Works with running containers (no downtime)

### Alternative Compose Files

There are also standalone alternative compose files that can be used instead:

- **`docker-compose.miner.yml`**: Run only a miner (no webserver)
  ```bash
  docker compose -f docker-compose.miner.yml up -d
  ```

- **`docker-compose.webserver.yml`**: Run only a webserver (no miner)
  ```bash
  docker compose -f docker-compose.webserver.yml up -d
  ```

  Note: this file also starts a `redis` service for rate limiting.

**Note:** These alternative files are **NOT automatically called**. They're manual alternatives you can use with the `-f` flag if you want to run only miners or only webservers separately. The main `docker-compose.yml` is recommended for most use cases as it supports scaling both services together.

### Port Assignments

#### Internal Ports (within Docker network)

Each instance automatically configures unique internal ports:

**Miners:**
- Instance 1: P2P port 2001
- Instance 2: P2P port 2002
- Instance 3: P2P port 2003
- ... and so on

**Webservers:**
- Instance 1: Web port 8080, P2P port 2101
- Instance 2: Web port 8081, P2P port 2102
- Instance 3: Web port 8082, P2P port 2103
- ... and so on

For detailed information about port mapping and external access, see [Port Mapping & External Access](#port-mapping--external-access) and [05-Port-Mapping.md](05-Port-Mapping.md).

### Data Directories

Each instance uses its own isolated data directory:
- Instance 1: `data1/`
- Instance 2: `data2/`
- Instance 3: `data3/`
- ... and so on

Data is persisted in Docker volumes:
- `miner-data` and `miner-wallets` for miners
- `webserver-data` and `webserver-wallets` for webservers

### Accessing the Wallet File

The wallet file (`wallets.dat`) is stored inside each container at `/app/wallets/wallets.dat` and persisted in Docker volumes.

#### Location Details

**Inside Container:**
- Full path: `/app/wallets/wallets.dat`
- Working directory: `/app` (set in Dockerfile)
- Environment variable: `WALLET_FILE=wallets/wallets.dat`

**Docker Volumes:**
- **Miners**: Volume `miner-wallets` mounted at `/app/wallets`
- **Webservers**: Volume `webserver-wallets` mounted at `/app/wallets`

**Important (Docker Compose volume names):** Docker Compose prefixes volume names with the **project name**.
For example, if you run from `ci/docker-compose/configs/`, the project name is typically `configs`, so volumes become:

- Miner wallets: `configs_miner-wallets`
- Webserver wallets: `configs_webserver-wallets`

#### Accessing Wallet Files

**Method 1: Via Container Exec**

```bash
# List wallet files in a container
docker exec -it <container_name> ls -la /app/wallets/

# View wallet file (binary, use with caution)
docker exec -it <container_name> cat /app/wallets/wallets.dat

# Check if wallet file exists
docker exec -it <container_name> test -f /app/wallets/wallets.dat && echo "Wallet file exists"
```

**Method 2: Via Volume Inspection (Find Host Path)**

```bash
# List all volumes
docker volume ls | grep wallets

# Inspect volume to find host path
docker volume inspect configs_miner-wallets
docker volume inspect configs_webserver-wallets

# Or print only the mountpoint:
docker volume inspect configs_webserver-wallets --format '{{.Mountpoint}}'
```

**Host Filesystem Location:**

The actual location on your local server depends on your operating system:

**Linux:**
```bash
# Volumes are stored at:
/var/lib/docker/volumes/<volume_name>/_data

# Example paths:
/var/lib/docker/volumes/configs_miner-wallets/_data/wallets.dat
/var/lib/docker/volumes/configs_webserver-wallets/_data/wallets.dat

# Find exact path:
docker volume inspect configs_webserver-wallets | grep Mountpoint
```

**macOS (Docker Desktop):**
```bash
# Docker Desktop stores volumes on local macOS storage, but inside a Linux VM
# The VM's filesystem is stored at:
# ~/Library/Containers/com.docker.docker/Data/vms/0/data/docker/volumes/

# Use docker volume inspect to get the path inside the VM:
docker volume inspect configs_webserver-wallets

# Output shows "Mountpoint" which is the path inside the Docker VM
# Example: /var/lib/docker/volumes/blockchain_miner-wallets/_data

# The VM filesystem is stored on macOS at:
# ~/Library/Containers/com.docker.docker/Data/vms/0/data/docker/volumes/<volume_name>/_data

# However, accessing files directly requires navigating the VM disk image
# Easier methods: use docker exec or docker cp commands
```

**Windows (Docker Desktop):**
```bash
# Similar to macOS, volumes are stored on local Windows storage inside a Linux VM
# Use docker volume inspect to get the path inside the VM
docker volume inspect blockchain_miner-wallets

# To access from host, use docker exec or docker cp commands
```

**Finding the Exact Path:**

```bash
# Get the mountpoint (path inside Docker VM)
docker volume inspect blockchain_miner-wallets --format '{{ .Mountpoint }}'

# On Linux (direct host access):
# Full wallet file path:
$(docker volume inspect blockchain_miner-wallets --format '{{ .Mountpoint }}')/wallets.dat

# Example: List files in volume on Linux
sudo ls -la $(docker volume inspect blockchain_miner-wallets --format '{{ .Mountpoint }}')

# On macOS (VM filesystem stored locally):
# VM path: /var/lib/docker/volumes/blockchain_miner-wallets/_data
# macOS storage location: ~/Library/Containers/com.docker.docker/Data/vms/0/data/docker/volumes/blockchain_miner-wallets/_data
# Note: The VM disk image format makes direct file access complex
# Recommended: Use docker exec or docker cp for file operations
```

**Method 3: Copy Wallet File to Host**

```bash
# Copy wallet file from container to host
docker cp <container_name>:/app/wallets/wallets.dat ./wallets-backup.dat

# Copy wallet file from host to container
docker cp ./wallets-backup.dat <container_name>:/app/wallets/wallets.dat
```

**Method 4: Mount Host Directory (Development)**

For development, you can mount a host directory instead of using volumes:

```yaml
# In docker-compose.override.yml
services:
  miner:
    volumes:
      - ./local-wallets:/app/wallets  # Mount host directory
```

**Important Notes:**
- Each service type (miner/webserver) has its own isolated wallet volume
- Wallet files are binary-encoded (using bincode) and not human-readable
- Back up wallet volumes before removing containers: `docker volume export <volume-name> > backup.tar`
- Wallet files contain private keys - handle with appropriate security measures

### Configuration

#### Environment Variables

You can set these environment variables before running `docker compose up`:

```bash
# Number of instances (alternative to --scale)
export NUM_MINERS=3
export NUM_WEBSERVERS=2

# Mining addresses (one per miner instance)
export MINER_ADDRESS_1=3npBNyKSEwhCQWTXHFjwR8Rb66kjq6khfZSdmLPm8Gde9XoTwW
export MINER_ADDRESS_2=another-address-here
export MINER_ADDRESS_3=yet-another-address

# Connect nodes
export MINER_CONNECT_NODES=local
export WEBSERVER_CONNECT_NODES=127.0.0.1:2001

# API keys (for webservers)
export BITCOIN_API_ADMIN_KEY=your-admin-key
export BITCOIN_API_WALLET_KEY=your-wallet-key
```

#### Wallet Address Distribution

**Two methods are supported for distributing wallet addresses:**

**Method 1: Address Pool (Recommended for Scaling)**

Use `WALLET_ADDRESS_POOL` with a comma-separated list of addresses. Each node automatically selects an address based on its instance number:

```bash
# Create a pool of addresses (one per instance you plan to run)
export WALLET_ADDRESS_POOL="addr1,addr2,addr3,addr4,addr5"

# Start multiple instances - each gets a different address automatically
docker compose up -d --scale miner=3 --scale webserver=2
# miner_1 → addr1, miner_2 → addr2, miner_3 → addr3
# webserver_1 → addr4, webserver_2 → addr5
```

**How it works:**
- Instance 1 selects index 0 (first address)
- Instance 2 selects index 1 (second address)
- Instance 3 selects index 2 (third address)
- And so on...

**Method 2: Direct Assignment**

Use `NODE_MINING_ADDRESS` to assign the same address to all instances:

```bash
export NODE_MINING_ADDRESS=3npBNyKSEwhCQWTXHFjwR8Rb66kjq6khfZSdmLPm8Gde9XoTwW
docker compose up -d --scale miner=3
# All miners use the same address
```

**Method 3: Per-Service Override (Advanced)**

For different pools per service, use `docker-compose.override.yml`:

```yaml
services:
  miner:
    environment:
      - WALLET_ADDRESS_POOL=${MINER_ADDRESS_POOL}
  webserver:
    environment:
      - WALLET_ADDRESS_POOL=${WEBSERVER_ADDRESS_POOL}
```

```bash
export MINER_ADDRESS_POOL="miner-addr1,miner-addr2,miner-addr3"
export WEBSERVER_ADDRESS_POOL="webserver-addr1,webserver-addr2"
docker compose up -d --scale miner=3 --scale webserver=2
```

### Wallet Address Distribution

The blockchain node requires a wallet address (`wlt_mining_addr`) to be set on startup. Docker Compose supports two methods for distributing addresses:

#### Method 1: Address Pool (Recommended)

Use `WALLET_ADDRESS_POOL` with a comma-separated list. Each node automatically selects an address based on its instance number:

```bash
# Set up a pool of addresses (one per instance)
export WALLET_ADDRESS_POOL="addr1,addr2,addr3,addr4,addr5"

# Start multiple instances
docker compose up -d --scale miner=3 --scale webserver=2

# Result:
# - miner_1 (instance 1) → selects addr1 (index 0)
# - miner_2 (instance 2) → selects addr2 (index 1)
# - miner_3 (instance 3) → selects addr3 (index 2)
# - webserver_1 (instance 1) → selects addr4 (index 0, but in webserver context)
# - webserver_2 (instance 2) → selects addr5 (index 1)
```

**Important Notes:**
- Instance numbers are 1-indexed (1, 2, 3...)
- Array indices are 0-indexed (0, 1, 2...)
- Each service type (miner/webserver) has its own instance numbering
- Ensure your pool has enough addresses for all instances you plan to run

#### Method 2: Direct Assignment

Use `NODE_MINING_ADDRESS` to assign the same address to all instances:

```bash
export NODE_MINING_ADDRESS=3npBNyKSEwhCQWTXHFjwR8Rb66kjq6khfZSdmLPm8Gde9XoTwW
docker compose up -d --scale miner=3
# All 3 miners use the same address
```

#### Method 3: Per-Service Pools (Advanced)

For different address pools per service type:

```bash
# Create separate pools
export MINER_POOL="miner-addr1,miner-addr2,miner-addr3"
export WEBSERVER_POOL="webserver-addr1,webserver-addr2"

# Use docker-compose.override.yml to assign per service
```

Create `docker-compose.override.yml`:
```yaml
services:
  miner:
    environment:
      - WALLET_ADDRESS_POOL=${MINER_POOL}
  webserver:
    environment:
      - WALLET_ADDRESS_POOL=${WEBSERVER_POOL}
```

### Basic Examples

#### Example 1: Single Miner and Webserver (Address Pool)

```bash
# Set up address pool with 2 addresses
export WALLET_ADDRESS_POOL="3npBNyKSEwhCQWTXHFjwR8Rb66kjq6khfZSdmLPm8Gde9XoTwW,1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"

# Start services
cd ci/docker-compose/configs
docker compose up -d

# Result:
# - miner_1 → uses first address from pool
# - webserver_1 → uses second address from pool
```

#### Example 1b: Single Miner and Webserver (Direct Assignment)

```bash
# Use same address for both
export NODE_MINING_ADDRESS=3npBNyKSEwhCQWTXHFjwR8Rb66kjq6khfZSdmLPm8Gde9XoTwW

cd ci/docker-compose/configs
docker compose up -d
```

```bash
docker compose up -d
```

Access webserver at: http://localhost:8080

#### Example 2: Multiple Miners, Single Webserver

```bash
./docker-compose.scale.sh 3 1
```

This creates:
- 3 miners on P2P ports 2001, 2002, 2003
- 1 webserver on Web port 8080, P2P port 2101

#### Example 3: Single Miner, Multiple Webservers

```bash
./docker-compose.scale.sh 1 3
```

This creates:
- 1 miner on P2P port 2001
- 3 webservers on Web ports 8080, 8081, 8082 and P2P ports 2101, 2102, 2103

### Monitoring

#### View Running Containers

```bash
docker compose ps
```

#### View Logs

```bash
# All services
docker compose logs -f

# Specific service
docker compose logs -f miner
docker compose logs -f webserver

# Specific instance
docker compose logs -f miner_1
docker compose logs -f webserver_2
```

#### Check Health

```bash
# Webserver health check
curl http://localhost:8080/api/health/liveness
curl http://localhost:8081/api/health/liveness  # Instance 2
```

### Stopping Services

```bash
# Stop all services
docker compose down

# Stop and remove volumes (deletes all data)
docker compose down -v
```

### Troubleshooting

#### Port Conflicts

If you get port conflicts, check which ports are in use:

```bash
docker compose ps
netstat -tulpn | grep -E '2001|8080|2101'
```

#### Instance Numbers Not Detected

If instance numbers aren't detected correctly, you can set them explicitly:

```bash
INSTANCE_NUMBER=1 docker compose up -d --scale miner=1
INSTANCE_NUMBER=2 docker compose up -d --scale miner=2
```

#### Data Persistence

Each instance's data is stored in separate directories within the Docker volumes. To access data:

```bash
# Inspect volumes
docker volume ls
docker volume inspect blockchain_miner-data

# Access container filesystem
docker compose exec miner_1 ls -la /app/data1
```

---

## Architecture & Container System

See [02-Architecture.md](02-Architecture.md) for detailed information about:
- Container naming conventions
- Instance number detection
- Service type identification
- Volume and data directory structure
- Container lifecycle

---

## Execution Flow & Startup Process

See [03-Execution-Flow.md](03-Execution-Flow.md) for detailed information about:
- Complete code execution order
- Docker Compose initialization
- Container startup sequence
- Entrypoint script execution
- Blockchain binary execution
- Health checks and dependencies

---

## Network Configuration & Node Connections

See [04-Network-Configuration.md](04-Network-Configuration.md) for detailed information about:
- How nodes connect to each other
- Miner connection chain
- Webserver connection behavior
- NODE_CONNECT_NODES configuration
- Network topology

---

## Port Mapping & External Access

See [05-Port-Mapping.md](05-Port-Mapping.md) for detailed information about:
- Port mapping limitations with `--scale`
- Using the scaling helper script
- Manual port override generation
- Port assignment reference
- External access strategies

---

## Scaling & Deployment

See [06-Scaling.md](06-Scaling.md) for detailed information about:
- Scaling methods comparison
- Scaling running containers
- Helper script usage
- Incremental scaling
- Data persistence during scaling

---

## Sequential Startup

See [07-Sequential-Startup.md](07-Sequential-Startup.md) for detailed information about:
- How sequential startup works
- Wait script behavior
- Health checks and timeouts
- Enabling/disabling sequential startup
- Troubleshooting startup issues

---

## Deployment Scenarios & Examples

See [08-Deployment-Scenarios.md](08-Deployment-Scenarios.md) for detailed information about:
- Common deployment scenarios
- 1 Miner + 3 Webservers example
- Separate services approach
- Production deployment patterns
- Best practices

---

---

<div align="center">

**Local Navigation - Table of Contents**

| [← First Section: Introduction](#introduction--quick-start) | [↑ Table of Contents](#table-of-contents) | [Next Section: Architecture & Container System →](02-Architecture.md) |
|:---:|:---:|:---:|
| *Start of Chapter 7* | *Current Section* | *Section 2* |

</div>

---

## Additional Resources

- Docker Compose documentation: https://docs.docker.com/compose/
- Docker networking: https://docs.docker.com/network/
- Volume management: https://docs.docker.com/storage/volumes/
