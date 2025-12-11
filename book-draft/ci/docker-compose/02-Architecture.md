<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../../README.md)
2. [Chapter 2: Transaction System](../../bitcoin-blockchain/02-Transaction-System.md)
3. [Chapter 4: Desktop Admin Interface](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)
4. [Chapter 5: Wallet User Interface](../../bitcoin-wallet-ui/04-Wallet-UI.md)
5. [Chapter 6: Embedded Database & Persistence](../../bitcoin-wallet-ui/05-Embedded-Database.md)
6. [Chapter 7: Web Admin Interface](../../bitcoin-web-ui/06-Web-Admin-UI.md)

### Part II: Deployment & Operations

8. [Chapter 8: Docker Compose Deployment](01-Introduction.md) - Main chapter
   - **Section 2: Architecture & Container System** ← *You are here*
   - [Section 3: Execution Flow](03-Execution-Flow.md)
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

# Chapter 7, Section 2: Architecture & Container System

**Part II: Deployment & Operations** | **Chapter 8: Docker Compose Deployment**

<div align="center">

**📚 [← Section 1: Introduction](01-Introduction.md)** | **Section 2: Architecture** | **[Section 3: Execution Flow →](03-Execution-Flow.md)** 📚

</div>

---

## Prerequisites

Before reading this section, you should have:
- Completed [Section 1: Introduction & Quick Start](01-Introduction.md)
- Basic understanding of Docker containers and Docker Compose
- Familiarity with container networking concepts

## Learning Objectives

After reading this section, you will understand:
- How containers are named and identified in the deployment system
- How instance numbers are detected and used
- The volume and data directory structure
- Container lifecycle and service type identification

---

This section explains the container architecture, naming conventions, and how the system identifies and configures containers.

## Container Naming System

### Container Name Source

In `docker-entrypoint.sh` at **line 36**:

```bash
CONTAINER_NAME="${HOSTNAME:-}"
```

### The Chain of Events

#### 1. Docker Sets HOSTNAME Environment Variable

When Docker starts a container, it automatically sets the `HOSTNAME` environment variable to the container's hostname, which is typically the container name.

#### 2. Docker Compose Container Naming

When using Docker Compose, containers are named using this pattern:

```
<project>_<service>_<instance_number>
```

**Examples:**
- `blockchain_miner_1` (first miner instance)
- `blockchain_miner_2` (second miner instance)
- `blockchain_webserver_1` (first webserver instance)
- `blockchain_webserver_2` (second webserver instance)

**Note:** The project name defaults to the directory name (e.g., `blockchain`), but can be overridden with:
- `COMPOSE_PROJECT_NAME` environment variable
- `-p` or `--project-name` flag in docker compose commands

#### 3. Entrypoint Script Reads HOSTNAME

The entrypoint script reads `HOSTNAME` into `CONTAINER_NAME`:

```bash
CONTAINER_NAME="${HOSTNAME:-}"  # Gets value from Docker's HOSTNAME env var
```

If `HOSTNAME` is not set (shouldn't happen in Docker), it defaults to an empty string.

#### 4. Instance Number Extraction

The script then extracts the instance number from `CONTAINER_NAME`:

```bash
# Pattern match: blockchain_miner_1 matches _([0-9]+)$
if [[ "${CONTAINER_NAME}" =~ _([0-9]+)$ ]]; then
    INSTANCE_NUMBER="${BASH_REMATCH[1]}"  # Extracts "1" from "blockchain_miner_1"
fi
```

**How it works:**
- The regex pattern `_([0-9]+)$` matches an underscore followed by one or more digits at the end of the string
- `BASH_REMATCH[1]` captures the first (and only) capture group, which is the instance number
- If no match is found, `INSTANCE_NUMBER` defaults to `1`

#### 5. Service Type Detection

The script also detects the service type from the container name:

```bash
if [[ "${CONTAINER_NAME}" =~ miner ]]; then
    SERVICE_NAME_FROM_CONTAINER="miner"
elif [[ "${CONTAINER_NAME}" =~ webserver ]]; then
    SERVICE_NAME_FROM_CONTAINER="webserver"
fi
```

**How it works:**
- Checks if the container name contains the string "miner" or "webserver"
- Sets `SERVICE_NAME_FROM_CONTAINER` accordingly
- This is used as a fallback if environment variables aren't set

## Example Flow

### For Miner Instance 1:

1. **Docker Compose creates container**: `blockchain_miner_1`
2. **Docker sets HOSTNAME**: `HOSTNAME=blockchain_miner_1`
3. **Entrypoint script reads**: `CONTAINER_NAME="blockchain_miner_1"`
4. **Extracts instance number**: `INSTANCE_NUMBER=1`
5. **Detects service type**: `SERVICE_NAME_FROM_CONTAINER="miner"`

### For Webserver Instance 2:

1. **Docker Compose creates container**: `blockchain_webserver_2`
2. **Docker sets HOSTNAME**: `HOSTNAME=blockchain_webserver_2`
3. **Entrypoint script reads**: `CONTAINER_NAME="blockchain_webserver_2"`
4. **Extracts instance number**: `INSTANCE_NUMBER=2`
5. **Detects service type**: `SERVICE_NAME_FROM_CONTAINER="webserver"`

## Verification

You can verify this by checking inside a running container:

```bash
# Check HOSTNAME environment variable
docker compose exec miner_1 env | grep HOSTNAME
# Output: HOSTNAME=blockchain_miner_1

# Or check the hostname directly
docker compose exec miner_1 hostname
# Output: blockchain_miner_1

# Check instance number detection
docker compose exec miner_1 echo $INSTANCE_NUMBER
# Output: 1
```

## Custom Container Names

If you set `container_name` in docker compose.yml (not recommended for scaling), Docker will use that name instead:

```yaml
services:
  miner:
    container_name: my-custom-miner-name
    # ...
```

In this case:
- `HOSTNAME=my-custom-miner-name`
- `CONTAINER_NAME="my-custom-miner-name"`
- Instance number extraction might fail (defaults to 1)
- Service type detection might fail (falls back to environment variables)

**Note:** Using `container_name` prevents scaling, so it's not recommended for multi-instance setups.

## Port Calculation

Based on the instance number, ports are calculated as follows:

### Miners

```bash
P2P_PORT=$((2001 + INSTANCE_NUMBER - 1))
```

**Examples:**
- Instance 1: `2001 + 1 - 1 = 2001`
- Instance 2: `2001 + 2 - 1 = 2002`
- Instance 3: `2001 + 3 - 1 = 2003`

### Webservers

```bash
WEB_PORT=$((8080 + INSTANCE_NUMBER - 1))
P2P_PORT=$((2101 + INSTANCE_NUMBER - 1))
```

**Examples:**
- Instance 1: Web `8080`, P2P `2101`
- Instance 2: Web `8081`, P2P `2102`
- Instance 3: Web `8082`, P2P `2103`

## Data Directory Structure

Each instance uses an isolated data directory based on its instance number:

```bash
DATA_DIR="data${INSTANCE_NUMBER}"
BLOCKS_TREE="blocks${INSTANCE_NUMBER}"
```

**Examples:**
- Instance 1: `data1/`, `blocks1/`
- Instance 2: `data2/`, `blocks2/`
- Instance 3: `data3/`, `blocks3/`

### Volume Mounting

Data directories are stored within Docker volumes:

**Miners:**
- Volume: `miner-data`
- Mount point: `/app/data`
- Instance directories: `/app/data/data1`, `/app/data/data2`, etc.

**Webservers:**
- Volume: `webserver-data`
- Mount point: `/app/data`
- Instance directories: `/app/data/data1`, `/app/data/data2`, etc.

**Wallets:**
- Miners: `miner-wallets` → `/app/wallets`
- Webservers: `webserver-wallets` → `/app/wallets`
- Wallet file location: `/app/wallets/wallets.dat` (inside container)
- Environment variable: `WALLET_FILE=wallets/wallets.dat`

**Accessing Wallet Files:**

```bash
# List wallet files
docker exec -it <container_name> ls -la /app/wallets/

# Copy wallet file to host
docker cp <container_name>:/app/wallets/wallets.dat ./backup.dat

# Copy wallet file from host to container
docker cp ./backup.dat <container_name>:/app/wallets/wallets.dat

# Inspect wallet volume (shows host path)
docker volume inspect blockchain_miner-wallets
docker volume ls | grep wallets

# Get exact host path
docker volume inspect blockchain_miner-wallets --format '{{ .Mountpoint }}'
```

**Host Filesystem Location:**

**Linux:**
- Path: `/var/lib/docker/volumes/blockchain_miner-wallets/_data/wallets.dat`
- Path: `/var/lib/docker/volumes/blockchain_webserver-wallets/_data/wallets.dat`

**macOS (Docker Desktop):**
- Volumes are stored on local macOS storage, but inside Docker's Linux VM filesystem
- VM filesystem location: `~/Library/Containers/com.docker.docker/Data/vms/0/data/docker/volumes/`
- VM path (shown by `docker volume inspect`): `/var/lib/docker/volumes/blockchain_miner-wallets/_data`
- Access via `docker exec` or `docker cp` commands (recommended)
- Direct file access requires navigating the VM disk image (complex)

**Windows (Docker Desktop):**
- Similar to macOS, volumes stored on local Windows storage inside Docker's Linux VM
- Use `docker volume inspect` to see the mountpoint path inside the VM
- Access via `docker exec` or `docker cp` commands

**Note:** Wallet files are binary-encoded and contain private keys. Handle with appropriate security measures.

## Container Lifecycle

### Creation

1. Docker Compose reads `docker compose.yml`
2. Creates containers with names following the pattern `<project>_<service>_<number>`
3. Sets `HOSTNAME` environment variable
4. Mounts volumes
5. Sets environment variables from compose file

### Startup

1. Container starts
2. Entrypoint script (`docker-entrypoint.sh`) executes
3. Script reads `HOSTNAME` → `CONTAINER_NAME`
4. Extracts instance number and service type
5. Calculates ports and data directories
6. Configures node connection (if sequential startup enabled)
7. Executes blockchain binary

### Runtime

- Containers run independently
- Each has its own isolated data directory
- Containers communicate via Docker network using service names
- Health checks run periodically

### Shutdown

- Containers stop gracefully
- Data persists in volumes
- Volumes are not deleted unless explicitly removed with `docker compose down -v`

## Summary

```
Docker Compose
    ↓
Creates container with name: blockchain_miner_1
    ↓
Docker sets HOSTNAME=blockchain_miner_1
    ↓
Entrypoint script: CONTAINER_NAME="${HOSTNAME:-}"
    ↓
CONTAINER_NAME="blockchain_miner_1"
    ↓
Extract INSTANCE_NUMBER=1 from container name
    ↓
Detect SERVICE_TYPE="miner" from container name
    ↓
Calculate P2P_PORT=2001, DATA_DIR=data1
    ↓
Configure and start blockchain node
```

## Key Points

1. **Container names follow a strict pattern**: `<project>_<service>_<number>`
2. **Instance numbers are extracted automatically** from container names
3. **Ports are calculated** based on instance numbers
4. **Data directories are isolated** per instance within shared volumes
5. **Service types are detected** from container names as a fallback
6. **Custom container names break scaling** - avoid using `container_name` in compose files

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Introduction](01-Introduction.md) | [↑ Table of Contents](#) | [Next Section: Execution Flow →](03-Execution-Flow.md) |
|:---:|:---:|:---:|
| *Section 1* | *Current Section* | *Section 3* |

</div>

---
