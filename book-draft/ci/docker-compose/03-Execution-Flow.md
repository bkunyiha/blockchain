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
   - **Section 3: Execution Flow & Startup Process** ← *You are here*
   - [Section 4: Network Configuration](04-Network-Configuration.md)
   - [Section 5: Sequential Startup](07-Sequential-Startup.md)
   - [Section 6: Port Mapping](05-Port-Mapping.md)
   - [Section 7: Scaling](06-Scaling.md)
   - [Section 8: Deployment Scenarios](08-Deployment-Scenarios.md)
   - [Section 9: Accessing Webserver](09-Accessing-Webserver.md)
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

# Chapter 7, Section 3: Execution Flow & Startup Process

**Part II: Deployment & Operations** | **Chapter 8: Docker Compose Deployment**

<div align="center">

**📚 [← Section 2: Architecture](02-Architecture.md)** | **Section 3: Execution Flow** | **[Section 4: Network Configuration →](04-Network-Configuration.md)** 📚

</div>

---

## Prerequisites

Before reading this section, you should have:
- Completed [Section 2: Architecture & Container System](02-Architecture.md)
- Understanding of Docker Compose service definitions
- Basic knowledge of shell scripting (helpful but not required)

## Learning Objectives

After reading this section, you will understand:
- The complete execution timeline from Docker Compose initialization to blockchain node startup
- How entrypoint scripts orchestrate container startup
- The role of health checks and dependencies
- How the multi-stage Docker build process works

---

This section provides a detailed walkthrough of the complete execution flow when starting containers, from Docker Compose initialization through to the blockchain nodes running.

## Code Execution Order Overview

```
1. docker-compose.yml (Docker Compose reads configuration)
   ↓
2. Dockerfile (Multi-stage build)
   ├─ Stage 1: rust-builder (Builds Rust blockchain binary)
   ├─ Stage 2: web-ui-builder (Builds React web UI)
   └─ Stage 3: Runtime (Combines binary + web UI)
   ↓
3. docker-entrypoint.sh (Container startup script)
   ├─ wait-for-node.sh (if sequential startup, instance > 1)
   ↓
4. bitcoin/src/main.rs (Rust binary entry point)
   ├─ bitcoin/src/config.rs (Configuration loading)
   ├─ bitcoin/src/lib.rs (Library initialization)
   ├─ bitcoin/src/node/context.rs (NodeContext creation)
   ├─ bitcoin/src/node/server.rs (P2P network server)
   └─ bitcoin/src/web/server.rs (Web server, if enabled)
      ├─ bitcoin/src/web/routes/*.rs (Route definitions)
      │  └─ bitcoin/src/web/routes/web.rs (Serves React app from /app/bitcoin-web-ui/dist)
      ├─ bitcoin/src/web/handlers/*.rs (Request handlers)
      └─ bitcoin/src/web/middleware/*.rs (Middleware)
```

## Phase 1: Docker Compose Initialization

### Step 1.1: Parse docker-compose.yml

**File**: `docker-compose.yml`

Docker Compose reads the configuration and creates two services:

1. **`miner` service**:
   - Port mapping: `2001:2001` (host:container)
   - Volumes: `miner-data:/app/data`, `miner-wallets:/app/wallets`
   - Environment variables set:
     - `NODE_IS_MINER=yes`
     - `NODE_IS_WEB_SERVER=no`
     - `NODE_CONNECT_NODES=local` (default)
     - `SEQUENTIAL_STARTUP=yes` (default)
     - `WALLET_ADDRESS_POOL=<comma-separated-addresses>` (Option 1: auto-select by instance number)
     - `NODE_MINING_ADDRESS=<wallet-address>` (Option 2: direct assignment, at least one must be set)
     - `WALLET_FILE=wallets/wallets.dat`

2. **`webserver` service**:
   - Port mappings: `8080:8080`, `2101:2001`
   - Volumes: `webserver-data:/app/data`, `webserver-wallets:/app/wallets`
   - **Dependency**: `depends_on: miner: condition: service_healthy`
   - Environment variables set:
     - `NODE_IS_MINER=no`
     - `NODE_IS_WEB_SERVER=yes`
     - `NODE_CONNECT_NODES=miner_1:2001` (default - connects to first miner)
     - `SEQUENTIAL_STARTUP=yes` (default)
     - `BITCOIN_API_ADMIN_KEY=admin-secret`
     - `BITCOIN_API_WALLET_KEY=wallet-secret`
     - `WALLET_FILE=wallets/wallets.dat`

### Step 1.2: Container Creation

Docker Compose creates containers:
- `blockchain_miner_1` (or `<project>_miner_1`)
- `blockchain_webserver_1` (or `<project>_webserver_1`)

**Note**: Webserver depends on miner (`depends_on: miner: condition: service_healthy`), so:
1. Miner starts first
2. Miner health check passes (port 2001 listening)
3. Webserver starts after miner is healthy

## Phase 2: Miner Container Startup

### Step 2.1: Container Initialization

**Container**: `blockchain_miner_1`

1. Docker mounts volumes:
   - `miner-data` → `/app/data`
   - `miner-wallets` → `/app/wallets`

2. Docker sets environment variables from `docker-compose.yml`

3. Docker sets `HOSTNAME` environment variable to container name: `blockchain_miner_1`

### Step 2.2: Entrypoint Script Execution

**File**: `docker-entrypoint.sh` (line 1)

**Line 1-12**: Set default values from environment
```bash
NODE_IS_MINER="yes"           # From docker-compose.yml
NODE_IS_WEB_SERVER="no"       # From docker-compose.yml
NODE_CONNECT_NODES="local"    # From docker-compose.yml (default)
NODE_MINING_ADDRESS="<wallet-address>"  # REQUIRED: Must be set
```

**Line 36**: Get container name
```bash
CONTAINER_NAME="blockchain_miner_1"  # From HOSTNAME
```

**Line 37-48**: Determine instance number
```bash
# Pattern match: blockchain_miner_1 matches _([0-9]+)$
INSTANCE_NUMBER=1  # Extracted from container name
```

**Line 52-57**: Determine service name from container
```bash
# Pattern match: "blockchain_miner_1" contains "miner"
SERVICE_NAME_FROM_CONTAINER="miner"
```

**Line 61-64**: Determine service type and calculate ports
```bash
SERVICE_TYPE="miner"  # From container name pattern match
P2P_PORT=$((2001 + 1 - 1))  # = 2001
```

**Line 84-85**: Set data directory names
```bash
DATA_DIR="data1"
BLOCKS_TREE="blocks1"
```

**Line 82-110**: Set up isolated data directory
```bash
# Base directory where volume is mounted
BASE_DATA_DIR="/app/data"
INSTANCE_DATA_DIR_NAME="data1"
INSTANCE_DATA_DIR="/app/data/data1"

# Export environment variables
export NODE_ADDR="0.0.0.0:2001"
export TREE_DIR="data/data1"  # Relative to /app, stored within volume
export BLOCKS_TREE="blocks1"

# Create isolated data directory
mkdir -p "/app/data/data1"  # Creates isolated directory within volume
echo "Using isolated blockchain data directory: /app/data/data1"
```

**Line 114-123**: Auto-configure webserver connection (not applicable for miner, skipped)

**Line 126**: Check sequential startup condition
```bash
# SEQUENTIAL_STARTUP=yes (from env)
# INSTANCE_NUMBER=1
# Condition: [ "yes" = "yes" ] && [ 1 -gt 1 ]
# Result: false (INSTANCE_NUMBER is 1, not > 1)
# SKIP wait script (first instance doesn't wait)
```

**Line 238-260**: Determine wallet address from pool or direct assignment
```bash
# Option 1: Use WALLET_ADDRESS_POOL (comma-separated list)
if [ -n "${WALLET_ADDRESS_POOL}" ]; then
    # Convert to array and select by instance number
    IFS=',' read -ra ADDRESSES <<< "${WALLET_ADDRESS_POOL}"
    INDEX=$((INSTANCE_NUMBER - 1))
    NODE_MINING_ADDRESS="${ADDRESSES[${INDEX}]}"
    # Instance 1 → index 0, Instance 2 → index 1, etc.
fi

# Option 2: Use NODE_MINING_ADDRESS directly
# If neither is set, exit with error
```

**Line 262-264**: Validate required environment variables
```bash
# NODE_MINING_ADDRESS must be set (either from pool or direct)
if [ -z "${NODE_MINING_ADDRESS}" ]; then
    echo "ERROR: Either WALLET_ADDRESS_POOL or NODE_MINING_ADDRESS must be set"
    exit 1
fi
```

**Line 266-267**: Build command
```bash
CMD="./blockchain startnode yes no local -- <wallet-address>"
# Format: startnode <is_miner> <is_web_server> <connect_nodes> -- <mining_address>
# Note: Wallet address is required (selected from pool or direct assignment)
```

**Line 212-229**: Log configuration
```
==========================================
Starting blockchain node
  Service Type: miner
  Instance Number: 1
  Container Name: blockchain_miner_1
  Mode: miner=yes, webserver=no
  P2P Port: 2001
  Data Directory: /app/data/data1 (isolated per instance)
  TREE_DIR: data/data1
  Connect Nodes: local
==========================================
```

**Line 232**: Execute blockchain binary
```bash
exec ./blockchain startnode yes no local
```

### Step 2.3: Blockchain Binary Execution

**File**: `bitcoin/src/main.rs`

The Rust binary receives arguments: `["startnode", "yes", "no", "local"]`

**Line 349-357**: Parse command
```rust
Command::StartNode {
    is_miner: IsMiner::Yes,
    is_web_server: IsWebServer::No,
    connect_nodes: vec![ConnectNode::Local],
    wlt_mining_addr: "3npBNyKSEwhCQWTXHFjwR8Rb66kjq6khfZSdmLPm8Gde9XoTwW".to_string(),  // REQUIRED
}
```

**Line 240-250**: Start node function
- Opens or creates blockchain
- Creates NodeContext
- Gets node address from config (0.0.0.0:2001)

**Line 265-273**: Start network server
- Binds to 0.0.0.0:2001
- Starts P2P network server
- Since `connect_nodes` contains `Local`, it acts as seed node

**Line 280-337**: Since `is_web_server = No` and `is_miner = Yes`:
- Skips web server startup
- Runs only network server
- Waits for Ctrl+C or shutdown signal

**Miner is now running** on P2P port 2001, ready to accept connections.

## Phase 3: Webserver Container Startup

### Step 3.1: Container Initialization

**Container**: `blockchain_webserver_1`

1. Docker mounts volumes:
   - `webserver-data` → `/app/data`
   - `webserver-wallets` → `/app/wallets`

2. Docker sets environment variables from `docker-compose.yml`

3. Docker sets `HOSTNAME` to: `blockchain_webserver_1`

### Step 3.2: Entrypoint Script Execution

**File**: `docker-entrypoint.sh`

**Line 1-12**: Set default values
```bash
NODE_IS_MINER="no"                  # From docker-compose.yml
NODE_IS_WEB_SERVER="yes"           # From docker-compose.yml
NODE_CONNECT_NODES="miner_1:2001"  # From docker-compose.yml (default)
NODE_MINING_ADDRESS="<wallet-address>"  # REQUIRED: Must be set
```

**Line 36**: Get container name
```bash
CONTAINER_NAME="blockchain_webserver_1"
```

**Line 37-48**: Determine instance number
```bash
INSTANCE_NUMBER=1  # Extracted from "blockchain_webserver_1"
```

**Line 52-57**: Determine service name
```bash
SERVICE_NAME_FROM_CONTAINER="webserver"  # Contains "webserver"
```

**Line 66-69**: Determine service type and ports
```bash
SERVICE_TYPE="webserver"
WEB_PORT=$((8080 + 1 - 1))   # = 8080
P2P_PORT=$((2101 + 1 - 1))  # = 2101
```

**Line 84-85**: Set data directory
```bash
DATA_DIR="data1"
BLOCKS_TREE="blocks1"
```

**Line 82-110**: Set up isolated data directory
```bash
# Base directory where volume is mounted
BASE_DATA_DIR="/app/data"
INSTANCE_DATA_DIR_NAME="data1"
INSTANCE_DATA_DIR="/app/data/data1"

# Export environment variables
export NODE_ADDR="0.0.0.0:2101"  # Note: Uses 2101, not 2001
export TREE_DIR="data/data1"  # Relative to /app, stored within volume
export BLOCKS_TREE="blocks1"

# Create isolated data directory
mkdir -p "/app/data/data1"  # Creates isolated directory within volume
echo "Using isolated blockchain data directory: /app/data/data1"
```

**Line 114-123**: Auto-configure webserver to connect to miner
```bash
# NODE_IS_WEB_SERVER=yes, NODE_IS_MINER=no
# INSTANCE_NUMBER=1
# NODE_CONNECT_NODES="miner_1:2001" (from docker-compose.yml default)
# Since INSTANCE_NUMBER=1, sets NODE_CONNECT_NODES="miner_1:2001"
# This ensures webserver connects to miner, not acts as seed node
```

**Line 126**: Check sequential startup
```bash
# SEQUENTIAL_STARTUP=yes
# INSTANCE_NUMBER=1
# Condition: [ "yes" = "yes" ] && [ 1 -gt 1 ]
# Result: false (first instance, no wait needed)
# SKIP wait script
```

**Line 205**: Build command
```bash
CMD="./blockchain startnode no yes miner_1:2001 -- <wallet-address>"
# Note: NODE_CONNECT_NODES is now "miner_1:2001" instead of "local"
# Note: Wallet address is now required for all nodes
```

**Line 212-229**: Log configuration
```
==========================================
Starting blockchain node
  Service Type: webserver
  Instance Number: 1
  Container Name: blockchain_webserver_1
  Mode: miner=no, webserver=yes
  P2P Port: 2101
  Web Port: 8080
  Data Directory: /app/data/data1 (isolated per instance)
  TREE_DIR: data/data1
  Connect Nodes: miner_1:2001
==========================================
```

**Line 232**: Execute blockchain binary
```bash
exec ./blockchain startnode no yes miner_1:2001
```

### Step 3.3: Blockchain Binary Execution

**File**: `bitcoin/src/main.rs`

**Line 349-357**: Parse command
```rust
Command::StartNode {
    is_miner: IsMiner::No,
    is_web_server: IsWebServer::Yes,
    connect_nodes: vec![ConnectNode::Remote("miner_1:2001".parse().unwrap())],
    wlt_mining_addr: "3npBNyKSEwhCQWTXHFjwR8Rb66kjq6khfZSdmLPm8Gde9XoTwW".to_string(),  // REQUIRED
}
```

**Line 240-250**: Start node
- Opens or creates blockchain
- Creates NodeContext
- Gets node address: 0.0.0.0:2101 (from NODE_ADDR env var)

**Line 265-273**: Start network server
- Binds to 0.0.0.0:2101
- Starts P2P server
- Connects to `miner_1:2001` (from connect_nodes parameter)

**Line 280-313**: Since `is_web_server = Yes` and `is_miner = No`:
- Creates web server with NodeContext
- Starts web server on port 8080 (from WebServerConfig default)
- Runs both network server and web server concurrently
- Webserver is connected to miner via P2P network

**Webserver is now running**:
- Web API on port 8080
- P2P network on port 2101
- Connected to miner at `miner_1:2001` via P2P network

## Phase 4: Health Checks (Background)

Docker Compose starts health checks after containers start:

**Note**: The miner health check must pass before the webserver container starts (due to `depends_on: condition: service_healthy`).

### Miner Health Check
**Line 39-46** in `docker-compose.yml`:
```bash
# Every 10 seconds, check if port 2001 is listening
timeout 1 bash -c 'echo > /dev/tcp/localhost/2001'
```

### Webserver Health Check
**Line 81-86** in `docker-compose.yml`:
```bash
# Every 10 seconds, check HTTP health endpoint
curl -f http://localhost:8080/api/health/ready
```

## Execution Timeline

```
Time 0ms:   docker-compose.yml read
Time 100ms: Dockerfile image built (if needed)
            ├─ Stage 1: Rust binary built
            ├─ Stage 2: React web UI built (npm install + npm run build)
            └─ Stage 3: Runtime image created (binary + web UI copied)
Time 200ms: Miner container starts
Time 201ms: docker-entrypoint.sh executes (miner)
Time 202ms: bitcoin/src/main.rs::main() called (miner)
Time 203ms: Blockchain initialized (miner)
Time 204ms: P2P server starts (miner, port 2001)
Time 5000ms: Miner health check passes
Time 5001ms: Webserver container starts
Time 5002ms: docker-entrypoint.sh executes (webserver)
Time 5003ms: bitcoin/src/main.rs::main() called (webserver)
Time 5004ms: Blockchain initialized (webserver)
Time 5005ms: P2P server starts (webserver, port 2101)
Time 5006ms: Web server starts (webserver, port 8080)
Time 5007ms: Both containers running
```

## Key File Responsibilities

| File | Responsibility | When Executed |
|------|---------------|---------------|
| `docker-compose.yml` | Service configuration | Before container creation |
| `Dockerfile` | Container image build (multi-stage: Rust + React) | Build time |
| `docker-entrypoint.sh` | Container startup logic | Container start |
| `wait-for-node.sh` | Sequential startup wait | Conditional (instance > 1) |
| `bitcoin/src/main.rs` | Application entry point | Container start |
| `bitcoin/src/config.rs` | Configuration management | Application initialization |
| `bitcoin/src/node/server.rs` | P2P network server | Runtime (continuous) |
| `bitcoin/src/web/server.rs` | HTTP web server | Runtime (continuous, webserver only) |
| `bitcoin/src/web/routes/web.rs` | Serves React web UI from `/app/bitcoin-web-ui/dist` | Runtime (webserver only) |
| `bitcoin/src/web/handlers/*.rs` | HTTP request handlers | On HTTP request |
| `bitcoin/src/store/file_system_db_chain.rs` | Blockchain storage | On blockchain operations |

## Additional Instances

When scaling to multiple instances, the execution flow is similar but with these differences:

### Additional Miners (Instance 2+)

1. **Sequential startup enabled**: Waits for previous miner
2. **NODE_CONNECT_NODES**: Auto-configured to previous miner (not "local")
3. **Port calculation**: Uses instance number (2002, 2003, etc.)
4. **Data directory**: Uses instance-specific directory (data2, data3, etc.)

### Additional Webservers (Instance 2+)

1. **Sequential startup enabled**: Waits for miner_1 (not previous webserver)
2. **NODE_CONNECT_NODES**: Always connects to miner_1:2001
3. **Port calculation**: Uses instance number (8081/2102, 8082/2103, etc.)
4. **Data directory**: Uses instance-specific directory (data2, data3, etc.)

For detailed information about how additional instances connect, see [Chapter 4: Network Configuration & Node Connections](04-Network-Configuration.md).

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Architecture](02-Architecture.md) | [↑ Table of Contents](#) | [Next Section: Network Configuration →](04-Network-Configuration.md) |
|:---:|:---:|:---:|
| *Section 2* | *Current Section* | *Section 4* |

</div>

---
