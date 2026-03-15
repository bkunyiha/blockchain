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
16. <a href="../../bitcoin-desktop-ui-iced/03-Desktop-Admin-UI.md">Chapter 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="../../bitcoin-wallet-ui-iced/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../../bitcoin-wallet-ui-iced/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
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

## Chapter 8, Section 5: Port Mapping & External Access

**Part II: Deployment & Operations** | **Chapter 8: Docker Compose Deployment**

<div align="center">

**📚 [← Chapter 6: Web Admin UI](../../bitcoin-web-ui/06-Web-Admin-UI.md)** | **Chapter 8: Docker Compose** | **[Chapter 9: Kubernetes →](../kubernetes/README.md)** 📚

</div>

---

This section explains port mapping behavior, the limitations when using `--scale`, and how to ensure all instances have ports accessible externally.

> **Methods involved**
> - `docker-compose.scale.sh` (override generation + scaling, [Listing 8.4](01A-Docker-Compose-Code-Listings.md#listing-84-cidocker-composeconfigsdocker-composescalesh))
> - `generate-compose-ports.sh` (override generation only, [Listing 8.5](01A-Docker-Compose-Code-Listings.md#listing-85-cidocker-composeconfigsgenerate-compose-portssh))
> - `docker-compose.yml` baseline ports ([Listing 8.1](01A-Docker-Compose-Code-Listings.md#listing-81-cidocker-composeconfigsdocker-composeyml))

## The Problem

**Docker Compose only maps ports for the first instance of each service** when using `--scale`.

### Important: Without `--scale`

If you run `docker compose up -d` **without** `--scale`:
- You get **1 instance** of each service (1 miner, 1 webserver)
- ✅ **Ports ARE mapped** for that single instance:
  - ✅ `miner_1`: Port `2001` → `localhost:2001` (accessible externally)
  - ✅ `webserver_1`: Ports `8080` and `2101` → `localhost:8080` and `localhost:2101` (accessible externally)

### With `--scale`: Port Mapping Limitation

If you run:
```bash
docker compose up -d --scale miner=2 --scale webserver=1
```

**Port Mapping Result:**
- ✅ `miner_1`: Port `2001` **IS mapped** to host (accessible externally)
- ❌ `miner_2`: Port `2002` **NOT mapped** to host (only accessible via Docker network)
- ✅ `webserver_1`: Ports `8080` and `2101` **ARE mapped** to host (accessible externally)

**Key Point**: When using `--scale`, only the **first instance** gets ports mapped. Additional instances created by scaling do NOT get ports mapped to the host.

### Why This Happens

Docker Compose's `ports:` section in `docker compose.yml` only applies to the **first instance** when using `--scale`. Additional instances:
- Use their internal ports (2002, 2003, etc.)
- Are accessible within the Docker network
- Are **NOT** accessible from the host machine

## Solution: Use Scaling Helper Script (Recommended)

The `docker compose.scale.sh` script **automatically generates port mappings** for all instances, ensuring all ports are accessible externally.

### Recommended: Use Helper Script

```bash
cd configs
# Automatically generates port mappings and scales services
./docker compose.scale.sh 2 1  # 2 miners, 1 webserver
```

This script:
1. Automatically generates `docker compose.override.yml` with port mappings for all instances
2. Scales services with all ports accessible externally
3. Maps ports for **all instances**:
   - ✅ `miner_1`: Port `2001` → `localhost:2001`
   - ✅ `miner_2`: Port `2002` → `localhost:2002`
   - ✅ `webserver_1`: Port `8080` → `localhost:8080`, Port `2101` → `localhost:2101`

**All ports are automatically accessible externally!**

### Alternative: Manual Port Override File

If you prefer to generate the override file manually:

```bash
cd configs
# Step 1: Generate override file for 2 miners and 1 webserver
./generate-compose-ports.sh 2 1
```

This creates `docker compose.override.yml` with port mappings for all instances:

```yaml
version: '3.8'

services:
  miner:
    ports:
      - "2001:2001"  # Miner instance 1
      - "2002:2002"  # Miner instance 2

  webserver:
    ports:
      - "8080:8080"  # Webserver instance 1 - Web
      - "2101:2001"  # Webserver instance 1 - P2P
```

```bash
# Step 2: Start services (override file is automatically used)
docker compose up -d --scale miner=2 --scale webserver=1
```

## Port Mapping Reference

### Miners

| Instance | Internal Port | External Port (with override) | Accessible Without Override? |
|----------|---------------|-------------------------------|------------------------------|
| miner_1  | 2001          | 2001                          | ✅ Yes                       |
| miner_2  | 2002          | 2002                          | ❌ No                        |
| miner_3  | 2003          | 2003                          | ❌ No                        |

### Webservers

| Instance   | Internal Web Port | External Web Port (with override) | Internal P2P Port | External P2P Port (with override) | Accessible Without Override? |
|------------|-------------------|------------------------------------|-------------------|-----------------------------------|------------------------------|
| webserver_1 | 8080              | 8080                              | 2101              | 2101                              | ✅ Yes                       |
| webserver_2 | 8081              | 8081                              | 2102              | 2102                              | ❌ No                        |
| webserver_3 | 8082              | 8082                              | 2103              | 2103                              | ❌ No                        |

## Complete Workflow

### Scenario: Start with 2 Miners, 1 Webserver (All Ports Exposed)

```bash
# 1. Generate port override file
./docker compose.scale.sh 2 1

# 2. Verify ports are accessible
curl http://localhost:2001  # miner_1 ✅
curl http://localhost:2002  # miner_2 ✅
curl http://localhost:8080/api/health/ready  # webserver_1 ✅
```

### Scenario: Scale Up Later (Need to Regenerate Override)

```bash
# Current: 2 miners, 1 webserver
# Want to scale to: 3 miners, 2 webservers

# 1. Regenerate override file for new counts
./docker compose.scale.sh 3 2

# 2. Scale up (keeps existing containers running)
docker compose up -d --scale miner=3 --scale webserver=2

# 3. Now all ports accessible:
#    miner_1: 2001 ✅
#    miner_2: 2002 ✅
#    miner_3: 2003 ✅
#    webserver_1: 8080, 2101 ✅
#    webserver_2: 8081, 2102 ✅
```

## Important Notes

### 1. Override File Must Match Scale Count

**Important**: The `docker compose.override.yml` file must include port mappings for **all instances** you plan to run. If you scale beyond what's in the override file, additional instances won't have ports mapped.

**Example:**
```bash
# Override file has ports for 2 miners
./generate-compose-ports.sh 2 1

# But you scale to 3 miners
docker compose up -d --scale miner=3

# Result:
# - miner_1: Port mapped ✅ (from override)
# - miner_2: Port mapped ✅ (from override)
# - miner_3: Port NOT mapped ❌ (not in override)
```

**Solution**: Regenerate override file before scaling:
```bash
./docker compose.scale.sh 3 1  # Regenerate for 3 miners
docker compose up -d --scale miner=3
```

### 2. Port Conflicts

Make sure ports aren't already in use:
```bash
# Check if ports are available
netstat -an | grep 2001
netstat -an | grep 2002
netstat -an | grep 8080

# Or on Linux
ss -tulpn | grep -E '2001|2002|8080'
```

### 3. Internal Access (Without Override)

Even without port mapping, containers can access each other via Docker network:
- `miner_2` can be accessed from `miner_1` as `miner_2:2002`
- `webserver_2` can be accessed from `webserver_1` as `webserver_2:8081`

This works because all containers are on the same Docker network.

## Alternative: Load Balancer / Reverse Proxy

Instead of exposing all ports, you can use a reverse proxy (nginx, traefik, etc.) to route traffic:

```yaml
# docker compose.yml addition
services:
  nginx:
    image: nginx:alpine
    ports:
      - "2001:2001"  # Single entry point
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
```

Then configure nginx to load balance across all miner instances internally.

## Alternative: Separate Services (No Scale Needed)

Instead of using `--scale`, you can define **separate services** in docker compose.yml:

```yaml
services:
  miner1:
    ports:
      - "2001:2001"  # ✅ Mapped
  miner2:
    ports:
      - "2002:2002"  # ✅ Mapped (different service = different mapping)
  webserver1:
    ports:
      - "8080:8080"  # ✅ Mapped
      - "2101:2001"  # ✅ Mapped
```

Then run `docker compose up -d` **without `--scale`**:
- ✅ All ports are accessible externally
- ❌ Cannot dynamically scale (must edit compose file)

See [Chapter 8: Deployment Scenarios & Examples](08-Deployment-Scenarios.md) for complete example.

## Summary

| Scenario | Port Mapping | Solution |
|----------|--------------|----------|
| Start with 2 miners, 1 webserver (using `--scale`) | Only first instance mapped | ✅ Use `docker compose.scale.sh` (auto-generates ports) |
| Start with 2 miners, 1 webserver (separate services) | ✅ All instances mapped | Define separate services, no `--scale` needed |
| Scale from 1 to 2 miners | Only first instance mapped | ✅ Use `docker compose.scale.sh` (auto-updates ports) |
| Need all instances accessible | ✅ All instances mapped | ✅ Use `docker compose.scale.sh` (recommended) |
| Only need first instance | Default behavior | Use `--scale` directly (not recommended for blockchain) |
| Fixed number of instances | Use separate services | All ports accessible, no override needed |

**Recommendation**: 
- **For blockchain (ports must be accessible)**: Always use `docker compose.scale.sh` - it automatically ensures all ports are mapped
- **Fixed instances**: Use separate services for all ports accessible
- **Dynamic scaling**: Use `docker compose.scale.sh` - it handles port mapping automatically

## Troubleshooting

### Port Already in Use

If you get an error that a port is already in use:

```bash
# Find what's using the port
lsof -i :2001
# Or on Linux
ss -tulpn | grep :2001

# Stop the conflicting service or change the port in override file
```

### Override File Not Working

If ports still aren't mapped after generating override:

1. **Check override file exists**: `ls -la docker compose.override.yml`
2. **Verify file contents**: `cat docker compose.override.yml`
3. **Check Docker Compose is using it**: `docker compose config` (shows merged config)
4. **Regenerate**: `./generate-compose-ports.sh <miners> <webservers>`

### Ports Not Accessible from Host

If ports are mapped but not accessible:

1. **Check container is running**: `docker compose ps`
2. **Check port mapping**: `docker compose ps` shows port mappings
3. **Test from container**: `docker compose exec miner_1 curl localhost:2001`
4. **Check firewall**: Ensure firewall allows the ports

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Sequential Startup](07-Sequential-Startup.md) | [↑ Table of Contents](#) | [Next Section: Scaling →](06-Scaling.md) |
|:---:|:---:|:---:|
| *Section 5* | *Current Section* | *Section 7* |

</div>

---
