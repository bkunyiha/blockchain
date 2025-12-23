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
   - **Section 8: Deployment Scenarios & Examples** ← *You are here*
9. [Chapter 9: Kubernetes Deployment](../../kubernetes/README.md)

</details>

</div>

<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

# Chapter 7, Section 8: Deployment Scenarios & Examples

**Part II: Deployment & Operations** | **Chapter 8: Docker Compose Deployment**

<div align="center">

**📚 [← Chapter 6: Web Admin UI](../../bitcoin-web-ui/06-Web-Admin-UI.md)** | **Chapter 8: Docker Compose** | **[Chapter 9: Kubernetes →](../../kubernetes/README.md)** 📚

</div>

---

This section provides detailed examples of common deployment scenarios, including network topologies, connection patterns, and best practices.

## Scenario 1: Single Miner + Single Webserver

### Configuration

```bash
cd configs
docker compose up -d
```

### Network Topology

```
redis:6379 (rate limiting backend)
    ↑
miner_1:2001 (seed node, "local")
    ↑
webserver_1:2101 → connects to miner_1:2001
```

### Details

- **Redis**: Provides shared state for the webserver’s Redis-backed API rate limiting.
- **Miner 1**: Acts as seed node, P2P port 2001
- **Webserver 1**: Connects to miner_1:2001, Web port 8080, P2P port 2101
- **Ports accessible**: 2001, 8080, 2101
- **Use case**: Development, testing, small deployments

### Access Points

- Miner P2P: `localhost:2001`
- Webserver API: `http://localhost:8080`
- Webserver P2P: `localhost:2101`

## Scenario 2: Single Miner + Multiple Webservers

### Configuration

```bash
cd configs
./docker compose.scale.sh 1 3
```

### Network Topology

```
miner_1:2001 (seed node, "local")
    ↑
    ├── webserver_1:2101 → connects to miner_1:2001
    ├── webserver_2:2102 → connects to miner_1:2001
    └── webserver_3:2103 → connects to miner_1:2001
```

### Details

- **Miner 1**: Acts as seed node, P2P port 2001
- **Webserver 1**: Web port 8080, P2P port 2101
- **Webserver 2**: Web port 8081, P2P port 2102
- **Webserver 3**: Web port 8082, P2P port 2103
- **All webservers connect to miner_1:2001** (star topology)
- **Ports accessible**: 2001, 8080, 8081, 8082, 2101, 2102, 2103
- **Use case**: High availability web services, load balancing

### Access Points

- Miner P2P: `localhost:2001`
- Webserver 1 API: `http://localhost:8080`
- Webserver 2 API: `http://localhost:8081`
- Webserver 3 API: `http://localhost:8082`

### Execution Order

1. **Miner 1** starts immediately (seed node)
2. **Webserver 1** starts after miner_1 is healthy (depends_on)
3. **Webserver 2** waits for miner_1, then starts
4. **Webserver 3** waits for miner_1, then starts

## Scenario 3: Multiple Miners + Single Webserver

### Configuration

```bash
cd configs
./docker compose.scale.sh 3 1
```

### Network Topology

```
miner_1:2001 (seed, "local")
    ↑
miner_2:2002 → connects to miner_1:2001
    ↑
miner_3:2003 → connects to miner_2:2002
    ↑
webserver_1:2101 → connects to miner_1:2001
```

### Details

- **Miner 1**: Seed node, P2P port 2001
- **Miner 2**: Connects to miner_1:2001, P2P port 2002
- **Miner 3**: Connects to miner_2:2002, P2P port 2003
- **Webserver 1**: Connects to miner_1:2001, Web port 8080, P2P port 2101
- **Miners form a chain**, webserver connects to first miner
- **Ports accessible**: 2001, 2002, 2003, 8080, 2101
- **Use case**: Distributed mining, consensus testing

### Access Points

- Miner 1 P2P: `localhost:2001`
- Miner 2 P2P: `localhost:2002`
- Miner 3 P2P: `localhost:2003`
- Webserver API: `http://localhost:8080`

### Execution Order

1. **Miner 1** starts immediately (seed node)
2. **Miner 2** waits for miner_1, then starts
3. **Miner 3** waits for miner_2, then starts
4. **Webserver 1** starts after miner_1 is healthy (depends_on)

## Scenario 4: Multiple Miners + Multiple Webservers

### Configuration

```bash
cd configs
./docker compose.scale.sh 3 2
```

### Network Topology

```
miner_1:2001 (seed, "local")
    ↑
    ├── miner_2:2002 → connects to miner_1:2001
    │       ↑
    │   miner_3:2003 → connects to miner_2:2002
    │
    ├── webserver_1:2101 → connects to miner_1:2001
    └── webserver_2:2102 → connects to miner_1:2001
```

### Details

- **Miners form a chain**: miner_1 → miner_2 → miner_3
- **All webservers connect to miner_1**: Star topology around first miner
- **Ports accessible**: All miner and webserver ports
- **Use case**: Production deployment, high availability

### Access Points

- Miner P2P: `localhost:2001`, `localhost:2002`, `localhost:2003`
- Webserver APIs: `http://localhost:8080`, `http://localhost:8081`

## Scenario 5: Separate Services (No Scale)

### Configuration

Instead of using `--scale`, define separate services in `docker compose.yml`:

```yaml
version: '3.8'

services:
  # First miner
  miner1:
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "2001:2001"  # ✅ Port mapped
    volumes:
      - miner-data:/app/data
      - miner-wallets:/app/wallets
    environment:
      - NODE_IS_MINER=yes
      - NODE_IS_WEB_SERVER=no
      - NODE_CONNECT_NODES=local
      - SEQUENTIAL_STARTUP=no
    container_name: blockchain_miner_1

  # Second miner
  miner2:
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "2002:2002"  # ✅ Port mapped (different service = different port mapping)
    volumes:
      - miner-data:/app/data
      - miner-wallets:/app/wallets
    environment:
      - NODE_IS_MINER=yes
      - NODE_IS_WEB_SERVER=no
      - NODE_CONNECT_NODES=miner1:2001
      - SEQUENTIAL_STARTUP=no
    depends_on:
      - miner1
    container_name: blockchain_miner_2

  # Webserver
  webserver1:
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "8080:8080"  # ✅ Port mapped
      - "2101:2001"  # ✅ Port mapped
    volumes:
      - webserver-data:/app/data
      - webserver-wallets:/app/wallets
    environment:
      - NODE_IS_MINER=no
      - NODE_IS_WEB_SERVER=yes
      - NODE_CONNECT_NODES=miner1:2001
      - BITCOIN_API_ADMIN_KEY=${BITCOIN_API_ADMIN_KEY:-admin-secret}
      - BITCOIN_API_WALLET_KEY=${BITCOIN_API_WALLET_KEY:-wallet-secret}
    depends_on:
      - miner1
    container_name: blockchain_webserver_1

volumes:
  miner-data:
  miner-wallets:
  webserver-data:
  webserver-wallets:
```

### Running

```bash
cd configs
# Start all services (no --scale needed)
docker compose up -d
```

### Advantages

- ✅ **All ports accessible** - Each service has its own port mapping
- ✅ **No override file needed** - Ports defined directly in compose file
- ✅ **Explicit configuration** - Each service configured individually

### Disadvantages

- ❌ **Cannot dynamically scale** - Must edit compose file to add/remove instances
- ❌ **More verbose** - Requires defining each service separately
- ❌ **Less flexible** - Harder to scale up/down dynamically

### Use Case

- Fixed number of instances
- Production deployments with known requirements
- When you need explicit control over each instance

## Scenario 6: Development Environment

### Configuration

```bash
cd configs
# Single miner, single webserver for development
docker compose up -d
```

### Features

- Minimal resource usage
- Fast startup
- Easy debugging
- All ports accessible by default

### Access

- Miner: `localhost:2001`
- Webserver: `http://localhost:8080`

## Scenario 7: Production Deployment

### Configuration

```bash
cd configs
# Multiple miners for consensus, multiple webservers for HA
./docker compose.scale.sh 5 3
```

### Features

- High availability
- Load distribution
- Redundancy
- All ports accessible via helper script

### Network Topology

```
miner_1:2001 (seed)
    ↑
    ├── miner_2:2002 → miner_1
    ├── miner_3:2003 → miner_2
    ├── miner_4:2004 → miner_3
    ├── miner_5:2005 → miner_4
    │
    ├── webserver_1:2101 → miner_1
    ├── webserver_2:2102 → miner_1
    └── webserver_3:2103 → miner_1
```

## Best Practices

### 1. Always Use Helper Script for Scaling

```bash
# ✅ Good
./docker compose.scale.sh 3 2

# ❌ Bad (only first instance gets ports)
docker compose up -d --scale miner=3 --scale webserver=2
```

### 2. Verify Ports After Scaling

```bash
./docker compose.scale.sh 3 2
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

## Common Patterns

### Pattern 1: Horizontal Scaling

Scale webservers for increased web traffic:

```bash
# Start with 1 miner, 1 webserver
./docker compose.scale.sh 1 1

# Scale webservers to 5
./docker compose.scale.sh 1 5
```

### Pattern 2: Vertical Mining

Scale miners for increased mining power:

```bash
# Start with 1 miner, 1 webserver
./docker compose.scale.sh 1 1

# Scale miners to 5
./docker compose.scale.sh 5 1
```

### Pattern 3: Balanced Deployment

Equal miners and webservers:

```bash
# 3 miners, 3 webservers
./docker compose.scale.sh 3 3
```

## Troubleshooting Scenarios

### Scenario: Ports Not Accessible

**Problem**: After scaling, only first instance ports are accessible.

**Solution**: Use helper script to generate port mappings:
```bash
./docker compose.scale.sh 3 2
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

## Summary

Different deployment scenarios serve different purposes:

- **Development**: Single miner + single webserver
- **Testing**: Multiple miners + single webserver
- **High Availability**: Single miner + multiple webservers
- **Production**: Multiple miners + multiple webservers
- **Fixed Deployment**: Separate services (no scale)

Always use the helper script (`./docker compose.scale.sh`) to ensure all ports are accessible, which is critical for blockchain P2P networking.

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Scaling](06-Scaling.md) | [↑ Table of Contents](#) | [Next Section: Accessing Webserver →](09-Accessing-Webserver.md) |
|:---:|:---:|:---:|
| *Section 7* | *Current Section* | *Section 9* |

</div>

---
