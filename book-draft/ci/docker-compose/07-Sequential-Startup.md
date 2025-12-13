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
   - **Section 5: Sequential Startup** ← *You are here*
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

# Chapter 7, Section 5: Sequential Startup

**Part II: Deployment & Operations** | **Chapter 8: Docker Compose Deployment**

<div align="center">

**📚 [← Section 4: Network Configuration](04-Network-Configuration.md)** | **Section 5: Sequential Startup** | **[Section 6: Port Mapping →](05-Port-Mapping.md)** 📚

</div>

---

## Prerequisites

Before reading this section, you should have:
- Completed [Section 4: Network Configuration & Node Connections](04-Network-Configuration.md)
- Understanding of how nodes connect to each other
- Basic knowledge of health checks and service dependencies

## Learning Objectives

After reading this section, you will understand:
- How sequential startup ensures nodes start in the correct order
- The wait script mechanism and health check coordination
- How to enable or disable sequential startup
- Troubleshooting startup sequencing issues

---

This section explains how sequential startup works, where each node waits for the previous node to be ready before starting.

**Note**: All commands should be run from the `configs/` directory where `docker compose.yml` is located.

## How It Works

When `SEQUENTIAL_STARTUP=yes` (default), nodes start in sequence:

1. **Instance 1** starts immediately (no wait needed)
2. **Instance 2** waits for Instance 1 to be ready, then connects to it
3. **Instance 3** waits for Instance 2 to be ready, then connects to it
4. And so on...

Each node automatically sets `NODE_CONNECT_NODES` to the previous node's address.

## Configuration

### Enable Sequential Startup (Default)

Sequential startup is enabled by default. No configuration needed:

```bash
cd configs
./docker compose.scale.sh 3 2
```

### Disable Sequential Startup

To disable sequential startup and start all nodes simultaneously:

```bash
SEQUENTIAL_STARTUP=no ./docker compose.scale.sh 3 2
```

Or:

```bash
SEQUENTIAL_STARTUP=no docker compose up -d --scale miner=3 --scale webserver=2
```

**Note**: Disabling sequential startup may cause connection issues if nodes try to connect before their target nodes are ready.

## How Nodes Connect

### Miners

- **Miner 1**: Starts as seed node (`NODE_CONNECT_NODES=local`)
- **Miner 2**: Connects to `miner_1:2001`
- **Miner 3**: Connects to `miner_2:2002`
- **Miner 4**: Connects to `miner_3:2003`
- And so on...

### Webservers

**Important**: Webservers **always** connect to miners, never to other webservers.

- **Webserver 1**: Connects to `miner_1:2001` (first miner)
- **Webserver 2**: Connects to `miner_1:2001` (first miner, not webserver_1)
- **Webserver 3**: Connects to `miner_1:2001` (first miner, not webserver_2)
- All webservers connect to the same miner (`miner_1:2001`)

## Health Checks

### Wait Script Behavior

The wait script (`wait-for-node.sh`) uses a **combined loop** that:
- Iterates backwards through miner instances (miner_2, miner_1, etc.)
- For each miner instance, waits for it to be ready (up to 60 attempts)
- Checks if the P2P port is listening (TCP connection test)
- When a ready miner is found, outputs `PREV_NODE_ADDRESS=hostname:port` and exits immediately

**For Miners**: Waits for previous miner instance
- Miner 2 waits for `miner_1:2001`
- Miner 3 waits for `miner_2:2002` (or falls back to `miner_1:2001` if miner_2 doesn't exist)

**For Webservers**: Always waits for miners (iterates backwards to find available miner)
- Webserver 2 waits for `miner_1:2001` (not webserver_1)
- Webserver 3 waits for `miner_1:2001` (checks miner_2 first, then falls back to miner_1)

## Wait Timeout

The wait script will:
- Check every 2 seconds
- Retry up to 60 times (2 minutes total)
- Fail if the previous node doesn't become ready

### Wait Script Code Flow

```bash
# wait-for-node.sh execution for miner instance 2
WAIT_SERVICE_NAME="miner"
INSTANCE_NUMBER=2
PORT=2002
PREV_INSTANCE=$((2 - 1))  # = 1

# Iterate backwards from PREV_INSTANCE
CHECK_INSTANCE=1
CHECK_HOSTNAME="miner_1"
CHECK_PORT=2001

# Wait loop (up to 60 attempts)
for attempt in {1..60}; do
    # Try TCP connection
    if timeout 1 bash -c "echo > /dev/tcp/${CHECK_HOSTNAME}/${CHECK_PORT}"; then
        echo "PREV_NODE_ADDRESS=${CHECK_HOSTNAME}:${CHECK_PORT}"
        exit 0
    fi
    sleep 2
done

# If we get here, previous node never became ready
exit 1
```

## Examples

### Example 1: Sequential Miners

```bash
# Start 3 miners sequentially
./docker compose.scale.sh 3 1

# Miner 1 starts immediately
# Miner 2 waits for Miner 1, then connects to miner_1:2001
# Miner 3 waits for Miner 2, then connects to miner_2:2002
```

### Example 2: Sequential Webservers Connecting to Miners

```bash
# Start 1 miner and 2 webservers
./docker compose.scale.sh 1 2

# Webserver 1 connects to miner_1:2001 (auto-configured)
# Webserver 2 waits for miner_1:2001, then connects to miner_1:2001
```

### Example 3: Multiple Webservers All Connect to Same Miner

```bash
# Start 1 miner and 3 webservers
./docker compose.scale.sh 1 3

# All webservers connect to miner_1:2001
# Webserver 1: miner_1:2001
# Webserver 2: miner_1:2001 (waits for miner_1, not webserver_1)
# Webserver 3: miner_1:2001 (waits for miner_1, not webserver_2)
```

## Monitoring Sequential Startup

### View Startup Logs

```bash
# Watch all logs
docker compose logs -f

# Watch specific service
docker compose logs -f miner
docker compose logs -f webserver

# Watch specific instance
docker compose logs -f miner_2
```

Look for messages like:
```
Instance 2: Waiting for previous node...
  Previous node: miner_1
  Previous port: 2001
  Attempt 1/60: Waiting for miner_1:2001...
Instance 2: Previous node is ready!
  Using previous node address: miner_1:2001
```

### Check Node Status

```bash
# Check if nodes are running
docker compose ps

# Check health status
docker compose ps --format "table {{.Name}}\t{{.Status}}\t{{.Ports}}"
```

### Verify Sequential Order

```bash
# Check startup timestamps
docker compose ps --format "table {{.Name}}\t{{.Status}}\t{{.CreatedAt}}"

# Check logs for wait messages
docker compose logs miner_2 | grep -i wait
docker compose logs webserver_2 | grep -i wait
```

## Troubleshooting

### Node Not Starting

If a node fails to start because the previous node isn't ready:

1. **Check previous node logs**:
   ```bash
   docker compose logs miner_1
   ```

2. **Check if previous node is healthy**:
   ```bash
   # For webservers
   curl http://localhost:8080/api/health/ready
   
   # For miners (check port)
   nc -zv localhost 2001
   ```

3. **Increase wait timeout** (edit `wait-for-node.sh`):
   ```bash
   MAX_ATTEMPTS=120  # Wait up to 4 minutes
   ```

### Connection Issues

If nodes aren't connecting properly:

1. **Verify network connectivity**:
   ```bash
   docker compose exec miner_2 ping miner_1
   ```

2. **Check DNS resolution**:
   ```bash
   docker compose exec miner_2 nslookup miner_1
   ```

3. **Verify ports**:
   ```bash
   docker compose exec miner_2 nc -zv miner_1 2001
   ```

### Disable Sequential Startup Temporarily

If you need to start all nodes simultaneously for testing:

```bash
SEQUENTIAL_STARTUP=no ./docker compose.scale.sh 3 1
```

**Warning**: This may cause connection failures if nodes try to connect before their targets are ready.

## Advanced Configuration

### Custom Wait Script

You can customize the wait behavior by modifying `wait-for-node.sh`:

- Change wait interval: `sleep 2` → `sleep 5`
- Change max attempts: `MAX_ATTEMPTS=60` → `MAX_ATTEMPTS=120`
- Add custom health checks

### Per-Service Sequential Startup

You can enable sequential startup for one service but not another by setting environment variables per service:

```yaml
# docker compose.override.yml
services:
  miner:
    environment:
      - SEQUENTIAL_STARTUP=yes
  webserver:
    environment:
      - SEQUENTIAL_STARTUP=no
```

Note: This requires using separate compose files or override files.

## How Sequential Startup Works Internally

### Entrypoint Script Logic

**File**: `docker-entrypoint.sh`

**Line 126**: Check if sequential startup applies
```bash
if [ "${SEQUENTIAL_STARTUP:-yes}" = "yes" ] && [ "${INSTANCE_NUMBER}" -gt 1 ]; then
  # Miner 2, 3, 4... enter this block
```

**Line 132-137**: Determine wait service name
```bash
if [ "${NODE_IS_MINER}" = "yes" ]; then
    WAIT_SERVICE_NAME="miner"
else
    WAIT_SERVICE_NAME="miner"  # Webservers also wait for miners
fi
```

**Line 141**: Call wait script
```bash
/app/wait-for-node.sh "${WAIT_SERVICE_NAME}" "${INSTANCE_NUMBER}" "${P2P_PORT}" "${NODE_IS_WEB_SERVER}"
```

**Line 149-160**: Extract previous node address
```bash
PREV_ADDR=$(echo "${WAIT_OUTPUT}" | grep "PREV_NODE_ADDRESS=" | cut -d'=' -f2)
```

**Line 168-170**: Update NODE_CONNECT_NODES
```bash
if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ]; then
    NODE_CONNECT_NODES="${PREV_ADDR}"  # Replace "local" with previous node
fi
```

## Key Points

1. **Sequential startup is enabled by default** - No configuration needed
2. **First instance doesn't wait** - Instance 1 starts immediately
3. **Additional instances wait** - They wait for previous nodes before starting
4. **Webservers always wait for miners** - They never wait for other webservers
5. **Wait timeout is 2 minutes** - Configurable in wait script
6. **Automatic connection configuration** - NODE_CONNECT_NODES is set automatically

## Summary

Sequential startup ensures that:
- Nodes start in the correct order
- Each node waits for its target node to be ready
- Network connections are established properly
- No connection failures due to timing issues

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Network Configuration](04-Network-Configuration.md) | [↑ Table of Contents](#) | [Next Section: Port Mapping →](05-Port-Mapping.md) |
|:---:|:---:|:---:|
| *Section 4* | *Current Section* | *Section 6* |

</div>

---

This is especially important for blockchain networks where proper peer connections are critical for consensus and data synchronization.
