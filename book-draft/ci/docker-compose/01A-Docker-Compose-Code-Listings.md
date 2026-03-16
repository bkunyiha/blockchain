<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../../bitcoin-blockchain/README.md">Chapter 2: Introduction to Bitcoin & Blockchain</a>
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

34. <a href="01-Introduction.md">Chapter 22: Docker Compose Deployment</a>
35. **22A: Code Listings** ← *You are here*
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

## Chapter 22A: Docker Compose — Complete Code Listings

This companion chapter contains **complete, verbatim listings** of the Docker Compose deployment artifacts used by Chapter 22.

The goal is practical: when the narrative chapter says “the entrypoint resolves hostnames to IP addresses” or “the scaling helper generates an override file,” you can inspect the **exact implementation** here without opening the repository.

---

## How to read these listings

Each listing begins with a short guide explaining:

- what the artifact does in the deployment system,
- which parts are operationally critical,
- and what other artifacts it interacts with.

---

## Listing 8.1: `ci/docker-compose/configs/docker-compose.yml`

This is the **primary Compose file**. It defines the multi-service local network:

- `redis` (rate limiting backend),
- `miner` (P2P + mining),
- `webserver` (HTTP API + P2P + rate limiting).

Important to understand:

- The Compose file is responsible for **volumes, ports, and environment variables**.
- **Sequential startup** and **port selection** are not implemented in YAML; they are implemented in `docker-entrypoint.sh` and helper scripts.
- The “only the first scaled instance gets ports” limitation is why this repo provides override generators (`docker-compose.scale.sh`, `generate-compose-ports.sh`).

> **Methods involved**
> - Artifact: `docker-compose.yml` (service definitions and wiring)

```yaml
# Multi-instance blockchain network
# Default: 1 miner + 1 webserver
# Scale with: docker compose up -d --scale miner=3 --scale webserver=2
# Or set NUM_MINERS and NUM_WEBSERVERS environment variables

services:
  # Redis for axum_rate_limiter (rate limiting state)
  redis:
    image: redis:7-alpine
    restart: unless-stopped
    volumes:
      - redis-data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 5s
      timeout: 3s
      retries: 20

  # Miner service - can be scaled to run multiple miners
  miner:
    build:
      context: /Users/bkunyiha/Rust/blockchain
      dockerfile: Dockerfile
    # Note: When scaling, only the first instance gets port mapping
    # Other instances use internal ports only (accessible via docker network)
    # To expose all instances, use port ranges or create separate services
    ports:
      - "2001:2001"  # First miner instance
    volumes:
      # Volume mounted at /app/data - each instance stores data in subdirectories
      # Instance 1: /app/data/data1, Instance 2: /app/data/data2, etc.
      # This ensures each node has its own isolated blockchain copy
      - miner-data:/app/data
      - miner-wallets:/app/wallets
    environment:
      # Node configuration
      # Note: NODE_IS_MINER=yes overrides the Dockerfile default (which is "no")
      # The default "no" is a safe fallback for standalone container runs
      - NODE_IS_MINER=yes
      - NODE_IS_WEB_SERVER=no
      # Connect nodes: "local" for seed node, or remote addresses
      # When SEQUENTIAL_STARTUP=yes, this will be auto-configured to previous node
      - NODE_CONNECT_NODES=${MINER_CONNECT_NODES:-local}
      # Mining address configuration (choose one):
      # Option 1: Use WALLET_ADDRESS_POOL (comma-separated) - each instance auto-selects by index
      # Option 2: Use NODE_MINING_ADDRESS directly for all instances
      # At least one must be set
      - WALLET_ADDRESS_POOL=${WALLET_ADDRESS_POOL:-}
      - NODE_MINING_ADDRESS=${NODE_MINING_ADDRESS:-}
      # Sequential startup: each node waits for previous node (default: yes)
      - SEQUENTIAL_STARTUP=${SEQUENTIAL_STARTUP:-yes}
      # Optional: Central node configuration
      - CENTERAL_NODE=${CENTERAL_NODE:-}
      # Wallet file path
      - WALLET_FILE=wallets/wallets.dat
    restart: unless-stopped
    healthcheck:
      # Health check for miners: check if P2P port is listening
      # Note: This is a simple TCP check, actual readiness is verified by wait script
      test: ["CMD-SHELL", "timeout 1 bash -c 'echo > /dev/tcp/localhost/2001' || exit 1"]
      interval: 10s
      timeout: 5s
      retries: 3
      start_period: 30s
    # Instance number is auto-detected from container name by entrypoint script

  # Webserver service - can be scaled to run multiple webservers
  webserver:
    build:
      context: /Users/bkunyiha/Rust/blockchain
      dockerfile: Dockerfile
    # Webservers depend on at least one miner being available
    depends_on:
      miner:
        condition: service_healthy
      redis:
        condition: service_healthy
    # Note: When scaling, only the first instance gets port mapping
    # Other instances use internal ports only (accessible via docker network)
    # To expose all instances, use port ranges or create separate services
    ports:
      - "8080:8080"  # First webserver instance - Web port
      - "2101:2001"  # First webserver instance - P2P port (mapped to 2101 to avoid conflict)
    volumes:
      # Volume mounted at /app/data - each instance stores data in subdirectories
      # Instance 1: /app/data/data1, Instance 2: /app/data/data2, etc.
      # This ensures each node has its own isolated blockchain copy
      - webserver-data:/app/data
      - webserver-wallets:/app/wallets
      # Rate limiter settings (used by axum_rate_limiter via RL_SETTINGS_PATH)
      - ./Settings.toml:/app/Settings.toml:ro
    environment:
      # Node configuration
      - NODE_IS_MINER=no
      - NODE_IS_WEB_SERVER=yes
      # Connect nodes: webservers connect to first miner by default
      # When SEQUENTIAL_STARTUP=yes, this will be auto-configured to previous node
      # Default connects to first miner (miner_1:2001)
      - NODE_CONNECT_NODES=${WEBSERVER_CONNECT_NODES:-miner_1:2001}
      # Sequential startup: each node waits for previous node (default: yes)
      - SEQUENTIAL_STARTUP=${SEQUENTIAL_STARTUP:-yes}
      # Mining address configuration (choose one):
      # Option 1: Use WALLET_ADDRESS_POOL (comma-separated) - each instance auto-selects by index
      # Option 2: Use NODE_MINING_ADDRESS directly for all instances
      # At least one must be set
      - WALLET_ADDRESS_POOL=${WALLET_ADDRESS_POOL:-}
      - NODE_MINING_ADDRESS=${NODE_MINING_ADDRESS:-}
      # API authentication keys
      - BITCOIN_API_ADMIN_KEY=${BITCOIN_API_ADMIN_KEY:-admin-secret}
      - BITCOIN_API_WALLET_KEY=${BITCOIN_API_WALLET_KEY:-wallet-secret}
      # Rate limiting configuration (axum_rate_limiter)
      - RL_SETTINGS_PATH=/app/Settings.toml
      # Optional: Central node configuration
      - CENTERAL_NODE=${CENTERAL_NODE:-}
      # Wallet file path
      - WALLET_FILE=wallets/wallets.dat
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/api/health/ready"]
      interval: 10s
      timeout: 5s
      retries: 3
      start_period: 30s
    # Instance number is auto-detected from container name by entrypoint script

volumes:
  redis-data:
    driver: local
  miner-data:
    driver: local
  miner-wallets:
    driver: local
  webserver-data:
    driver: local
  webserver-wallets:
    driver: local
```

---

## Listing 8.2: `ci/docker-compose/configs/docker-entrypoint.sh`

This is the **heart** of the Docker Compose deployment:

- determines instance number from container name,
- computes per-instance ports and per-instance data directories,
- applies the sequential startup policy,
- resolves hostnames into IP addresses (because Rust’s `SocketAddr` parsing does not accept hostnames),
- selects or creates a mining address,
- and finally executes `./blockchain startnode ...`.

Important to understand:

- The algorithm has to work across both **Docker Compose** naming (`project_service_1`) and **Kubernetes StatefulSet** naming (`miner-0`), which is why this script contains both code paths.
- “Complete method coverage” matters here: all helper functions (`resolve_hostname_to_ip`, `construct_prev_addr`, etc.) are what make the deployment deterministic and debuggable.

> **Methods involved**
> - `debug_log`
> - `resolve_hostname_to_ip`
> - `validate_and_fix_miner_address`
> - `construct_prev_addr`

```bash
#!/bin/bash
set -e

# Debug logging control (set DEBUG=1 to enable verbose logging)
DEBUG="${DEBUG:-0}"

# Helper function for debug logging
debug_log() {
    if [ "${DEBUG}" = "1" ]; then
        echo "DEBUG: $*" >&2
    fi
}

# Default values
# NODE_IS_MINER defaults to "no" (webserver mode) as a safe default:
# - If container is run directly without docker-compose, it won't accidentally start mining
# - docker-compose.yml explicitly sets NODE_IS_MINER=yes for miner service (overrides this default)
# - The entrypoint script also detects service type from container name, but this env var is what gets passed to the binary
NODE_IS_MINER="${NODE_IS_MINER:-no}"
NODE_IS_WEB_SERVER="${NODE_IS_WEB_SERVER:-yes}"

# Normalize NODE_CONNECT_NODES: handle empty string, whitespace-only, or unset
# Empty string should be treated as "local" (seed node)
if [ -z "${NODE_CONNECT_NODES}" ] || [ -z "$(echo "${NODE_CONNECT_NODES}" | xargs)" ]; then
    NODE_CONNECT_NODES="local"
else
    # Trim whitespace
    NODE_CONNECT_NODES=$(echo "${NODE_CONNECT_NODES}" | xargs)
    # If trimming resulted in empty string, default to "local"
    if [ -z "${NODE_CONNECT_NODES}" ]; then
        NODE_CONNECT_NODES="local"
    fi
fi

NODE_MINING_ADDRESS="${NODE_MINING_ADDRESS:-}"

# Helper function to resolve hostname:port to IP:port
# Docker service names with underscores (e.g., miner_1) need to be resolved to IP addresses
# because Rust's SocketAddr::from_str() doesn't accept hostnames with underscores
resolve_hostname_to_ip() {
    local addr="${1}"
    local max_retries="${2:-5}"  # Default to 5 retries
    local retry_delay="${3:-2}"  # Default to 2 seconds between retries
    
    # Validate input is not empty
    if [ -z "${addr}" ]; then
        echo "ERROR: resolve_hostname_to_ip called with empty address" >&2
        return 1
    fi
    
    # Trim whitespace
    addr=$(echo "${addr}" | xargs)
    
    # Validate again after trimming
    if [ -z "${addr}" ]; then
        echo "ERROR: resolve_hostname_to_ip called with empty address (after trimming whitespace)" >&2
        return 1
    fi
    
    # If it's already an IP address or "local", return as-is
    if [[ "${addr}" == "local" ]] || [[ "${addr}" =~ ^[0-23]+\.[0-23]+\.[0-23]+\.[0-23]+:[0-23]+$ ]]; then
        echo "${addr}"
        return 0
    fi
    
    # Extract hostname and port
    local hostname="${addr%%:*}"
    local port="${addr##*:}"
    
    # Validate port is numeric
    if ! [[ "${port}" =~ ^[0-23]+$ ]]; then
        echo "ERROR: Invalid port '${port}' in address '${addr}'" >&2
        return 1
    fi
    
    # Try multiple methods to resolve hostname to IP with retries
    local ip=""
    local attempt=0
    
    while [ ${attempt} -lt ${max_retries} ]; do
        attempt=$((attempt + 1))
        
        # Debug: Log resolution attempt
        if [ ${attempt} -gt 1 ]; then
            debug_log "Retry ${attempt}/${max_retries}: Resolving hostname '${hostname}' to IP address..."
            sleep ${retry_delay}
        else
            debug_log "Resolving hostname '${hostname}' to IP address (attempt ${attempt}/${max_retries})..."
        fi
        
        # Method 1: Try getent hosts (preferred, works with Docker's internal DNS)
        if command -v getent >/dev/null 2>&1; then
            debug_log "Trying getent hosts ${hostname}..."
            local getent_output
            getent_output=$(getent hosts "${hostname}" 2>&1)
            local getent_exit=$?
            
            if [ ${getent_exit} -eq 0 ]; then
                ip=$(echo "${getent_output}" | awk '{print $1}' | head -n1)
                debug_log "getent result: '${ip}'"
                if [ -n "${ip}" ] && [[ "${ip}" =~ ^[0-23]+\.[0-23]+\.[0-23]+\.[0-23]+$ ]]; then
                    debug_log "Successfully resolved ${hostname} to ${ip}"
                    echo "${ip}:${port}"
                    return 0
                fi
            else
                debug_log "getent failed with exit code ${getent_exit}"
            fi
        else
            debug_log "getent not available"
        fi
        
        # Method 2: Try nslookup (if getent failed and nslookup is available)
        if [ -z "${ip}" ] && command -v nslookup >/dev/null 2>&1; then
            debug_log "Trying nslookup ${hostname}..."
            ip=$(nslookup "${hostname}" 2>/dev/null | grep -A1 "Name:" | grep "Address:" | awk '{print $2}' | head -n1)
            if [ -n "${ip}" ] && [[ "${ip}" =~ ^[0-23]+\.[0-23]+\.[0-23]+\.[0-23]+$ ]]; then
                debug_log "Successfully resolved ${hostname} to ${ip} via nslookup"
                echo "${ip}:${port}"
                return 0
            fi
        fi
        
        # Method 3: Try host command (if both above failed and host is available)
        if [ -z "${ip}" ] && command -v host >/dev/null 2>&1; then
            debug_log "Trying host ${hostname}..."
            ip=$(host "${hostname}" 2>/dev/null | grep "has address" | awk '{print $4}' | head -n1)
            if [ -n "${ip}" ] && [[ "${ip}" =~ ^[0-23]+\.[0-23]+\.[0-23]+\.[0-23]+$ ]]; then
                debug_log "Successfully resolved ${hostname} to ${ip} via host"
                echo "${ip}:${port}"
                return 0
            fi
        fi
        
        # Method 4: Try ping with timeout (as last resort, works with Docker DNS)
        if [ -z "${ip}" ] && command -v ping >/dev/null 2>&1; then
            debug_log "Trying ping ${hostname}..."
            # Use ping to resolve hostname (ping resolves DNS but we just need the IP)
            # Extract IP from ping output: "PING miner_1 (172.18.0.2)"
            ip=$(ping -c 1 -W 1 "${hostname}" 2>/dev/null | grep -oE '\([0-23]+\.[0-23]+\.[0-23]+\.[0-23]+\)' | tr -d '()' | head -n1)
            if [ -n "${ip}" ] && [[ "${ip}" =~ ^[0-23]+\.[0-23]+\.[0-23]+\.[0-23]+$ ]]; then
                debug_log "Successfully resolved ${hostname} to ${ip} via ping"
                echo "${ip}:${port}"
                return 0
            fi
        fi
    done
    
    # If all resolution methods failed after retries, fail with error
    echo "ERROR: Failed to resolve hostname '${hostname}' to IP address after ${max_retries} attempts." >&2
    echo "ERROR: Tried methods: getent, nslookup, host, ping" >&2
    echo "ERROR: Docker service names should resolve via Docker's internal DNS." >&2
    echo "ERROR: This usually means:" >&2
    echo "ERROR:   1. The target service is not running yet" >&2
    echo "ERROR:   2. Docker DNS is not ready" >&2
    echo "ERROR:   3. The hostname is incorrect" >&2
    return 1
}

# Helper function to validate and fix miner_0 addresses for webservers
# Returns 0 if valid or fixed, 1 if invalid and cannot be fixed
validate_and_fix_miner_address() {
    local addr="${1}"
    local is_webserver="${2:-no}"
    
    # Check if address contains miner_0
    if [[ "${addr}" =~ miner_0 ]]; then
        if [ "${is_webserver}" = "yes" ]; then
            echo "ERROR: Invalid address '${addr}' - webservers cannot connect to miner_0" >&2
            echo "DEBUG: Overriding with miner_1:2001 for webserver" >&2
            echo "miner_1:2001"
            return 0
        else
            echo "ERROR: Cannot use miner_0 address '${addr}' - miners start at instance 1" >&2
            return 1
        fi
    fi
    
    # Extract hostname part to check for miner_0
    local hostname_only=$(echo "${addr}" | cut -d':' -f1)
    if [ "${hostname_only}" = "miner_0" ]; then
        if [ "${is_webserver}" = "yes" ]; then
            echo "ERROR: Hostname is miner_0: '${addr}'" >&2
            echo "DEBUG: Overriding with miner_1:2001 for webserver" >&2
            echo "miner_1:2001"
            return 0
        else
            echo "ERROR: Cannot use miner_0 hostname '${addr}'" >&2
            return 1
        fi
    fi
    
    # Address is valid
    echo "${addr}"
    return 0
}

# Helper function to construct PREV_ADDR for a node
construct_prev_addr() {
    local instance_number="${1}"
    local is_webserver="${2:-no}"
    local is_miner="${3:-no}"
    local wait_service_name="${4:-miner}"
    
    if [ "${is_webserver}" = "yes" ] && [ "${is_miner}" = "no" ]; then
        # Webservers always connect to miner_1
        echo "miner_1:2001"
    else
        # Miners connect to previous miner
        local prev_instance=$((instance_number - 1))
        if [ ${prev_instance} -lt 1 ]; then
            echo "ERROR: Cannot connect to miner_0 - miners start at instance 1" >&2
            return 1
        fi
        local prev_p2p_port=$((2001 + prev_instance - 1))
        local prev_hostname="${wait_service_name}_${prev_instance}"
        echo "${prev_hostname}:${prev_p2p_port}"
    fi
}

# Determine instance number from container name or environment variable
#
# Container Naming Pattern:
# Docker Compose automatically creates containers with names following this pattern:
#   <project>_<service>_<instance_number>
#
# Where:
#   - <project>: Defaults to directory name (e.g., "blockchain") or can be set via:
#     * COMPOSE_PROJECT_NAME environment variable
#     * -p/--project-name flag in docker-compose commands
#   - <service>: Service name from docker-compose.yml (e.g., "miner", "webserver")
#   - <instance_number>: Instance number when scaling (1, 2, 3, etc.)
#
# Examples:
#   - blockchain_miner_1 (project=blockchain, service=miner, instance=1)
#   - blockchain_miner_2 (project=blockchain, service=miner, instance=2)
#   - blockchain_webserver_1 (project=blockchain, service=webserver, instance=1)
#
# How HOSTNAME is Set:
# Docker automatically sets the HOSTNAME environment variable to match the container name.
# This happens in Docker's daemon code (not in this repository) when the container starts.
# The entrypoint script reads HOSTNAME to get the container name.
CONTAINER_NAME="${HOSTNAME:-}"
if [ -z "${INSTANCE_NUMBER:-}" ]; then
    # Try to extract instance number from container name
    # Pattern: <service>_<number> or <project>_<service>_<number>
    # Also supports Kubernetes StatefulSet pattern: miner-0, miner-1, etc.
    if [[ "${CONTAINER_NAME}" =~ _([0-23]+)$ ]]; then
        # Docker Compose pattern: blockchain_miner_1, blockchain_miner_2
        INSTANCE_NUMBER="${BASH_REMATCH[1]}"
    elif [[ "${CONTAINER_NAME}" =~ -([0-23]+)$ ]]; then
        # Kubernetes StatefulSet pattern: miner-0, miner-1, webserver-0, etc.
        # Extract ordinal and convert to instance number (0-based -> 1-based)
        ORDINAL="${BASH_REMATCH[1]}"
        INSTANCE_NUMBER=$((ORDINAL + 1))
    else
        # Default to 1 if we can't determine
        INSTANCE_NUMBER=1
    fi
fi

# Determine service name from container name
# Extract service name (miner or webserver) from container name
SERVICE_NAME_FROM_CONTAINER=""
if [[ "${CONTAINER_NAME}" =~ miner ]]; then
    SERVICE_NAME_FROM_CONTAINER="miner"
elif [[ "${CONTAINER_NAME}" =~ webserver ]]; then
    SERVICE_NAME_FROM_CONTAINER="webserver"
fi

# Determine service type (miner or webserver) from container name or environment
SERVICE_TYPE=""
if [[ "${CONTAINER_NAME}" =~ miner ]]; then
    SERVICE_TYPE="miner"
    # Miners: P2P ports start at 2001
    P2P_PORT=$((2001 + INSTANCE_NUMBER - 1))
elif [[ "${CONTAINER_NAME}" =~ webserver ]]; then
    SERVICE_TYPE="webserver"
    # Webservers: Web ports start at 8080, P2P ports start at 2101
    WEB_PORT=$((8080 + INSTANCE_NUMBER - 1))
    P2P_PORT=$((2101 + INSTANCE_NUMBER - 1))
else
    # Fallback: use environment variables to determine
    if [ "${NODE_IS_MINER}" = "yes" ]; then
        SERVICE_TYPE="miner"
        P2P_PORT=$((2001 + INSTANCE_NUMBER - 1))
    else
        SERVICE_TYPE="webserver"
        WEB_PORT=$((8080 + INSTANCE_NUMBER - 1))
        P2P_PORT=$((2101 + INSTANCE_NUMBER - 1))
    fi
fi

# Kubernetes mode: keep *container* ports stable.
#
# In Docker Compose we vary ports per instance on the host (and sometimes inside the container),
# but in Kubernetes each pod has its own IP, so all pods can (and should) listen on the same
# internal ports. Our manifests and probes assume:
# - miner P2P: 2001
# - webserver HTTP: 8080
# - webserver P2P: 2001
if [ -n "${POD_NAME:-}" ]; then
    if [ "${NODE_IS_MINER}" = "yes" ]; then
        P2P_PORT=2001
    fi
    if [ "${NODE_IS_WEB_SERVER}" = "yes" ] && [ "${NODE_IS_MINER}" = "no" ]; then
        WEB_PORT=8080
        P2P_PORT=2001
    fi
fi

# Instance-specific data directory
# Each instance gets its own isolated blockchain data directory
DATA_DIR="data${INSTANCE_NUMBER}"
BLOCKS_TREE="blocks${INSTANCE_NUMBER}"

# Base data directory (where volumes are mounted)
# Volume is mounted at /app/data, so we store instance data within it
BASE_DATA_DIR="/app/data"

# Instance-specific directory name (relative to base)
# This will be stored as /app/data/data1, /app/data/data2, etc.
INSTANCE_DATA_DIR_NAME="${DATA_DIR}"

# Full path to instance-specific data directory within the volume
INSTANCE_DATA_DIR="${BASE_DATA_DIR}/${INSTANCE_DATA_DIR_NAME}"

# Update environment variables for this instance
# TREE_DIR should be relative to current working directory (/app)
# So we use "data/data1", "data/data2", etc. to store within the mounted volume
export NODE_ADDR="0.0.0.0:${P2P_PORT}"
export TREE_DIR="data/${INSTANCE_DATA_DIR_NAME}"
export BLOCKS_TREE="${BLOCKS_TREE}"

# Create instance-specific data directory if it doesn't exist
# This ensures each node has its own isolated blockchain data within the volume
mkdir -p "${INSTANCE_DATA_DIR}"
echo "Using isolated blockchain data directory: ${INSTANCE_DATA_DIR}"
echo "  TREE_DIR=${TREE_DIR} (relative to /app)"
echo "  BLOCKS_TREE=${BLOCKS_TREE}"

# Auto-configure webservers to connect to first miner if NODE_CONNECT_NODES is "local" or empty
# This ensures webservers connect to miners, not act as seed nodes
# NOTE: For webserver instance 1, we set it to "local" here and let sequential startup handle
# the resolution after waiting for the miner to be ready. This ensures the miner is available
# before we try to resolve its hostname.
if [ "${NODE_IS_WEB_SERVER}" = "yes" ] && [ "${NODE_IS_MINER}" = "no" ]; then
    # Normalize empty string to "local" for consistency
    # For webservers, "local" means "wait for miner and connect to it" (handled by sequential startup)
    if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ] || [ "${NODE_CONNECT_NODES}" = "" ]; then
        # Set to "local" - sequential startup will handle waiting for miner and resolution
        NODE_CONNECT_NODES="local"
        echo "Webserver instance ${INSTANCE_NUMBER}: Will connect to miner (sequential startup will handle resolution)"
    else
        # Trim whitespace and validate it's not empty after trimming
        NODE_CONNECT_NODES=$(echo "${NODE_CONNECT_NODES}" | xargs)
        if [ -z "${NODE_CONNECT_NODES}" ]; then
            echo "WARNING: NODE_CONNECT_NODES is empty after trimming whitespace" >&2
            echo "WARNING: Setting to 'local' - sequential startup will handle resolution" >&2
            NODE_CONNECT_NODES="local"
        else
            # If it's already an IP address, keep it as-is
            if [[ "${NODE_CONNECT_NODES}" =~ ^[0-23]+\.[0-23]+\.[0-23]+\.[0-23]+:[0-23]+$ ]]; then
                echo "Webserver instance ${INSTANCE_NUMBER}: Using configured IP address: ${NODE_CONNECT_NODES}"
            else
                # Contains a hostname - sequential startup will wait for service and resolve it
                echo "Webserver instance ${INSTANCE_NUMBER}: Will resolve hostname '${NODE_CONNECT_NODES}' after waiting for service"
                # Don't resolve yet - let sequential startup handle it after waiting
            fi
        fi
    fi
fi

# Kubernetes StatefulSet Support: Auto-configure NODE_CONNECT_NODES for miners
# StatefulSet pods have stable names: miner-0, miner-1, miner-2, etc.
# Pods can connect using headless service DNS: miner-0.miner-headless.blockchain.svc.cluster.local
if [ "${NODE_IS_MINER}" = "yes" ] && [ -n "${POD_NAME}" ]; then
    # Extract ordinal from pod name (e.g., miner-0 -> 0, miner-1 -> 1)
    if [[ "${POD_NAME}" =~ -([0-23]+)$ ]]; then
        ORDINAL="${BASH_REMATCH[1]}"
        
        if [ "${ORDINAL}" = "0" ]; then
            # First miner (miner-0) starts as seed node
            if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ]; then
                NODE_CONNECT_NODES="local"
                echo "Kubernetes StatefulSet: First miner (miner-0) starting as seed node"
            fi
        else
            # Subsequent miners connect to previous miner via headless service
            PREV_ORDINAL=$((ORDINAL - 1))
            if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ]; then
                # Use Kubernetes DNS format for headless service
                NODE_CONNECT_NODES="miner-${PREV_ORDINAL}.miner-headless.blockchain.svc.cluster.local:2001"
                echo "Kubernetes StatefulSet: Miner ${ORDINAL} connecting to miner-${PREV_ORDINAL}"
            fi
        fi
    fi
fi

# Sequential startup: Wait for previous node if enabled (Docker Compose mode)
# For webservers, we always wait for miners (even instance 1) to ensure miner is ready
# For miners, only wait if instance number > 1
if [ "${SEQUENTIAL_STARTUP:-yes}" = "yes" ]; then
    # Webservers always wait for miners (even instance 1)
    # Miners only wait if not the first instance
    if [ "${NODE_IS_WEB_SERVER}" = "yes" ] && [ "${NODE_IS_MINER}" = "no" ]; then
        # All webservers wait for miners
        SHOULD_WAIT=true
    elif [ "${NODE_IS_MINER}" = "yes" ] && [ "${INSTANCE_NUMBER}" -gt 1 ]; then
        # Miners wait only if not first instance
        SHOULD_WAIT=true
    else
        SHOULD_WAIT=false
    fi
    
    if [ "${SHOULD_WAIT}" = "true" ]; then
        # Skip sequential startup logic if we're in Kubernetes StatefulSet mode
        # (StatefulSet handles ordered startup automatically)
        if [ -z "${POD_NAME}" ] || [[ ! "${POD_NAME}" =~ -([0-23]+)$ ]]; then
            if [ "${NODE_IS_WEB_SERVER}" = "yes" ] && [ "${NODE_IS_MINER}" = "no" ]; then
                echo "Sequential startup enabled: Webserver waiting for miner to be ready..."
            else
                echo "Sequential startup enabled: Waiting for previous node..."
            fi
        
        # Determine service name for wait script
        # Miners connect to previous miner
        # Webservers ALWAYS connect to miners (never to other webservers)
        WAIT_SERVICE_NAME="miner"
        
        # Run wait script and capture output
        if [ -f "/app/wait-for-node.sh" ]; then
            # For webservers, we need to pass a higher instance number so the wait script
            # looks for miner_1 (not miner_0). The wait script calculates PREV_INSTANCE = INSTANCE_NUMBER - 1
            # So for webserver instance 1, pass instance 2 to get PREV_INSTANCE = 1 (miner_1)
            WAIT_INSTANCE_NUMBER="${INSTANCE_NUMBER}"
            if [ "${NODE_IS_WEB_SERVER}" = "yes" ] && [ "${NODE_IS_MINER}" = "no" ]; then
                # Webservers always wait for miners, so use instance number that will result in miner_1
                # If webserver is instance 1, pass 2 to get miner_1 (2-1=1)
                WAIT_INSTANCE_NUMBER=$((INSTANCE_NUMBER + 1))
            fi
            debug_log "Calling wait script with:"
            debug_log "  WAIT_SERVICE_NAME=${WAIT_SERVICE_NAME}"
            debug_log "  WAIT_INSTANCE_NUMBER=${WAIT_INSTANCE_NUMBER}"
            debug_log "  INSTANCE_NUMBER=${INSTANCE_NUMBER}"
            debug_log "  NODE_IS_WEB_SERVER=${NODE_IS_WEB_SERVER}"
            WAIT_OUTPUT=$(/app/wait-for-node.sh "${WAIT_SERVICE_NAME}" "${WAIT_INSTANCE_NUMBER}" "${P2P_PORT}" "${NODE_IS_WEB_SERVER}" 2>&1)
            WAIT_EXIT_CODE=$?
            debug_log "Wait script exit code: ${WAIT_EXIT_CODE}"
            
            # Display wait script output
            echo "${WAIT_OUTPUT}"
            
            if [ ${WAIT_EXIT_CODE} -eq 0 ]; then
                # Extract PREV_NODE_ADDRESS from output if present
                PREV_ADDR=$(echo "${WAIT_OUTPUT}" | grep "PREV_NODE_ADDRESS=" | cut -d'=' -f2)
                debug_log "Extracted PREV_ADDR from wait script: '${PREV_ADDR}'"
                
                # Construct previous node address if not provided
                if [ -z "${PREV_ADDR}" ]; then
                    debug_log "PREV_ADDR not found in wait script output, constructing..."
                    PREV_ADDR=$(construct_prev_addr "${INSTANCE_NUMBER}" "${NODE_IS_WEB_SERVER}" "${NODE_IS_MINER}" "${WAIT_SERVICE_NAME}")
                    if [ $? -ne 0 ]; then
                        exit 1
                    fi
                    debug_log "Constructed PREV_ADDR: ${PREV_ADDR}"
                else
                    # Validate extracted address - webservers should never get miner_0
                    PREV_ADDR=$(validate_and_fix_miner_address "${PREV_ADDR}" "${NODE_IS_WEB_SERVER}")
                    if [ $? -ne 0 ]; then
                        exit 1
                    fi
                fi
                
                debug_log "Final PREV_ADDR before resolution: '${PREV_ADDR}'"
                
                # Resolve Docker service name to IP address
                # Docker service names with underscores (e.g., miner_1) need to be resolved to IP
                # For Docker Compose, "miner_1" doesn't resolve, but "miner" does
                # So we convert miner_1 to miner for resolution purposes
                RESOLVE_ADDR="${PREV_ADDR}"
                if [[ "${PREV_ADDR}" =~ ^miner_([0-23]+): ]]; then
                    # Extract instance number and use "miner" service name for resolution
                    INSTANCE_NUM="${BASH_REMATCH[1]}"
                    PORT_PART="${PREV_ADDR##*:}"
                    RESOLVE_ADDR="miner:${PORT_PART}"
                    debug_log "Converting ${PREV_ADDR} to ${RESOLVE_ADDR} for Docker Compose DNS resolution"
                fi
                
                if ! PREV_ADDR_RESOLVED=$(resolve_hostname_to_ip "${RESOLVE_ADDR}"); then
                    echo "ERROR: Failed to resolve previous node address '${RESOLVE_ADDR}' (from '${PREV_ADDR}')" >&2
                    exit 1
                fi
                
                # Use the previous node address for NODE_CONNECT_NODES
                # Miners: connect to previous miner
                # Webservers: always connect to miners (first miner for all webservers)
                if [ "${NODE_IS_MINER}" = "yes" ]; then
                    # Miners connect to previous miner
                    if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ]; then
                        NODE_CONNECT_NODES="${PREV_ADDR_RESOLVED}"
                        echo "  Auto-configured connect nodes: ${PREV_ADDR} -> ${NODE_CONNECT_NODES}"
                    else
                        # Resolve configured connect nodes if they contain hostnames
                        if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "${NODE_CONNECT_NODES}"); then
                            echo "ERROR: Failed to resolve configured connect nodes '${NODE_CONNECT_NODES}'" >&2
                            exit 1
                        fi
                        echo "  Using configured connect nodes: ${NODE_CONNECT_NODES}"
                    fi
                else
                    # Webservers always connect to first miner (miner_1:2001)
                    # Sequential startup ensures miner_1 is ready before webservers start
                    # Use "miner:2001" for Docker Compose DNS resolution
                    if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "miner:2001"); then
                        echo "ERROR: Failed to resolve miner:2001 for webserver" >&2
                        exit 1
                    fi
                    echo "  Auto-configured webserver to connect to first miner: miner:2001 -> ${NODE_CONNECT_NODES}"
                fi
            else
                echo "  ERROR: Wait script failed (exit code ${WAIT_EXIT_CODE}), but continuing startup..." >&2
                # Even if wait script failed, try to extract PREV_NODE_ADDRESS from output
                # (in case it outputted the address before failing)
                PREV_ADDR=$(echo "${WAIT_OUTPUT}" | grep "PREV_NODE_ADDRESS=" | cut -d'=' -f2)
                debug_log "Extracted PREV_ADDR from failed wait script: '${PREV_ADDR}'"
                
                # If we didn't get an address from wait script, construct it
                if [ -z "${PREV_ADDR}" ]; then
                    debug_log "Constructing PREV_ADDR after wait script failure..."
                    PREV_ADDR=$(construct_prev_addr "${INSTANCE_NUMBER}" "${NODE_IS_WEB_SERVER}" "${NODE_IS_MINER}" "${WAIT_SERVICE_NAME}")
                    if [ $? -ne 0 ]; then
                        exit 1
                    fi
                    debug_log "Constructed PREV_ADDR: ${PREV_ADDR}"
                else
                    # Validate extracted address - webservers should never get miner_0
                    PREV_ADDR=$(validate_and_fix_miner_address "${PREV_ADDR}" "${NODE_IS_WEB_SERVER}")
                    if [ $? -ne 0 ]; then
                        exit 1
                    fi
                fi
                
                debug_log "PREV_ADDR after wait script failure handling: '${PREV_ADDR}'"
                
                # Resolve Docker service name to IP address
                # For Docker Compose, "miner_1" doesn't resolve, but "miner" does
                RESOLVE_ADDR="${PREV_ADDR}"
                if [[ "${PREV_ADDR}" =~ ^miner_([0-23]+): ]]; then
                    INSTANCE_NUM="${BASH_REMATCH[1]}"
                    PORT_PART="${PREV_ADDR##*:}"
                    RESOLVE_ADDR="miner:${PORT_PART}"
                    debug_log "Converting ${PREV_ADDR} to ${RESOLVE_ADDR} for Docker Compose DNS resolution"
                fi
                
                if ! PREV_ADDR_RESOLVED=$(resolve_hostname_to_ip "${RESOLVE_ADDR}"); then
                    echo "ERROR: Failed to resolve previous node address '${RESOLVE_ADDR}' (from '${PREV_ADDR}')" >&2
                    exit 1
                fi
                
                # Use the previous node address for NODE_CONNECT_NODES
                if [ "${NODE_IS_MINER}" = "yes" ]; then
                    if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ]; then
                        NODE_CONNECT_NODES="${PREV_ADDR_RESOLVED}"
                        echo "  Auto-configured connect nodes: ${PREV_ADDR} -> ${NODE_CONNECT_NODES}"
                    fi
                else
                    # Webservers always connect to first miner (miner_1:2001)
                    # Use "miner:2001" for Docker Compose DNS resolution
                    if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "miner:2001"); then
                        echo "ERROR: Failed to resolve miner:2001 for webserver" >&2
                        exit 1
                    fi
                    echo "  Auto-configured webserver to connect to first miner: miner:2001 -> ${NODE_CONNECT_NODES}"
                fi
            fi
        else
            echo "  Warning: wait-for-node.sh not found, skipping wait"
            # Still construct node address even without wait script
            if [ "${NODE_IS_MINER}" = "yes" ]; then
                # Miners connect to previous miner
                if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ]; then
                    PREV_ADDR=$(construct_prev_addr "${INSTANCE_NUMBER}" "${NODE_IS_WEB_SERVER}" "${NODE_IS_MINER}" "miner")
                    if [ $? -ne 0 ]; then
                        exit 1
                    fi
                    if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "${PREV_ADDR}"); then
                        echo "ERROR: Failed to resolve previous miner address '${PREV_ADDR}' (no wait)" >&2
                        exit 1
                    fi
                    echo "  Auto-configured connect nodes (no wait): ${PREV_ADDR} -> ${NODE_CONNECT_NODES}"
                else
                    # Resolve configured connect nodes if they contain hostnames
                    if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "${NODE_CONNECT_NODES}"); then
                        echo "ERROR: Failed to resolve configured connect nodes '${NODE_CONNECT_NODES}' (no wait)" >&2
                        exit 1
                    fi
                    echo "  Using configured connect nodes (no wait): ${NODE_CONNECT_NODES}"
                fi
            else
                # Webservers always connect to first miner (miner_1, not miner_0)
                # Use "miner:2001" for Docker Compose DNS resolution
                if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ]; then
                    if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "miner:2001"); then
                        echo "ERROR: Failed to resolve miner:2001 for webserver (no wait)" >&2
                        exit 1
                    fi
                    echo "  Auto-configured webserver to connect to first miner (no wait): miner:2001 -> ${NODE_CONNECT_NODES}"
                else
                    # Resolve configured connect nodes if they contain hostnames
                    if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "${NODE_CONNECT_NODES}"); then
                        echo "ERROR: Failed to resolve configured connect nodes '${NODE_CONNECT_NODES}' (no wait)" >&2
                        exit 1
                    fi
                    echo "  Using configured connect nodes (no wait): ${NODE_CONNECT_NODES}"
                fi
            fi
        fi
        fi  # Close POD_NAME check
    fi  # Close SHOULD_WAIT check
else
    # Sequential startup disabled - for webservers, still try to wait for miner if instance 1
    # IMPORTANT: This Docker-Compose-style wait logic should NOT run in Kubernetes.
    # In Kubernetes, we use initContainers (and/or readiness probes) for ordering and service availability.
    if [ -n "${POD_NAME:-}" ]; then
        debug_log "Kubernetes mode detected (POD_NAME set) - skipping Docker Compose sequential wait logic"
    else
    if [ "${NODE_IS_WEB_SERVER}" = "yes" ] && [ "${NODE_IS_MINER}" = "no" ] && [ "${INSTANCE_NUMBER}" -eq 1 ]; then
        echo "Sequential startup disabled, but webserver instance 1 will wait for miner..."
        if [ -f "/app/wait-for-node.sh" ]; then
            # For webservers, we always wait for miner_1, not a previous webserver instance
            # Pass instance number 2 so wait script looks for miner_1 (2-1=1)
            WAIT_OUTPUT=$(/app/wait-for-node.sh "miner" "2" "${P2P_PORT}" "yes" 2>&1)
            WAIT_EXIT_CODE=$?
            echo "${WAIT_OUTPUT}"
            if [ ${WAIT_EXIT_CODE} -eq 0 ]; then
                PREV_ADDR=$(echo "${WAIT_OUTPUT}" | grep "PREV_NODE_ADDRESS=" | cut -d'=' -f2)
                if [ -z "${PREV_ADDR}" ]; then
                    PREV_ADDR="miner_1:2001"
                else
                    PREV_ADDR=$(validate_and_fix_miner_address "${PREV_ADDR}" "yes")
                    if [ $? -ne 0 ]; then
                        exit 1
                    fi
                fi
                if [ "${NODE_CONNECT_NODES}" = "local" ]; then
                    if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "${PREV_ADDR}"); then
                        echo "ERROR: Failed to resolve miner address '${PREV_ADDR}' for webserver instance 1" >&2
                        exit 1
                    fi
                    echo "Webserver instance 1: Resolved miner address: ${PREV_ADDR} -> ${NODE_CONNECT_NODES}"
                fi
            else
                echo "WARNING: Wait script failed, but continuing..." >&2
            fi
        fi
    fi
    fi
fi

# Determine wallet address from pool or direct assignment
# Option 1: Use WALLET_ADDRESS_POOL (comma-separated list) - each instance picks by index
# Option 2: Use NODE_MINING_ADDRESS directly

# Debug: Log received environment variables
debug_log "WALLET_ADDRESS_POOL='${WALLET_ADDRESS_POOL}'"
debug_log "NODE_MINING_ADDRESS='${NODE_MINING_ADDRESS}'"
debug_log "INSTANCE_NUMBER=${INSTANCE_NUMBER}"

if [ -n "${WALLET_ADDRESS_POOL}" ]; then
    # Convert comma-separated list to array
    IFS=',' read -ra ADDRESSES <<< "${WALLET_ADDRESS_POOL}"
    # Select address based on instance number (1-indexed, so subtract 1)
    INDEX=$((INSTANCE_NUMBER - 1))
    if [ ${INDEX} -lt ${#ADDRESSES[@]} ]; then
        NODE_MINING_ADDRESS="${ADDRESSES[${INDEX}]}"
        echo "Selected address from pool (instance ${INSTANCE_NUMBER}, index ${INDEX}): ${NODE_MINING_ADDRESS}"
    else
        echo "ERROR: Not enough addresses in pool for instance ${INSTANCE_NUMBER}"
        echo "  Pool size: ${#ADDRESSES[@]}, Required index: ${INDEX}"
        echo "  Available addresses: ${WALLET_ADDRESS_POOL}"
        exit 1
    fi
fi

# Auto-generate a mining address if none is provided (Kubernetes-friendly).
#
# Why: `startnode` requires a valid wallet address argument. In Kubernetes we prefer not to
# force users to pre-generate and inject addresses. Instead, if no address is provided, we
# create one in the container and persist it to the wallet volume.
#
# We also treat the common placeholder "your-wallet-address-here" as "unset".
PLACEHOLDER_MINING_ADDR="your-wallet-address-here"
MINING_ADDR_FILE=""

# Determine wallet directory from WALLET_FILE (default: wallets.dat in /app).
WALLET_FILE_PATH="${WALLET_FILE:-wallets.dat}"
WALLET_DIR="/app/$(dirname "${WALLET_FILE_PATH}")"
if [ "${WALLET_DIR}" = "/app/." ]; then
    WALLET_DIR="/app"
fi
MINING_ADDR_FILE="${WALLET_DIR}/mining_address.txt"

if [ "${NODE_MINING_ADDRESS}" = "${PLACEHOLDER_MINING_ADDR}" ]; then
    NODE_MINING_ADDRESS=""
fi

if [ -z "${NODE_MINING_ADDRESS}" ]; then
    # Prefer previously persisted mining address (stable across restarts).
    if [ -f "${MINING_ADDR_FILE}" ]; then
        NODE_MINING_ADDRESS="$(cat "${MINING_ADDR_FILE}" | head -n1 | xargs || true)"
    fi
fi

if [ -z "${NODE_MINING_ADDRESS}" ]; then
    echo "NODE_MINING_ADDRESS not provided; creating a new wallet address for mining..."
    echo "  Wallet file: /app/${WALLET_FILE_PATH}"
    echo "  Mining address cache: ${MINING_ADDR_FILE}"

    CREATE_OUTPUT=$(/app/blockchain createwallet 2>&1 || true)
    # The CLI prints: "Your new address: <ADDR>"
    NEW_ADDR=$(echo "${CREATE_OUTPUT}" | sed -n 's/.*Your new address: \([^[:space:]]\+\).*/\1/p' | tail -n1)

    if [ -z "${NEW_ADDR}" ]; then
        echo "ERROR: Failed to create wallet address. Raw output:" >&2
        echo "${CREATE_OUTPUT}" >&2
        exit 1
    fi

    NODE_MINING_ADDRESS="${NEW_ADDR}"
    mkdir -p "${WALLET_DIR}"
    echo "${NODE_MINING_ADDRESS}" > "${MINING_ADDR_FILE}"
    echo "Generated mining address: ${NODE_MINING_ADDRESS}"
fi

# Final resolution: Ensure NODE_CONNECT_NODES doesn't contain hostnames with underscores
# This handles cases where NODE_CONNECT_NODES was set via environment variables (e.g., docker-compose.yml)
# and might contain Docker service names like "miner_1:2001"
# This is a critical safety check - Rust cannot parse hostnames, only IP addresses

# Normalize empty or whitespace-only values
if [ -z "${NODE_CONNECT_NODES}" ] || [ -z "$(echo "${NODE_CONNECT_NODES}" | xargs)" ]; then
    echo "WARNING: NODE_CONNECT_NODES is empty or whitespace-only, defaulting to 'local'" >&2
    NODE_CONNECT_NODES="local"
fi

# Trim whitespace
NODE_CONNECT_NODES=$(echo "${NODE_CONNECT_NODES}" | xargs)

# Validate it's still not empty after trimming
if [ -z "${NODE_CONNECT_NODES}" ]; then
    echo "ERROR: NODE_CONNECT_NODES is empty after normalization" >&2
    echo "ERROR: Cannot proceed without a valid connect nodes value" >&2
    exit 1
fi

if [ "${NODE_CONNECT_NODES}" != "local" ]; then
    # Check if it's already a valid IP:port format
    if [[ ! "${NODE_CONNECT_NODES}" =~ ^[0-23]+\.[0-23]+\.[0-23]+\.[0-23]+:[0-23]+$ ]]; then
        # Contains a hostname (not "local" and not an IP address), resolve it
        echo "Final resolution: Resolving NODE_CONNECT_NODES '${NODE_CONNECT_NODES}' to IP address..."
        echo "  WARNING: Hostname detected - must resolve to IP before passing to Rust binary" >&2
        
        if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "${NODE_CONNECT_NODES}"); then
            echo "ERROR: Final resolution failed for NODE_CONNECT_NODES '${NODE_CONNECT_NODES}'" >&2
            echo "ERROR: Cannot proceed - Rust binary requires IP address, not hostname" >&2
            exit 1
        fi
        
        # Verify the result is actually an IP address
        if [[ ! "${NODE_CONNECT_NODES}" =~ ^[0-23]+\.[0-23]+\.[0-23]+\.[0-23]+:[0-23]+$ ]]; then
            echo "ERROR: Resolution returned invalid format: '${NODE_CONNECT_NODES}'" >&2
            echo "ERROR: Expected IP:port format (e.g., 172.18.0.2:2001)" >&2
            exit 1
        fi
        
        echo "Final resolution result: ${NODE_CONNECT_NODES}"
    else
        echo "Final resolution: NODE_CONNECT_NODES is already an IP address: ${NODE_CONNECT_NODES}"
    fi
else
    echo "Final resolution: NODE_CONNECT_NODES is 'local' (seed node mode)"
fi

# Final validation: Double-check that NODE_CONNECT_NODES is valid before building command
# This is a critical safety check to prevent Rust parse errors
if [ "${NODE_CONNECT_NODES}" != "local" ]; then
    if [[ ! "${NODE_CONNECT_NODES}" =~ ^[0-23]+\.[0-23]+\.[0-23]+\.[0-23]+:[0-23]+$ ]]; then
        echo "ERROR: CRITICAL - NODE_CONNECT_NODES contains invalid value: '${NODE_CONNECT_NODES}'" >&2
        echo "ERROR: Expected 'local' or IP:port format (e.g., 172.18.0.2:2001)" >&2
        echo "ERROR: Hostnames like 'miner_1:2001' cannot be passed to Rust binary" >&2
        echo "ERROR: This indicates a bug in the entrypoint script - resolution should have occurred earlier" >&2
        exit 1
    fi
fi

# Build the command
# Format: startnode <is_miner> <is_web_server> <connect_nodes> -- <mining_address>
CMD="./blockchain startnode ${NODE_IS_MINER} ${NODE_IS_WEB_SERVER} ${NODE_CONNECT_NODES} -- ${NODE_MINING_ADDRESS}"

# Debug: Show the exact command that will be executed
debug_log "Command to execute: ${CMD}"

# Log instance configuration
echo "=========================================="
echo "Starting blockchain node"
echo "  Service Type: ${SERVICE_TYPE}"
echo "  Instance Number: ${INSTANCE_NUMBER}"
echo "  Container Name: ${CONTAINER_NAME}"
echo "  Mode: miner=${NODE_IS_MINER}, webserver=${NODE_IS_WEB_SERVER}"
echo "  P2P Port: ${P2P_PORT}"
if [ "${NODE_IS_WEB_SERVER}" = "yes" ]; then
    echo "  Web Port: ${WEB_PORT}"
fi
echo "  Data Directory: ${INSTANCE_DATA_DIR} (isolated per instance)"
echo "  TREE_DIR: ${TREE_DIR}"
echo "  Connect Nodes: ${NODE_CONNECT_NODES}"
echo "  Mining Address: ${NODE_MINING_ADDRESS}"
echo "=========================================="

# Execute the command
exec $CMD
```

---

## Listing 8.3: `ci/docker-compose/configs/wait-for-node.sh`

This script implements the “**sequential startup**” wait step for Docker Compose:

- for miners: “wait for a previous miner instance to be listening on its P2P port,”
- for webservers: “wait for miner_1 to be listening (never miner_0).”

Important to understand:

- It never returns `miner_0`. This is enforced both by parameter validation and by loop bounds.
- It outputs `PREV_NODE_ADDRESS=...` so the entrypoint can capture and act on it.

> **Methods involved**
> - Artifact: `wait-for-node.sh` (sequential startup algorithm)

```bash
#!/bin/bash
# Wait for a node to be ready before starting this node
# Usage: wait-for-node.sh <wait_service_name> <instance_number> <port> [is_webserver]

set -e

WAIT_SERVICE_NAME="${1}"
INSTANCE_NUMBER="${2}"
PORT="${3}"
IS_WEBSERVER="${4:-no}"

# Validate required parameters
if [ -z "${WAIT_SERVICE_NAME}" ] || [ -z "${INSTANCE_NUMBER}" ] || [ -z "${PORT}" ]; then
    echo "ERROR: Missing required parameters" >&2
    echo "Usage: wait-for-node.sh <wait_service_name> <instance_number> <port> [is_webserver]" >&2
    exit 1
fi

# Validate INSTANCE_NUMBER is numeric
if ! [[ "${INSTANCE_NUMBER}" =~ ^[0-23]+$ ]]; then
    echo "ERROR: INSTANCE_NUMBER must be numeric, got: '${INSTANCE_NUMBER}'" >&2
    exit 1
fi

# Calculate previous instance number
PREV_INSTANCE=$((INSTANCE_NUMBER - 1))

# CRITICAL: Ensure PREV_INSTANCE is never less than 1 (miners start at instance 1)
# This prevents miner_0 from ever being used
if [ ${PREV_INSTANCE} -lt 1 ]; then
    echo "ERROR: PREV_INSTANCE calculated as ${PREV_INSTANCE} (from INSTANCE_NUMBER=${INSTANCE_NUMBER})" >&2
    echo "ERROR: Miners start at instance 1, so PREV_INSTANCE must be >= 1" >&2
    if [ "${IS_WEBSERVER}" = "yes" ]; then
        echo "DEBUG: For webserver, forcing PREV_INSTANCE to 1 (miner_1)" >&2
        PREV_INSTANCE=1
    else
        echo "ERROR: Cannot proceed with PREV_INSTANCE < 1" >&2
        exit 1
    fi
fi

# If this is instance 1 and NOT a webserver, no need to wait
# Webservers always wait for miners (even instance 1 waits for miner_1)
if [ "${INSTANCE_NUMBER}" -eq 1 ] && [ "${IS_WEBSERVER}" != "yes" ]; then
    echo "Instance ${INSTANCE_NUMBER}: First instance (not webserver), no need to wait"
    exit 0
fi

# Determine previous node's hostname and port
# Docker Compose creates containers with names like: blockchain_miner_1, blockchain_miner_2
# For miners: iterate backwards through miner instances until we find one that exists
# For non-miners: use the service name and previous instance directly

PREV_HOSTNAME=""
PREV_PORT=""
READY=false

# Determine which node to wait for and wait for it to be ready
if [[ "${WAIT_SERVICE_NAME}" == *"miner"* ]]; then
    # For miners: iterate backwards through miner instances and wait for each one until ready
    # This handles cases where some miner instances don't exist (e.g., only miner_1 and miner_3)
    if [ "${IS_WEBSERVER}" = "yes" ]; then
        echo "Instance ${INSTANCE_NUMBER}: Searching for available miner (webserver connecting to miner)..."
    else
        echo "Instance ${INSTANCE_NUMBER}: Searching for available miner (miner connecting to previous miner)..."
    fi
    
    CHECK_INSTANCE=${PREV_INSTANCE}
    
    # Ensure we never check miner_0 (miners start at instance 1)
    if [ ${CHECK_INSTANCE} -lt 1 ]; then
        CHECK_INSTANCE=1
    fi
    
    # Combined loop: iterate backwards through miner instances and wait for each one
    # Stop at miner_1 (never check miner_0)
    # Docker Compose uses service names for DNS, so we use "miner" as the hostname
    # The port is calculated based on instance number
    while [ ${CHECK_INSTANCE} -ge 1 ]; do
        # Use service name "miner" for Docker Compose DNS resolution
        # Docker Compose automatically resolves service names to the appropriate container
        CHECK_HOSTNAME="miner"
        CHECK_PORT=$((2001 + CHECK_INSTANCE - 1))
        
        echo "  Checking miner instance ${CHECK_INSTANCE}: ${CHECK_HOSTNAME}:${CHECK_PORT}..."
        
        PREV_HOSTNAME="${CHECK_HOSTNAME}"
        PREV_PORT="${CHECK_PORT}"
        
        # Wait for this miner instance to be ready
        MAX_ATTEMPTS=60
        ATTEMPT=0
        
        while [ ${ATTEMPT} -lt ${MAX_ATTEMPTS} ]; do
            # Check if port is listening and ready
            # Try multiple methods for port checking (for compatibility across different environments)
            if command -v nc >/dev/null 2>&1; then
                # Method 1: netcat (preferred, most reliable)
                if nc -z -w 1 "${PREV_HOSTNAME}" "${PREV_PORT}" 2>/dev/null; then
                    READY=true
                    break
                fi
            elif command -v timeout >/dev/null 2>&1 && [ -c /dev/tcp ]; then
                # Method 2: timeout + /dev/tcp (works on most Linux systems)
                if timeout 1 bash -c "echo > /dev/tcp/${PREV_HOSTNAME}/${PREV_PORT}" 2>/dev/null; then
                    READY=true
                    break
                fi
            else
                # Method 3: /dev/tcp fallback (works on bash with /dev/tcp support)
                if (echo > /dev/tcp/${PREV_HOSTNAME}/${PREV_PORT}) 2>/dev/null; then
                    READY=true
                    break
                fi
            fi
            
            ATTEMPT=$((ATTEMPT + 1))
            # Log progress: first 5 attempts, then every 10th attempt
            if [ ${ATTEMPT} -le 5 ] || [ $((ATTEMPT % 10)) -eq 0 ]; then
                echo "    Attempt ${ATTEMPT}/${MAX_ATTEMPTS}: Waiting for ${PREV_HOSTNAME}:${PREV_PORT}..."
            fi
            sleep 2
        done
        
        if [ "${READY}" = "true" ]; then
            echo "  Miner ${PREV_HOSTNAME}:${PREV_PORT} is ready!"
            echo "Instance ${INSTANCE_NUMBER}: Previous node is ready!"
            # Output the connect nodes address for use by the entrypoint
            # Format: PREV_NODE_ADDRESS=hostname:port
            # Use miner_${CHECK_INSTANCE} format for the address (entrypoint will resolve it)
            OUTPUT_HOSTNAME="miner_${CHECK_INSTANCE}"
            echo "PREV_NODE_ADDRESS=${OUTPUT_HOSTNAME}:${PREV_PORT}"
            # Also export for potential use
            export PREV_NODE_ADDRESS="${OUTPUT_HOSTNAME}:${PREV_PORT}"
            exit 0
        else
            echo "  Miner ${PREV_HOSTNAME}:${PREV_PORT} did not become ready, trying next instance..."
            READY=false
            CHECK_INSTANCE=$((CHECK_INSTANCE - 1))
            # Never check miner_0 - stop at miner_1
            if [ ${CHECK_INSTANCE} -lt 1 ]; then
                echo "  ERROR: Reached miner_0 - miners start at instance 1" >&2
                break
            fi
        fi
    done
    
    # If we exit the loop without finding a ready miner, it's an error
    if [ "${READY}" != "true" ]; then
        echo "ERROR: No available miner found or ready (checked instances ${PREV_INSTANCE} down to 1)" >&2
        echo "ERROR: This usually means:" >&2
        echo "ERROR:   1. No miners are running" >&2
        echo "ERROR:   2. Miners are not listening on expected ports" >&2
        echo "ERROR:   3. Network connectivity issues" >&2
        exit 1
    fi
else
    # Error: WAIT_SERVICE_NAME should always be "miner" in normal operation
    echo "ERROR: WAIT_SERVICE_NAME is '${WAIT_SERVICE_NAME}', but expected 'miner'" >&2
    echo "  This indicates a bug in docker-entrypoint.sh or manual script invocation with wrong parameters" >&2
    echo "  Miners and webservers should always wait for miners (WAIT_SERVICE_NAME='miner')" >&2
    exit 1
fi
```

---

## Listing 8.4: `ci/docker-compose/configs/docker-compose.scale.sh`

This helper script provides the repo’s recommended scaling interface:

- it generates a `docker-compose.override.yml` containing port mappings for **all** instances,
- then runs `docker compose up ... --scale ...` without recreating existing containers.

Important to understand:

- Compose itself cannot “assign unique host ports per replica” automatically.
- This script makes scaling repeatable and keeps the “port math” in one place.

> **Methods involved**
> - `compose` (wrapper for `docker compose` vs `docker-compose`)

```bash
#!/bin/bash
# Helper script to scale miners and webservers
# Works with running containers - existing containers are kept running
# Automatically generates port mappings so ALL instances have ports accessible externally
# Usage: ./docker-compose.scale.sh [num_miners] [num_webservers]
# Example: ./docker-compose.scale.sh 3 2  (3 miners, 2 webservers)

set -e

NUM_MINERS=${1:-1}
NUM_WEBSERVERS=${2:-1}

# Compose wrapper: prefer `docker compose`, fall back to `docker-compose`.
compose() {
    if docker compose version >/dev/null 2>&1; then
        docker compose "$@"
    elif command -v docker-compose >/dev/null 2>&1; then
        docker-compose "$@"
    else
        echo "Error: neither 'docker compose' nor 'docker-compose' is available" >&2
        exit 1
    fi
}

# Get current counts
CURRENT_MINERS=$(compose ps -q miner 2>/dev/null | wc -l | tr -d ' ')
CURRENT_WEBSERVERS=$(compose ps -q webserver 2>/dev/null | wc -l | tr -d ' ')

echo "Current blockchain network:"
echo "  Miners: ${CURRENT_MINERS}"
echo "  Webservers: ${CURRENT_WEBSERVERS}"
echo ""
echo "Scaling to:"
echo "  Miners: ${NUM_MINERS}"
echo "  Webservers: ${NUM_WEBSERVERS}"
echo ""

# Determine if scaling up or down
if [ "${NUM_MINERS}" -gt "${CURRENT_MINERS}" ]; then
    echo "  → Scaling UP miners: ${CURRENT_MINERS} → ${NUM_MINERS}"
elif [ "${NUM_MINERS}" -lt "${CURRENT_MINERS}" ]; then
    echo "  → Scaling DOWN miners: ${CURRENT_MINERS} → ${NUM_MINERS}"
fi

if [ "${NUM_WEBSERVERS}" -gt "${CURRENT_WEBSERVERS}" ]; then
    echo "  → Scaling UP webservers: ${CURRENT_WEBSERVERS} → ${NUM_WEBSERVERS}"
elif [ "${NUM_WEBSERVERS}" -lt "${CURRENT_WEBSERVERS}" ]; then
    echo "  → Scaling DOWN webservers: ${CURRENT_WEBSERVERS} → ${NUM_WEBSERVERS}"
fi

echo ""
echo "Generating port mappings for all instances..."

# Generate port override file to ensure ALL instances have ports mapped externally
OUTPUT_FILE="docker-compose.override.yml"

cat > "${OUTPUT_FILE}" <<EOF
version: '3.8'

# Auto-generated port mappings for multiple instances
# Generated by: docker-compose.scale.sh ${NUM_MINERS} ${NUM_WEBSERVERS}
# This ensures ALL instances have ports accessible externally

services:
  miner:
    ports:
EOF

# Generate miner port mappings
for i in $(seq 1 ${NUM_MINERS}); do
    P2P_PORT=$((2001 + i - 1))
    echo "      - \"${P2P_PORT}:${P2P_PORT}\"  # Miner instance ${i}" >> "${OUTPUT_FILE}"
done

cat >> "${OUTPUT_FILE}" <<EOF

  webserver:
    ports:
EOF

# Generate webserver port mappings
for i in $(seq 1 ${NUM_WEBSERVERS}); do
    WEB_PORT=$((8080 + i - 1))
    P2P_PORT=$((2101 + i - 1))
    INTERNAL_P2P=$((2001 + i - 1))
    echo "      - \"${WEB_PORT}:${WEB_PORT}\"  # Webserver instance ${i} - Web" >> "${OUTPUT_FILE}"
    echo "      - \"${P2P_PORT}:${INTERNAL_P2P}\"  # Webserver instance ${i} - P2P" >> "${OUTPUT_FILE}"
done

echo "  ✓ Generated ${OUTPUT_FILE} with port mappings for all instances"
echo ""

# Scale services (--no-recreate prevents recreating existing containers)
# This keeps existing containers running and only adds/removes as needed
compose up -d --no-recreate --scale miner=${NUM_MINERS} --scale webserver=${NUM_WEBSERVERS}

echo ""
echo "Scaled services:"
compose ps

echo ""
echo "Port mappings (ALL instances accessible externally):"
echo "  Miners:"
for i in $(seq 1 ${NUM_MINERS}); do
    P2P_PORT=$((2001 + i - 1))
    echo "    Instance ${i}: P2P port ${P2P_PORT} → localhost:${P2P_PORT}"
done
echo "  Webservers:"
for i in $(seq 1 ${NUM_WEBSERVERS}); do
    WEB_PORT=$((8080 + i - 1))
    P2P_PORT=$((2101 + i - 1))
    echo "    Instance ${i}: Web port ${WEB_PORT} → localhost:${WEB_PORT}, P2P port ${P2P_PORT} → localhost:${P2P_PORT}"
done
echo ""
echo "✓ All instances have ports mapped and accessible externally"
```

---

## Listing 8.5: `ci/docker-compose/configs/generate-compose-ports.sh`

This script generates `docker-compose.override.yml` without actually scaling containers. It is used when you prefer a two-step process:

1. generate the port mappings,
2. run `docker compose up --scale ...`.

> **Methods involved**
> - Artifact: `generate-compose-ports.sh` (override generation)

```bash
#!/bin/bash
# Generate docker-compose.override.yml with port mappings for multiple instances
# Usage: ./generate-compose-ports.sh [num_miners] [num_webservers]

set -e

NUM_MINERS=${1:-1}
NUM_WEBSERVERS=${2:-1}

OUTPUT_FILE="docker-compose.override.yml"

echo "Generating ${OUTPUT_FILE} for ${NUM_MINERS} miners and ${NUM_WEBSERVERS} webservers..."

cat > "${OUTPUT_FILE}" <<EOF
version: '3.8'

# Auto-generated port mappings for multiple instances
# Generated by: ./generate-compose-ports.sh ${NUM_MINERS} ${NUM_WEBSERVERS}

services:
  miner:
    ports:
EOF

# Generate miner port mappings
for i in $(seq 1 ${NUM_MINERS}); do
    P2P_PORT=$((2001 + i - 1))
    echo "      - \"${P2P_PORT}:${P2P_PORT}\"  # Miner instance ${i}" >> "${OUTPUT_FILE}"
done

cat >> "${OUTPUT_FILE}" <<EOF

  webserver:
    ports:
EOF

# Generate webserver port mappings
for i in $(seq 1 ${NUM_WEBSERVERS}); do
    WEB_PORT=$((8080 + i - 1))
    P2P_PORT=$((2101 + i - 1))
    INTERNAL_P2P=$((2001 + i - 1))
    echo "      - \"${WEB_PORT}:${WEB_PORT}\"  # Webserver instance ${i} - Web" >> "${OUTPUT_FILE}"
    echo "      - \"${P2P_PORT}:${INTERNAL_P2P}\"  # Webserver instance ${i} - P2P" >> "${OUTPUT_FILE}"
done

echo ""
echo "Generated ${OUTPUT_FILE}"
echo ""
echo "Port mappings:"
echo "  Miners:"
for i in $(seq 1 ${NUM_MINERS}); do
    P2P_PORT=$((2001 + i - 1))
    echo "    Instance ${i}: P2P port ${P2P_PORT}"
done
echo "  Webservers:"
for i in $(seq 1 ${NUM_WEBSERVERS}); do
    WEB_PORT=$((8080 + i - 1))
    P2P_PORT=$((2101 + i - 1))
    echo "    Instance ${i}: Web port ${WEB_PORT}, P2P port ${P2P_PORT}"
done
echo ""
echo "Now run:"
echo "  docker compose up -d --scale miner=${NUM_MINERS} --scale webserver=${NUM_WEBSERVERS}"
echo "  # (or, legacy): docker-compose up -d --scale miner=${NUM_MINERS} --scale webserver=${NUM_WEBSERVERS}"
```

---

## Listing 8.6: `ci/docker-compose/configs/scale-up.sh`

Incremental scaling helper: adds one instance at a time and regenerates the override file to keep host ports consistent.

> **Methods involved**
> - `compose` (wrapper)

```bash
#!/bin/bash
# Incremental scaling script - adds one instance at a time
# Automatically updates port mappings so ALL instances have ports accessible externally
# Usage: ./scale-up.sh [service] [current_count]
# Example: ./scale-up.sh miner 2  (scales miners from 2 to 3)

set -e

SERVICE=${1:-webserver}
CURRENT_COUNT=${2:-1}
NEW_COUNT=$((CURRENT_COUNT + 1))

echo "Scaling ${SERVICE} up: ${CURRENT_COUNT} → ${NEW_COUNT}"
echo ""

# Compose wrapper: prefer `docker compose`, fall back to `docker-compose`.
compose() {
    if docker compose version >/dev/null 2>&1; then
        docker compose "$@"
    elif command -v docker-compose >/dev/null 2>&1; then
        docker-compose "$@"
    else
        echo "Error: neither 'docker compose' nor 'docker-compose' is available" >&2
        exit 1
    fi
}

# Get current counts for both services
CURRENT_MINERS=$(compose ps -q miner 2>/dev/null | wc -l | tr -d ' ')
CURRENT_WEBSERVERS=$(compose ps -q webserver 2>/dev/null | wc -l | tr -d ' ')

# Determine new counts after scaling
if [ "${SERVICE}" = "miner" ]; then
    NEW_MINERS=${NEW_COUNT}
    NEW_WEBSERVERS=${CURRENT_WEBSERVERS}
else
    NEW_MINERS=${CURRENT_MINERS}
    NEW_WEBSERVERS=${NEW_COUNT}
fi

# Generate port override file for updated counts
echo "Updating port mappings..."
OUTPUT_FILE="docker-compose.override.yml"

cat > "${OUTPUT_FILE}" <<EOF
version: '3.8'

# Auto-generated port mappings for multiple instances
# Generated by: scale-up.sh
# This ensures ALL instances have ports accessible externally

services:
  miner:
    ports:
EOF

# Generate miner port mappings
for i in $(seq 1 ${NEW_MINERS}); do
    P2P_PORT=$((2001 + i - 1))
    echo "      - \"${P2P_PORT}:${P2P_PORT}\"  # Miner instance ${i}" >> "${OUTPUT_FILE}"
done

cat >> "${OUTPUT_FILE}" <<EOF

  webserver:
    ports:
EOF

# Generate webserver port mappings
for i in $(seq 1 ${NEW_WEBSERVERS}); do
    WEB_PORT=$((8080 + i - 1))
    P2P_PORT=$((2101 + i - 1))
    INTERNAL_P2P=$((2001 + i - 1))
    echo "      - \"${WEB_PORT}:${WEB_PORT}\"  # Webserver instance ${i} - Web" >> "${OUTPUT_FILE}"
    echo "      - \"${P2P_PORT}:${INTERNAL_P2P}\"  # Webserver instance ${i} - P2P" >> "${OUTPUT_FILE}"
done

echo "  ✓ Updated ${OUTPUT_FILE}"
echo ""

# Scale up by one
compose up -d --no-recreate --scale ${SERVICE}=${NEW_COUNT}

echo ""
echo "Waiting for new container to start..."
sleep 5

echo ""
echo "Current ${SERVICE} containers:"
compose ps ${SERVICE}

echo ""
echo "Port mappings (ALL instances accessible externally):"
if [ "${SERVICE}" = "miner" ]; then
    P2P_PORT=$((2001 + NEW_COUNT - 1))
    echo "  New miner instance ${NEW_COUNT}: P2P port ${P2P_PORT} → localhost:${P2P_PORT}"
else
    WEB_PORT=$((8080 + NEW_COUNT - 1))
    P2P_PORT=$((2101 + NEW_COUNT - 1))
    echo "  New webserver instance ${NEW_COUNT}: Web port ${WEB_PORT} → localhost:${WEB_PORT}, P2P port ${P2P_PORT} → localhost:${P2P_PORT}"
fi

echo ""
echo "View logs of new container:"
echo "  docker compose logs -f ${SERVICE}_${NEW_COUNT}"
```

---

## Listing 8.7: `ci/docker-compose/configs/scale-down.sh`

Incremental scale-down helper: removes one instance and regenerates the override file to keep port mappings coherent for remaining instances.

> **Methods involved**
> - `compose` (wrapper)

```bash
#!/bin/bash
# Incremental scaling script - removes one instance at a time
# Automatically updates port mappings so remaining instances have ports accessible externally
# Usage: ./scale-down.sh [service] [current_count]
# Example: ./scale-down.sh webserver 3  (scales webservers from 3 to 2)

set -e

SERVICE=${1:-webserver}
CURRENT_COUNT=${2:-2}
NEW_COUNT=$((CURRENT_COUNT - 1))

if [ "${NEW_COUNT}" -lt 1 ]; then
    echo "Error: Cannot scale below 1 instance"
    exit 1
fi

echo "Scaling ${SERVICE} down: ${CURRENT_COUNT} → ${NEW_COUNT}"
echo ""

# Compose wrapper: prefer `docker compose`, fall back to `docker-compose`.
compose() {
    if docker compose version >/dev/null 2>&1; then
        docker compose "$@"
    elif command -v docker-compose >/dev/null 2>&1; then
        docker-compose "$@"
    else
        echo "Error: neither 'docker compose' nor 'docker-compose' is available" >&2
        exit 1
    fi
}

# Get current counts for both services
CURRENT_MINERS=$(compose ps -q miner 2>/dev/null | wc -l | tr -d ' ')
CURRENT_WEBSERVERS=$(compose ps -q webserver 2>/dev/null | wc -l | tr -d ' ')

# Determine new counts after scaling
if [ "${SERVICE}" = "miner" ]; then
    NEW_MINERS=${NEW_COUNT}
    NEW_WEBSERVERS=${CURRENT_WEBSERVERS}
else
    NEW_MINERS=${CURRENT_MINERS}
    NEW_WEBSERVERS=${NEW_COUNT}
fi

# Generate port override file for updated counts
echo "Updating port mappings..."
OUTPUT_FILE="docker-compose.override.yml"

cat > "${OUTPUT_FILE}" <<EOF
version: '3.8'

# Auto-generated port mappings for multiple instances
# Generated by: scale-down.sh
# This ensures ALL instances have ports accessible externally

services:
  miner:
    ports:
EOF

# Generate miner port mappings
for i in $(seq 1 ${NEW_MINERS}); do
    P2P_PORT=$((2001 + i - 1))
    echo "      - \"${P2P_PORT}:${P2P_PORT}\"  # Miner instance ${i}" >> "${OUTPUT_FILE}"
done

cat >> "${OUTPUT_FILE}" <<EOF

  webserver:
    ports:
EOF

# Generate webserver port mappings
for i in $(seq 1 ${NEW_WEBSERVERS}); do
    WEB_PORT=$((8080 + i - 1))
    P2P_PORT=$((2101 + i - 1))
    INTERNAL_P2P=$((2001 + i - 1))
    echo "      - \"${WEB_PORT}:${WEB_PORT}\"  # Webserver instance ${i} - Web" >> "${OUTPUT_FILE}"
    echo "      - \"${P2P_PORT}:${INTERNAL_P2P}\"  # Webserver instance ${i} - P2P" >> "${OUTPUT_FILE}"
done

echo "  ✓ Updated ${OUTPUT_FILE}"
echo ""

# Scale down by one
compose up -d --no-recreate --scale ${SERVICE}=${NEW_COUNT}

echo ""
echo "Waiting for container removal..."
sleep 5

echo ""
echo "Current ${SERVICE} containers:"
compose ps ${SERVICE}

echo ""
echo "✓ Port mappings updated - remaining instances have ports accessible externally"
```

---

## Listing 8.8: `ci/docker-compose/configs/docker-compose.miner.yml`

Standalone miner-only Compose file (no webserver, no Redis). Useful for experiments and for building networks where webservers run elsewhere.

> **Methods involved**
> - Artifact: `docker-compose.miner.yml`

```yaml
# Alternative compose file: Run blockchain node as a miner (no web server)
# Usage: docker compose -f docker-compose.miner.yml up -d
# To scale: docker compose -f docker-compose.miner.yml up -d --scale miner=3
#
# Note: Instance numbers are auto-detected by the entrypoint script from container names
# Instance 1: P2P port 2001, Instance 2: P2P port 2002, etc.

services:
  miner:
    build:
      context: /Users/bkunyiha/Rust/blockchain
      dockerfile: Dockerfile
    # Remove container_name to allow scaling
    ports:
      - "2001:2001"  # First miner instance P2P port
    volumes:
      # Data directories are auto-configured by entrypoint (data1, data2, etc.)
      - miner-data:/app/data
      - miner-wallets:/app/wallets
    environment:
      # Node mode: miner (no web server)
      # Format matches: cargo run startnode yes no local -- <address>
      - NODE_IS_MINER=yes
      - NODE_IS_WEB_SERVER=no
      # Connect nodes: "local" for seed node, or remote addresses
      - NODE_CONNECT_NODES=${NODE_CONNECT_NODES:-local}
      # Mining address configuration (choose one):
      # Option 1: Use WALLET_ADDRESS_POOL (comma-separated) - each instance auto-selects by index
      # Option 2: Use NODE_MINING_ADDRESS directly for all instances
      # At least one must be set
      - WALLET_ADDRESS_POOL=${WALLET_ADDRESS_POOL:-}
      - NODE_MINING_ADDRESS=${NODE_MINING_ADDRESS:-}
      # Optional: Central node configuration
      - CENTERAL_NODE=${CENTERAL_NODE:-}
      # Wallet file path
      - WALLET_FILE=wallets/wallets.dat
    restart: unless-stopped

volumes:
  miner-data:
    driver: local
  miner-wallets:
    driver: local
```

---

## Listing 8.9: `ci/docker-compose/configs/docker-compose.webserver.yml`

Standalone webserver-only Compose file. It includes Redis (rate limiting backend) but omits miners.

> **Methods involved**
> - Artifact: `docker-compose.webserver.yml`

```yaml
# Alternative compose file: Run blockchain node as a web server (no miner)
# Usage: docker compose -f docker-compose.webserver.yml up -d
# To scale: docker compose -f docker-compose.webserver.yml up -d --scale webserver=3
#
# Note: Instance numbers are auto-detected by the entrypoint script from container names
# Instance 1: Web port 8080, P2P port 2101
# Instance 2: Web port 8081, P2P port 2102, etc.

services:
  # Redis for axum_rate_limiter (rate limiting state)
  redis:
    image: redis:7-alpine
    restart: unless-stopped
    volumes:
      - redis-data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 5s
      timeout: 3s
      retries: 20

  webserver:
    build:
      context: /Users/bkunyiha/Rust/blockchain
      dockerfile: Dockerfile
    depends_on:
      redis:
        condition: service_healthy
    # Remove container_name to allow scaling
    ports:
      - "8080:8080"  # First webserver instance - Web port
      - "2101:2001"  # First webserver instance - P2P port (mapped to 2101)
    volumes:
      # Data directories are auto-configured by entrypoint (data1, data2, etc.)
      - webserver-data:/app/data
      - webserver-wallets:/app/wallets
      # Rate limiter settings (used by axum_rate_limiter via RL_SETTINGS_PATH)
      - ./Settings.toml:/app/Settings.toml:ro
    environment:
      # Node mode: web server (no miner)
      # Format matches: cargo run startnode no yes <connect_nodes> [-- <address>]
      - NODE_IS_MINER=no
      - NODE_IS_WEB_SERVER=yes
      # Connect nodes: "local" for seed node, or remote addresses
      - NODE_CONNECT_NODES=${NODE_CONNECT_NODES:-local}
      # Mining address configuration (choose one):
      # Option 1: Use WALLET_ADDRESS_POOL (comma-separated) - each instance auto-selects by index
      # Option 2: Use NODE_MINING_ADDRESS directly for all instances
      # At least one must be set
      - WALLET_ADDRESS_POOL=${WALLET_ADDRESS_POOL:-}
      - NODE_MINING_ADDRESS=${NODE_MINING_ADDRESS:-}
      # API authentication keys
      - BITCOIN_API_ADMIN_KEY=${BITCOIN_API_ADMIN_KEY:-admin-secret}
      - BITCOIN_API_WALLET_KEY=${BITCOIN_API_WALLET_KEY:-wallet-secret}
      # Rate limiting configuration (axum_rate_limiter)
      - RL_SETTINGS_PATH=/app/Settings.toml
      # Optional: Central node configuration
      - CENTERAL_NODE=${CENTERAL_NODE:-}
      # Wallet file path
      - WALLET_FILE=wallets/wallets.dat
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/api/health/liveness"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s

volumes:
  redis-data:
    driver: local
  webserver-data:
    driver: local
  webserver-wallets:
    driver: local
```

---

## Listing 8.10: `ci/docker-compose/configs/Settings.toml`

Rate limiter configuration consumed by the Rust webserver when `RL_SETTINGS_PATH=/app/Settings.toml` is set.

> **Methods involved**
> - Artifact: `Settings.toml` (rate limiter configuration)

```toml
[rate_limiter]
# Docker Compose service DNS name + default Redis port.
# NOTE: axum_rate_limiter expects host:port (no redis:// scheme).
redis_addr = "redis:6379"

# Bypass rate limiting for trusted IPs.
# In containers, requests from outside Docker often appear as a bridge/gateway IP,
# so keep this list small unless you know your network topology.
ip_whitelist = ["127.0.0.1"]

# Default per-IP limit (token bucket)
[[rate_limiter.limiter]]
strategy = "ip"
global_bucket = { tokens_count = 20, add_tokens_every = 6 }
```

---

## Listing 8.11: `ci/docker-compose/configs/Dockerfile`

This Dockerfile builds and packages the Rust node binary, React web UI, and runtime scripts used by Compose (entrypoint + wait script).

> **Methods involved**
> - Artifact: Multi-stage Dockerfile (Rust + Node.js builds)

```dockerfile
# Build stage for Rust binary
FROM rust:1.91.1-slim AS rust-builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY bitcoin/Cargo.toml ./bitcoin/
# Copy other workspace members' Cargo.toml files (needed for workspace resolution)
COPY bitcoin-desktop-ui-iced/Cargo.toml ./bitcoin-desktop-ui-iced/
COPY bitcoin-wallet-ui-iced/Cargo.toml ./bitcoin-wallet-ui-iced/
COPY bitcoin-api/Cargo.toml ./bitcoin-api/

# Copy source code for all workspace members (Cargo needs them to resolve workspace)
COPY bitcoin/src ./bitcoin/src
COPY bitcoin-desktop-ui-iced/src ./bitcoin-desktop-ui-iced/src
COPY bitcoin-wallet-ui-iced/src ./bitcoin-wallet-ui-iced/src
COPY bitcoin-api/src ./bitcoin-api/src

# Build the blockchain binary
RUN cargo build --release -p blockchain

# Build stage for React web UI
FROM node:20-slim AS web-ui-builder

# Set working directory
WORKDIR /app

# Copy package files
COPY bitcoin-web-ui/package.json bitcoin-web-ui/package-lock.json ./bitcoin-web-ui/

# Install dependencies
WORKDIR /app/bitcoin-web-ui
RUN npm ci

# Copy source files
COPY bitcoin-web-ui/ ./

# Build React app
RUN npm run build

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    netcat-openbsd \
    libc-bin \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy binary from Rust builder
COPY --from=rust-builder /app/target/release/blockchain /app/blockchain

# Copy built React web UI from web-ui-builder
# The Rust server looks for bitcoin-web-ui/dist relative to the binary location
COPY --from=web-ui-builder /app/bitcoin-web-ui/dist /app/bitcoin-web-ui/dist

# Copy entrypoint script
COPY ci/docker-compose/configs/docker-entrypoint.sh /app/docker-entrypoint.sh
RUN chmod +x /app/docker-entrypoint.sh

# Copy wait script for sequential startup
COPY ci/docker-compose/configs/wait-for-node.sh /app/wait-for-node.sh
RUN chmod +x /app/wait-for-node.sh

# Create data directory
RUN mkdir -p /app/data

# Expose ports
# 8080: Web server
# 2001: P2P network
EXPOSE 8080 2001

# Set default environment variables
ENV TREE_DIR=data1
ENV BLOCKS_TREE=blocks1
ENV NODE_ADDR=0.0.0.0:2001

# Node configuration (can be overridden)
# NODE_IS_MINER defaults to "no" (webserver mode) as a safe default:
# - Prevents accidental mining if container is run directly without docker-compose
# - docker-compose.yml explicitly sets NODE_IS_MINER=yes for miner service (overrides this default)
# - This default is a fallback; docker-compose always sets the correct value for each service
ENV NODE_IS_MINER=no
ENV NODE_IS_WEB_SERVER=yes
ENV NODE_CONNECT_NODES=local
# NODE_MINING_ADDRESS is required and must be set at runtime

# Use entrypoint script for flexible node configuration
ENTRYPOINT ["/app/docker-entrypoint.sh"]
```

---

<div align="center">

**Reading order**

**[← Previous: Docker Compose Deployment](01-Introduction.md)** | **[Next: Kubernetes Deployment →](../kubernetes/README.md)**

</div>

---

