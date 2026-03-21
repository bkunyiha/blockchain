<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../../bitcoin-blockchain/README.md">Chapter 2: Introduction to Blockchain</a>
3. <a href="../../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="../../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. <a href="../../bitcoin-blockchain/Rust-Project-Index.md">Chapter 5: Rust Blockchain Project</a>
6. <a href="../../bitcoin-blockchain/primitives/README.md">Chapter 6: Primitives</a>
7. <a href="../../bitcoin-blockchain/util/README.md">Chapter 7: Utilities</a>
8. <a href="../../bitcoin-blockchain/crypto/README.md">Chapter 8: Cryptography</a>
9. <a href="../../bitcoin-blockchain/chain/README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. <a href="../../bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 10: Block Acceptance</a>
11. <a href="../../bitcoin-blockchain/store/README.md">Chapter 11: Storage Layer</a>
12. <a href="../../bitcoin-blockchain/net/README.md">Chapter 12: Network Layer</a>
13. <a href="../../bitcoin-blockchain/node/README.md">Chapter 13: Node Orchestration</a>
14. <a href="../../bitcoin-blockchain/wallet/README.md">Chapter 14: Wallet System</a>
15. <a href="../../bitcoin-blockchain/web/README.md">Chapter 15: Web API Architecture</a>
16. <a href="../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 16: Desktop Admin (Iced)</a>
17. <a href="../../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">16A: Code Walkthrough</a>
18. <a href="../../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">16B: Update Loop</a>
19. <a href="../../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">16C: View Layer</a>
20. <a href="../../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 17: Desktop Admin (Tauri)</a>
21. <a href="../../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">17A: Rust Backend</a>
22. <a href="../../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">17B: Frontend Infrastructure</a>
23. <a href="../../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">17C: Frontend Pages</a>
24. <a href="../../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 18: Wallet UI (Iced)</a>
25. <a href="../../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">18A: Code Listings</a>
26. <a href="../../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 19: Wallet UI (Tauri)</a>
27. <a href="../../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">19A: Rust Backend</a>
28. <a href="../../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">19B: Frontend Infrastructure</a>
29. <a href="../../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">19C: Frontend Pages</a>
30. <a href="../../embedded-database/06-Embedded-Database.md">Chapter 20: Embedded Database</a>
31. <a href="../../embedded-database/06A-Embedded-Database-Code-Listings.md">20A: Code Listings</a>
32. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 21: Web Admin Interface</a>
33. <a href="../../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">21A: Code Listings</a>

### Part II: Deployment & Operations

34. **Chapter 22: Docker Compose Deployment** ← *You are here*
35. <a href="01A-Docker-Compose-Code-Listings.md">22A: Code Listings</a>
36. <a href="../kubernetes/README.md">Chapter 23: Kubernetes Deployment</a>
37. <a href="../kubernetes/01A-Kubernetes-Code-Listings.md">23A: Code Listings</a>

### Part III: Language Reference

38. <a href="../../rust/README.md">Chapter 24: Rust Language Guide</a>

</details>

</div>

---
<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

## Chapter 22: Docker Compose Deployment - Complete Guide

**Part II: Deployment & Operations**

<div align="center">

**[← Chapter 21: Web Admin UI](../../bitcoin-web-ui/06-Web-Admin-UI.md)** | **Chapter 22: Docker Compose** | **[Chapter 23: Kubernetes →](../kubernetes/README.md)**
</div>

---

> **Prerequisites:**: This chapter assumes basic familiarity with Docker (images, containers, volumes) and command-line usage. No prior Docker Compose experience is required — we explain the compose file format as we go. You should have completed the core implementation chapters (Part I) to understand what the services in each container are doing.

**Why deploy with containers?** Up to this point in the book, we have been running a single node from `cargo run`. But Bitcoin is a distributed system — you need multiple nodes to test peer discovery, block propagation, and chain synchronization. Docker Compose lets you spin up a configurable number of interconnected nodes with a single command, each with its own storage volume, while sharing a common network. This chapter bridges the gap between "the code compiles" and "the system actually works as a network."

> **What you will learn in this chapter:**
> - Deploy a multi-node blockchain network using Docker Compose
> - Configure container networking, port mapping, and volume management
> - Scale services and manage deployment topologies for development and testing
> - Troubleshoot common Docker Compose deployment issues

> **Scope:** This chapter covers local development and small-scale Docker Compose deployments. For production-grade orchestration with autoscaling and high availability, see Chapter 23 (Kubernetes).

**Important**: All commands in this guide should be run from the `configs/` directory where `docker-compose.yml` and the helper scripts are located.

```bash
cd ci/docker-compose/configs  # Navigate to configs directory first
```

## Table of Contents

**Learning Path - Read in Order:**

1. [Introduction & Quick Start](#introduction--quick-start) - *Start here*
2. [Docker Compose Code Listings (verbatim)](01A-Docker-Compose-Code-Listings.md) - *Source of truth for scripts + compose files*
3. [Architecture & Execution Flow](02-Architecture-and-Execution.md) - *Understand system structure and startup*
4. [Deployment Topology](03-Deployment-Topology.md) - *Network configuration, ports, scaling, and startup coordination*
5. [Deployment Scenarios & Operations](04-Deployment-Scenarios-and-Operations.md) - *Practical deployment patterns and operational procedures*
6. [Accessing Webserver](05-Accessing-Webserver.md) - *Use the web interface*
7. [DNS Resolution Mechanism](06-DNS-Resolution-Mechanism.md) - *Advanced networking internals*

---

## Introduction & Quick Start

### Overview

This chapter explains how to deploy and operate the blockchain network using Docker Compose. The deployment is intentionally **code-driven**: Compose describes the container topology, while the real orchestration logic (instance numbering, port selection, sequential startup, hostname resolution) lives in the **entrypoint and helper scripts**.

Every referenced artifact is printed in full in the companion listings chapter:

- **[Chapter 22A: Docker Compose — Complete Code Listings](01A-Docker-Compose-Code-Listings.md)**

**Figure 22-1: Docker Compose Deployment Topology**

```text
 ┌─────────────── Docker Network ──────────────────┐
 │                                                  │
 │  ┌──────────┐  ┌──────────┐  ┌──────────┐      │
 │  │  Node 1  │  │  Node 2  │  │  Node 3  │      │
 │  │ :8000    │  │ :8001    │  │ :8002    │      │
 │  │ ┌──────┐ │  │ ┌──────┐ │  │ ┌──────┐ │      │
 │  │ │ Web  │ │  │ │ Web  │ │  │ │ Web  │ │      │
 │  │ │ API  │ │  │ │ API  │ │  │ │ API  │ │      │
 │  │ ├──────┤ │  │ ├──────┤ │  │ ├──────┤ │      │
 │  │ │ Node │◀┼──┼▶│ Node │◀┼──┼▶│ Node │ │      │
 │  │ │ P2P  │ │  │ │ P2P  │ │  │ │ P2P  │ │      │
 │  │ ├──────┤ │  │ ├──────┤ │  │ ├──────┤ │      │
 │  │ │Store │ │  │ │Store │ │  │ │Store │ │      │
 │  │ └──────┘ │  │ └──────┘ │  │ └──────┘ │      │
 │  │  vol_1   │  │  vol_2   │  │  vol_3   │      │
 │  └──────────┘  └──────────┘  └──────────┘      │
 │                                                  │
 └──────────────────────────────────────────────────┘
       :8000          :8001          :8002
         │              │              │
    ─────┴──────────────┴──────────────┴───── Host
```

### Quick Start

The simplest deployment uses one miner and one webserver. Before starting, we need to set up wallet addresses. There are two methods:

> **Tip:** Always run `docker compose down` before changing configuration files. Applying changes to a running deployment can leave containers in an inconsistent state.

> **Warning:** Docker volumes persist data across `docker compose down` and `up` cycles. To start with a completely clean blockchain, use `docker compose down -v` to remove volumes, or `docker volume prune` to remove all unused volumes.

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

**A Note on Sequential Startup:** By default, sequential startup is enabled. This means each node waits for the previous node to be ready before starting. This ensures a stable network formation. We'll explore this mechanism in detail in [Section 3: Deployment Topology](03-Deployment-Topology.md).

For detailed information on accessing the webserver, see [Section 5: Accessing Webserver](05-Accessing-Webserver.md). For deployment instructions, see [Section 4: Deployment Scenarios & Operations](04-Deployment-Scenarios-and-Operations.md).

We'll explore wallet address distribution in more detail in the [Wallet Address Distribution](#wallet-address-distribution) section below.

#### Scaling to Multiple Instances

Once you are comfortable with the basic setup, you will likely want to scale to multiple instances. Before scaling, we need to set up wallet addresses. We use `WALLET_ADDRESS_POOL` with enough addresses for all our nodes:

```bash
cd ci/docker-compose/configs
# Set up address pool (need at least 5 addresses for 3 miners + 2 webservers)
export WALLET_ADDRESS_POOL="addr1,addr2,addr3,addr4,addr5"
# Scale to 3 miners and 2 webservers
./docker-compose.scale.sh 3 2
# miner_1 → addr1, miner_2 → addr2, miner_3 → addr3
# webserver_1 → addr4, webserver_2 → addr5
```

**Important:** When using `--scale` directly with `docker compose up -d --scale`, only the first instance gets ports mapped. To ensure **ALL instances have ports accessible externally**, always use the helper script `./docker-compose.scale.sh`.

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

**Note:** We don't automatically invoke these files. Instead, we use them as manual alternatives with the `-f` flag to run only miners or only webservers. The main `docker-compose.yml` is recommended for most use cases since it supports scaling both services together.

Full listings:

- `docker-compose.miner.yml`: [Listing 22A.8](01A-Docker-Compose-Code-Listings.md#listing-22a8-cidocker-composeconfigsdocker-composemineryml)
- `docker-compose.webserver.yml`: [Listing 22A.9](01A-Docker-Compose-Code-Listings.md#listing-22a9-cidocker-composeconfigsdocker-composewebserveryml)

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

For detailed information about port mapping and external access, see [Port Mapping & External Access](#port-mapping--external-access) and [Section 3: Deployment Topology](03-Deployment-Topology.md).

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
docker exec -it <container_name> test -f /app/wallets/wallets.dat \
  && echo "Wallet file exists"
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
# ~/Library/Containers/com.docker.docker/Data/vms/0/data/docker/volumes/
# <volume_name>/_data

# However, accessing files directly requires navigating the VM disk image
# Easier methods: use docker exec or docker cp commands
```

**Windows (Docker Desktop):**
```bash
# Similar to macOS, volumes are stored on local Windows storage
# inside a Linux VM
# Use docker volume inspect to get the path inside the VM
docker volume inspect blockchain_miner-wallets

# To access from host, use docker exec or docker cp commands
```

**Finding the Exact Path:**

```bash
# Get the mountpoint (path inside Docker VM)
docker volume inspect blockchain_miner-wallets \
  --format '{{ .Mountpoint }}'

# On Linux (direct host access):
# Full wallet file path:
MOUNT_PATH=$(docker volume inspect blockchain_miner-wallets \
  --format '{{ .Mountpoint }}')
$MOUNT_PATH/wallets.dat

# Example: List files in volume on Linux
MOUNT_PATH=$(docker volume inspect blockchain_miner-wallets \
  --format '{{ .Mountpoint }}')
sudo ls -la $MOUNT_PATH

# On macOS (VM filesystem stored locally):
# VM path: /var/lib/docker/volumes/blockchain_miner-wallets/_data
# macOS storage location:
# ~/Library/Containers/com.docker.docker/Data/vms/0/data/docker/volumes/
# blockchain_miner-wallets/_data
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

We export the pool variables and pass them to `docker compose`:

```bash
export MINER_ADDRESS_POOL="miner-addr1,miner-addr2,miner-addr3"
export WEBSERVER_ADDRESS_POOL="webserver-addr1,webserver-addr2"
docker compose up -d --scale miner=3 --scale webserver=2
```


### Basic Examples

#### Example 1: Single Miner and Webserver (Address Pool)

```bash
# Set up address pool with 2 addresses
export WALLET_ADDRESS_POOL="\
3npBNyKSEwhCQWTXHFjwR8Rb66kjq6khfZSdmLPm8Gde9XoTwW,\
1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"

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
docker compose logs -f miner
docker compose logs -f webserver
```

#### Check Health

```bash
curl http://localhost:8080/api/health/liveness
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

```bash
docker compose ps
netstat -tulpn | grep -E '2001|8080'
```

#### Data Persistence

```bash
docker volume ls
docker volume inspect blockchain_miner-data
```

---

## Architecture & Execution Flow

See [Section 2: Architecture & Execution Flow](02-Architecture-and-Execution.md) for detailed information about:
- Container naming and identification
- Instance number detection and service type identification
- Complete execution timeline and startup sequence
- Docker Compose initialization and health checks
- Entrypoint script execution and Rust binary startup

---

## Deployment Topology

See [Section 3: Deployment Topology](03-Deployment-Topology.md) for detailed information about:
- Network configuration and node connections
- Miner connection chains and webserver networking
- Port mapping and external access strategies
- Scaling methods and incremental scaling
- Sequential startup coordination and health checks

---

## Deployment Scenarios & Operations

See [Section 4: Deployment Scenarios & Operations](04-Deployment-Scenarios-and-Operations.md) for detailed information about:
- Common deployment scenarios and patterns
- Production deployment procedures
- Operational guidance and troubleshooting
- Detailed deployment execution walkthrough with examples
- Best practices and complete workflows

---

<div align="center">

**Local Navigation - Table of Contents**

| [← First Section: Introduction](#introduction--quick-start) | [↑ Table of Contents](#table-of-contents) | [Next Section: Architecture & Execution Flow →](02-Architecture-and-Execution.md) |
|:---:|:---:|:---:|
| *Start of Chapter 22* | *Current Section* | *Section 2* |

</div>

---

## Additional Resources

- [Docker Compose documentation](https://docs.docker.com/compose/)
- [Docker networking](https://docs.docker.com/network/)
- [Volume management](https://docs.docker.com/storage/volumes/)

---

## Summary

- We deployed a multi-node blockchain network using Docker Compose.
- We configured container networking, port mapping, and volume management.
- We scaled services and managed deployment topologies for development and testing.
- We troubleshooted common Docker Compose deployment issues.

> **Companion Chapter:** Complete Docker Compose configuration files and deployment scripts are available in [22A: Code Listings](01A-Docker-Compose-Code-Listings.md). In the print edition, these listings appear in the Appendix: Source Reference.

---

## Exercises

1. **Scale to Five Nodes** — Modify the Docker Compose configuration to deploy 5 blockchain nodes instead of 3. Observe how the new nodes discover peers and synchronize the chain. Measure the time from startup to full synchronization.

2. **Volume Persistence Test** — Submit several transactions, mine a few blocks, then run `docker compose down` followed by `docker compose up`. Verify that the blockchain data persisted across the restart. Then run `docker compose down -v` and observe the difference.

---

> **Checkpoint:** With Docker Compose running, you now have a fully operational multi-node blockchain network. Verify it by running `docker compose ps` to confirm all containers are healthy, then `curl http://localhost:8080/api/admin/blockchain-info` to see the chain height increasing as blocks are mined. Open `http://localhost:8080` in your browser to see the Web Admin dashboard in action. If the chain height is stuck at 0, check the miner container logs with `docker compose logs miner_1` for error messages.

---

### Further Reading

- **Multi-stage Docker builds** — Our Dockerfile uses a multi-stage build to keep the final image small (compile in a `rust:latest` stage, copy the binary into a minimal `debian:slim` stage). The Docker documentation on multi-stage builds explains the pattern in depth.
- **Docker Compose profiles** — For more complex deployments (e.g., optional monitoring containers), Compose profiles let you define groups of services that are only started on demand.
- **Container health checks** — Adding `healthcheck` directives to `docker-compose.yml` lets Compose wait for a service to be ready before starting dependent services, replacing the `wait-for-node.sh` script.

---

<div align="center">

**Reading order**

**[← Previous: Web Admin Interface](../../bitcoin-web-ui/06-Web-Admin-UI.md)** | **[Next: Docker Compose — Code Listings →](01A-Docker-Compose-Code-Listings.md)**

</div>

---
