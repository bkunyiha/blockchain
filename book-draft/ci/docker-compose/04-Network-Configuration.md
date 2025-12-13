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
   - **Section 4: Network Configuration & Node Connections** ← *You are here*
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

# Chapter 7, Section 4: Network Configuration & Node Connections

**Part II: Deployment & Operations** | **Chapter 8: Docker Compose Deployment**

<div align="center">

**📚 [← Section 3: Execution Flow](03-Execution-Flow.md)** | **Section 4: Network Configuration** | **[Section 5: Sequential Startup →](07-Sequential-Startup.md)** 📚

</div>

---

## Prerequisites

Before reading this section, you should have:
- Completed [Section 3: Execution Flow & Startup Process](03-Execution-Flow.md)
- Understanding of Docker networking basics
- Familiarity with TCP/IP networking concepts

## Learning Objectives

After reading this section, you will understand:
- How nodes discover and connect to each other
- The miner connection chain mechanism
- Webserver connection behavior and configuration
- Network topology and node relationships

---

This section explains how nodes connect to each other, the network topology, and how `NODE_CONNECT_NODES` is configured for different scenarios.

## Overview

The blockchain network uses a peer-to-peer (P2P) architecture where:
- **Miners** form a chain, with each miner connecting to the previous one
- **Webservers** always connect to miners (specifically `miner_1:2001`), never to other webservers
- The first miner acts as a **seed node** using `NODE_CONNECT_NODES=local`

## Miner Connection Chain

### Default Behavior

When scaling miners, they form a sequential chain:

- **Miner 1**: `NODE_CONNECT_NODES=local` (acts as seed node)
- **Miner 2**: `NODE_CONNECT_NODES=miner_1:2001` (connects to Miner 1)
- **Miner 3**: `NODE_CONNECT_NODES=miner_2:2002` (connects to Miner 2)
- **Miner 4**: `NODE_CONNECT_NODES=miner_3:2003` (connects to Miner 3)
- And so on...

### How Miner Connections Are Configured

#### Miner Instance 1 (Seed Node)

**Configuration**:
- `NODE_CONNECT_NODES="local"` (from docker compose.yml default)
- `INSTANCE_NUMBER=1`
- Sequential startup skipped (first instance)

**Entrypoint Logic**:
```bash
# Line 126: Sequential startup check
# INSTANCE_NUMBER=1, so condition is false
# NODE_CONNECT_NODES remains "local"
```

**Result**: Miner 1 acts as seed node, creates genesis block if needed.

#### Miner Instance 2+

**Configuration**:
- `NODE_CONNECT_NODES="local"` (from docker compose.yml default, will be replaced)
- `INSTANCE_NUMBER > 1`
- Sequential startup enabled

**Entrypoint Logic**:

**Line 126**: Check sequential startup condition
```bash
# SEQUENTIAL_STARTUP=yes (default)
# INSTANCE_NUMBER=2
# Condition: [ "yes" = "yes" ] && [ 2 -gt 1 ]
# Result: true (enters sequential startup block)
```

**Line 132-137**: Determine wait service name
```bash
if [ "${NODE_IS_MINER}" = "yes" ]; then
    WAIT_SERVICE_NAME="miner"
fi
# For miners: WAIT_SERVICE_NAME="miner"
```

**Line 141**: Call wait script
```bash
/app/wait-for-node.sh "miner" "2" "2002" "no"
# Example for miner instance 2
```

**wait-for-node.sh execution**:
```bash
# Calculates PREV_INSTANCE = 2 - 1 = 1
# Iterates backwards: Checks miner_1:2001
# Waits for miner_1:2001 to be ready (up to 60 attempts, 2 minutes)
# When ready: Outputs PREV_NODE_ADDRESS=miner_1:2001 and exits 0
```

**Line 149-160**: Extract/construct previous node address
```bash
PREV_ADDR=$(echo "${WAIT_OUTPUT}" | grep "PREV_NODE_ADDRESS=" | cut -d'=' -f2)
# PREV_ADDR="miner_1:2001"
```

**Line 168-170**: Update NODE_CONNECT_NODES
```bash
if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ]; then
    NODE_CONNECT_NODES="${PREV_ADDR}"  # Replace "local" with previous node
fi
# NODE_CONNECT_NODES="miner_1:2001"
```

**Result**: Miner 2 connects to Miner 1, Miner 3 connects to Miner 2, etc.

### Example: 3 Miners

**Miner 1** (`blockchain_miner_1`):
- `NODE_CONNECT_NODES="local"`
- P2P Port: 2001
- Command: `./blockchain startnode yes no local`
- Acts as seed node

**Miner 2** (`blockchain_miner_2`):
- Waits for Miner 1
- `NODE_CONNECT_NODES="miner_1:2001"` (auto-configured)
- P2P Port: 2002
- Command: `./blockchain startnode yes no miner_1:2001`

**Miner 3** (`blockchain_miner_3`):
- Waits for Miner 2
- `NODE_CONNECT_NODES="miner_2:2002"` (auto-configured)
- P2P Port: 2003
- Command: `./blockchain startnode yes no miner_2:2002`

### Override Behavior

The entrypoint script only replaces `NODE_CONNECT_NODES` if it's:
- Empty (`-z "${NODE_CONNECT_NODES}"`)
- Set to `"local"` (`"${NODE_CONNECT_NODES}" = "local"`)

If you explicitly set `NODE_CONNECT_NODES` to a different value, it will be preserved:

```bash
# All miners connect to miner_1:2001
MINER_CONNECT_NODES=miner_1:2001 docker compose up -d --scale miner=3
```

In this case:
- Miner 1: `NODE_CONNECT_NODES="miner_1:2001"` (preserved, not changed to "local")
- Miner 2: `NODE_CONNECT_NODES="miner_1:2001"` (preserved, sequential startup won't override)
- Miner 3: `NODE_CONNECT_NODES="miner_1:2001"` (preserved)

## Webserver Connection Behavior

**Important**: Webservers **always** connect to miners, never to other webservers.

### Default Behavior

All webservers connect to the first miner (`miner_1:2001`):

- **Webserver 1**: `NODE_CONNECT_NODES=miner_1:2001`
- **Webserver 2**: `NODE_CONNECT_NODES=miner_1:2001`
- **Webserver 3**: `NODE_CONNECT_NODES=miner_1:2001`
- All webservers connect to the same miner

### How Webserver Connections Are Configured

#### Webserver Instance 1

**Configuration**:
- `NODE_CONNECT_NODES="miner_1:2001"` (from docker compose.yml default)
- `INSTANCE_NUMBER=1`
- Sequential startup skipped (first instance)

**Entrypoint Logic**:

**Line 114-123**: Auto-configuration for webserver instance 1
```bash
# NODE_IS_WEB_SERVER=yes, NODE_IS_MINER=no
# INSTANCE_NUMBER=1
# Sets NODE_CONNECT_NODES="miner_1:2001"
```

**Result**: Webserver 1 connects to Miner 1.

#### Webserver Instance 2+

**Configuration**:
- `NODE_CONNECT_NODES="miner_1:2001"` (from docker compose.yml default)
- `INSTANCE_NUMBER > 1`
- Sequential startup enabled

**Entrypoint Logic**:

**Line 114-123**: Auto-configuration skipped (`INSTANCE_NUMBER > 1`)

**Line 126**: Sequential startup check
```bash
# SEQUENTIAL_STARTUP=yes
# INSTANCE_NUMBER=2
# Condition: [ "yes" = "yes" ] && [ 2 -gt 1 ]
# Result: true (enters sequential startup block)
```

**Line 132-137**: Determine wait service name
```bash
if [ "${NODE_IS_MINER}" = "yes" ]; then
    WAIT_SERVICE_NAME="miner"
else
    # Webservers always connect to miners, not other webservers
    WAIT_SERVICE_NAME="miner"  # Always "miner" for webservers
fi
```

**Line 141**: Call wait script
```bash
/app/wait-for-node.sh "miner" "2" "2102" "yes"
# For webserver instance 2, waits for miner_1:2001
```

**wait-for-node.sh execution**:
```bash
# For webserver instance 2:
#   - CHECK_INSTANCE = PREV_INSTANCE (1)
#   - CHECK_HOSTNAME="miner_1"
#   - CHECK_PORT=2001
#   - Wait for miner_1:2001 to be ready (up to 60 attempts)
#   - When ready: Outputs PREV_NODE_ADDRESS=miner_1:2001 and exits 0
# Note: Script iterates backwards, so if miner_2 doesn't exist, it tries miner_1
```

**Line 175-178**: Set NODE_CONNECT_NODES for webservers
```bash
else
    # Webservers always connect to first miner (miner_1:2001)
    NODE_CONNECT_NODES="miner_1:2001"
    echo "  Auto-configured webserver to connect to first miner: ${NODE_CONNECT_NODES}"
fi
```

**Result**: All webservers connect to `miner_1:2001`, regardless of instance number.

### Example: 1 Miner + 3 Webservers

**Miner 1** (`blockchain_miner_1`):
- `NODE_CONNECT_NODES="local"`
- P2P Port: 2001
- Acts as seed node

**Webserver 1** (`blockchain_webserver_1`):
- `NODE_CONNECT_NODES="miner_1:2001"` (auto-configured)
- Web Port: 8080, P2P Port: 2101
- Connects to Miner 1

**Webserver 2** (`blockchain_webserver_2`):
- Waits for `miner_1:2001` (not webserver_1)
- `NODE_CONNECT_NODES="miner_1:2001"` (always first miner)
- Web Port: 8081, P2P Port: 2102
- Connects to Miner 1

**Webserver 3** (`blockchain_webserver_3`):
- Waits for `miner_1:2001` (not webserver_2)
- `NODE_CONNECT_NODES="miner_1:2001"` (always first miner)
- Web Port: 8082, P2P Port: 2103
- Connects to Miner 1

## Network Topology

### Single Miner + Multiple Webservers (Star Topology)

```
miner_1:2001 (seed node, "local")
    ↑
    ├── webserver_1:2101 → connects to miner_1:2001
    ├── webserver_2:2102 → connects to miner_1:2001
    └── webserver_3:2103 → connects to miner_1:2001
```

All webservers form a star topology around the single miner.

### Multiple Miners (Chain Topology)

```
miner_1:2001 (seed, "local")
    ↑
miner_2:2002 → connects to miner_1:2001
    ↑
miner_3:2003 → connects to miner_2:2002
```

Miners form a chain, with each miner connecting to the previous one.

### Multiple Miners + Multiple Webservers (Hybrid Topology)

```
miner_1:2001 (seed, "local")
    ↑
    ├── miner_2:2002 → connects to miner_1:2001
    │       ↑
    │   miner_3:2003 → connects to miner_2:2002
    │
    ├── webserver_1:2101 → connects to miner_1:2001
    ├── webserver_2:2102 → connects to miner_1:2001
    └── webserver_3:2103 → connects to miner_1:2001
```

Miners form a chain, while all webservers connect to the first miner.

## Key Points

1. **Miner 1 is always the seed node**: Uses `NODE_CONNECT_NODES=local`
2. **Additional miners form a chain**: Each connects to the previous miner
3. **All webservers connect to miner_1**: They never connect to other webservers
4. **Sequential startup ensures connectivity**: Nodes wait for their target node before starting
5. **Override behavior**: Explicit `NODE_CONNECT_NODES` values are preserved (except "local")

## Wait Script Behavior

The `wait-for-node.sh` script uses a **combined loop** that:
- Iterates backwards through miner instances (miner_2, miner_1, etc.)
- For each miner instance, waits for it to be ready (up to 60 attempts, 2 minutes)
- Checks if the P2P port is listening (TCP connection test)
- When a ready miner is found, outputs `PREV_NODE_ADDRESS=hostname:port` and exits immediately

**For Miners**: Waits for previous miner instance
- Miner 2 waits for `miner_1:2001`
- Miner 3 waits for `miner_2:2002` (or falls back to `miner_1:2001` if miner_2 doesn't exist)

**For Webservers**: Always waits for miners (iterates backwards to find available miner)
- Webserver 2 waits for `miner_1:2001` (not webserver_1)
- Webserver 3 waits for `miner_1:2001` (checks miner_2 first, then falls back to miner_1)

## Troubleshooting Connections

### Node Not Connecting

1. **Check if target node is ready**:
   ```bash
   # For miners
   docker compose exec miner_2 nc -zv miner_1 2001
   
   # For webservers
   docker compose exec webserver_2 nc -zv miner_1 2001
   ```

2. **Check logs for connection errors**:
   ```bash
   docker compose logs miner_2 | grep -i connect
   docker compose logs webserver_2 | grep -i connect
   ```

3. **Verify NODE_CONNECT_NODES**:
   ```bash
   docker compose exec miner_2 env | grep NODE_CONNECT_NODES
   docker compose exec webserver_2 env | grep NODE_CONNECT_NODES
   ```

### All Webservers Connecting to Same Miner

This is **expected behavior**. All webservers connect to `miner_1:2001` by design. If you need webservers to connect to different miners, you would need to:
1. Modify the entrypoint script
2. Use custom `NODE_CONNECT_NODES` per instance via override files
3. Set environment variables per container

### Miners Not Forming Chain

If miners aren't forming a chain:
1. **Check sequential startup is enabled**: `SEQUENTIAL_STARTUP=yes`
2. **Verify NODE_CONNECT_NODES**: Should be "local" for miner_1, or auto-configured for others
3. **Check wait script output**: Look for "Previous node is ready!" messages in logs

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Execution Flow](03-Execution-Flow.md) | [↑ Table of Contents](#) | [Next Section: Sequential Startup →](07-Sequential-Startup.md) |
|:---:|:---:|:---:|
| *Section 3* | *Current Section* | *Section 5* |

</div>

---
