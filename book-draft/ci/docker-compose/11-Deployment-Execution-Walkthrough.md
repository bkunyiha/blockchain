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
   - [Section 8: Deployment Scenarios](08-Deployment-Scenarios.md)
   - [Section 9: Accessing Webserver](09-Accessing-Webserver.md)
   - [Section 10: Deployment Guide](10-Deployment-Guide.md)
   - **Section 11: Deployment Execution Walkthrough** ← *You are here*
   - [Section 12: DNS Resolution Mechanism](12-DNS-Resolution-Mechanism.md)
9. [Chapter 9: Kubernetes Deployment](../../kubernetes/README.md)

</details>

</div>

<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

# Chapter 7, Section 11: Deployment Execution Walkthrough

**Part II: Deployment & Operations** | **Chapter 8: Docker Compose Deployment**

<div align="center">

**📚 [← Section 10: Deployment Guide](10-Deployment-Guide.md)** | **Section 11: Deployment Execution Walkthrough** | **[Section 12: DNS Resolution Mechanism →](12-DNS-Resolution-Mechanism.md)** 📚

</div>

---

## Overview

This section provides a detailed, step-by-step walkthrough of how the Docker Compose deployment system operates. Through practical examples, you will learn how containers initialize, how nodes discover and connect to each other, and how the system ensures reliable startup sequencing. This walkthrough traces the complete execution flow from container startup through node connection establishment.

**Prerequisites:**
- Understanding of [Section 3: Execution Flow](03-Execution-Flow.md)
- Familiarity with [Section 7: Sequential Startup](07-Sequential-Startup.md)
- Basic knowledge of Bash scripting and Docker networking

**Related Sections:**
   - [Section 12: DNS Resolution Mechanism](12-DNS-Resolution-Mechanism.md) - Deep dive into DNS resolution mechanisms
- [Section 4: Network Configuration](04-Network-Configuration.md) - Network setup context
- [Section 10: Deployment Guide](10-Deployment-Guide.md) - Deployment procedures

---

## Table of Contents

1. [Scenario 1: Webserver Container Startup](#scenario-1-webserver-container-startup)
2. [Scenario 2: Additional Miner Instance Startup](#scenario-2-additional-miner-instance-startup)
3. [Scenario 3: Instance Number Validation](#scenario-3-instance-number-validation)
4. [Scenario 4: DNS Resolution Process](#scenario-4-dns-resolution-process)
5. [Scenario 5: Web UI Build and Deployment](#scenario-5-web-ui-build-and-deployment)
6. [Summary: System Design Patterns](#summary-system-design-patterns)

---

## Scenario 1: Webserver Container Startup

This scenario demonstrates the most common deployment case: a webserver container starting and establishing a connection to the first miner node. This walkthrough shows how the system automatically configures the webserver to connect to the appropriate miner.

### Initial Container State

When Docker Compose starts a webserver container, it begins with the following configuration:

**Container:** `configs-webserver-1`  
**Environment Variables:**
```bash
NODE_IS_MINER=no
NODE_IS_WEB_SERVER=yes
NODE_CONNECT_NODES=miner_1:2001  # From docker-compose.yml default
INSTANCE_NUMBER=1
HOSTNAME=configs-webserver-1
```

### Step 1: Container Identity Extraction

The entrypoint script first extracts identifying information from the container's hostname.

**File:** `ci/docker-compose/configs/docker-entrypoint.sh` (Lines 243-260)

```bash
CONTAINER_NAME="${HOSTNAME:-}"  # CONTAINER_NAME="configs-webserver-1"

# Extract instance number
if [[ "${CONTAINER_NAME}" =~ _([0-9]+)$ ]]; then
    INSTANCE_NUMBER="${BASH_REMATCH[1]}"  # INSTANCE_NUMBER=1
fi
```

**What happens:**
- The script uses the container's hostname to determine its identity
- A regular expression extracts the instance number from the hostname pattern
- This instance number is used throughout the startup process for port calculation and node identification

**Result:**
- `CONTAINER_NAME="configs-webserver-1"`
- `INSTANCE_NUMBER=1`

### Step 2: Service Type Detection

Based on the container name, the system determines the service type and calculates appropriate port numbers.

**File:** `docker-entrypoint.sh` (Lines 262-292)

```bash
# Determine service type from container name
if [[ "${CONTAINER_NAME}" =~ webserver ]]; then
    SERVICE_TYPE="webserver"
    WEB_PORT=$((8080 + INSTANCE_NUMBER - 1))  # WEB_PORT=8080
    P2P_PORT=$((2101 + INSTANCE_NUMBER - 1))  # P2P_PORT=2101
fi
```

**What happens:**
- The script identifies the service type by pattern matching the container name
- Port numbers are calculated dynamically based on the instance number
- This allows multiple instances of the same service type to run without port conflicts

**Result:**
- `SERVICE_TYPE="webserver"`
- `WEB_PORT=8080`
- `P2P_PORT=2101`

### Step 3: Connection Configuration Normalization

The system normalizes the connection configuration, handling empty or whitespace-only values.

**File:** `docker-entrypoint.sh` (Lines 22-33)

```bash
# Normalize NODE_CONNECT_NODES
if [ -z "${NODE_CONNECT_NODES}" ] || [ -z "$(echo "${NODE_CONNECT_NODES}" | xargs)" ]; then
    NODE_CONNECT_NODES="local"
else
    NODE_CONNECT_NODES=$(echo "${NODE_CONNECT_NODES}" | xargs)  # Trim whitespace
fi
```

**What happens:**
- Empty or whitespace-only connection node specifications are normalized to `"local"`
- Valid specifications are trimmed of leading/trailing whitespace
- This ensures consistent handling of configuration values regardless of how they're specified

**Result:**
- `NODE_CONNECT_NODES="miner_1:2001"` (from docker-compose.yml, trimmed)

### Step 4: Sequential Startup Coordination

When sequential startup is enabled, the webserver waits for the miner to be ready before proceeding.

**File:** `docker-entrypoint.sh` (Lines 400-430)

```bash
if [ "${SEQUENTIAL_STARTUP}" = "yes" ]; then
    echo "Sequential startup enabled: Webserver waiting for miner to be ready..."
    
    # Call wait-for-node.sh
    WAIT_OUTPUT=$(
        /app/wait-for-node.sh "miner" "${WAIT_INSTANCE_NUMBER}" "2001" "yes"
    )
```

**What happens:**
- For webservers, `WAIT_INSTANCE_NUMBER` is calculated as `INSTANCE_NUMBER + 1`
- For `webserver-1`, this results in `WAIT_INSTANCE_NUMBER=2`
- This ensures webservers wait for `miner_1` (instance 1) to be ready
- The wait script verifies the miner is listening on its P2P port before allowing the webserver to continue

**Why this matters:**
- Prevents webservers from attempting to connect to miners that aren't ready
- Ensures reliable startup ordering in distributed deployments
- Reduces connection errors and retry attempts

### Step 5: Wait Script Execution

The wait script coordinates the startup sequence by checking for the availability of the previous node.

**File:** `wait-for-node.sh` (Lines 7-47)

```bash
WAIT_SERVICE_NAME="miner"
INSTANCE_NUMBER=2  # Passed as WAIT_INSTANCE_NUMBER
PORT=2001
IS_WEBSERVER="yes"

# Calculate previous instance
PREV_INSTANCE=$((INSTANCE_NUMBER - 1))  # PREV_INSTANCE=1

# CRITICAL: Ensure PREV_INSTANCE is never less than 1
if [ ${PREV_INSTANCE} -lt 1 ]; then
    # Validation prevents invalid instance numbers
fi

# If this is instance 1 and NOT a webserver, exit early
# For webservers, we always wait (even instance 1 waits for miner_1)
if [ "${INSTANCE_NUMBER}" -eq 1 ] && [ "${IS_WEBSERVER}" != "yes" ]; then
    # Early exit for first miner instance (seed node)
fi
```

**What happens:**
- The script calculates which previous instance it should wait for
- Validation ensures instance numbers are always valid (>= 1)
- Webservers always wait for a miner, even if they're the first webserver instance
- This ensures webservers never start before miners are available

**Result:**
- `PREV_INSTANCE=1` (valid, >= 1)
- Script continues to wait for `miner_1`

### Step 6: Node Discovery Process

The wait script discovers available miner nodes by checking Docker's internal DNS and port availability.

**File:** `wait-for-node.sh` (Lines 68-146)

```bash
CHECK_INSTANCE=${PREV_INSTANCE}  # CHECK_INSTANCE=1

# Ensure we never check invalid instance numbers
if [ ${CHECK_INSTANCE} -lt 1 ]; then
    CHECK_INSTANCE=1  # Safety validation
fi

while [ ${CHECK_INSTANCE} -ge 1 ]; do
    # Use service name "miner" for Docker Compose DNS resolution
    CHECK_HOSTNAME="miner"  # Service name, not instance name
    CHECK_PORT=$((2001 + CHECK_INSTANCE - 1))  # CHECK_PORT=2001
    
    echo "  Checking miner instance 1: miner:2001..."
    
    # Try to connect using netcat
    if nc -z -w 1 "miner" "2001" 2>/dev/null; then
        READY=true
        break
    fi
    
    # ... retry logic with exponential backoff ...
done
```

**What happens:**
- The script uses Docker Compose's service name (`miner`) for DNS resolution
- Docker's internal DNS resolves the service name to the appropriate container IP
- A port check verifies the miner is actually listening and ready to accept connections
- Retry logic with exponential backoff handles transient network issues

**Why use service name instead of instance name:**
- Docker Compose DNS resolves service names (e.g., `miner`) but not instance names (e.g., `miner_1`)
- Using the service name leverages Docker's built-in service discovery
- This approach works reliably across different Docker Compose network configurations

**Result:**
- `READY=true`
- Miner is discovered and verified ready

### Step 7: Previous Node Address Output

Once a miner is found, the wait script outputs the address information for the entrypoint script to use.

**File:** `wait-for-node.sh` (Lines 125-135)

```bash
if [ "${READY}" = "true" ]; then
    echo "  Miner miner:2001 is ready!"
    echo "Instance 2: Previous node is ready!"
    
    # Output the connect nodes address
    # Use miner_${CHECK_INSTANCE} format for clarity
    OUTPUT_HOSTNAME="miner_${CHECK_INSTANCE}"  # OUTPUT_HOSTNAME="miner_1"
    echo "PREV_NODE_ADDRESS=${OUTPUT_HOSTNAME}:${PREV_PORT}"  # PREV_NODE_ADDRESS=miner_1:2001
    exit 0
fi
```

**What happens:**
- The script outputs the discovered node address in a parseable format
- The instance name format (`miner_1`) is used for clarity in logs and configuration
- This address will be converted to an IP address in the next step

**Result:**
- Wait script outputs: `PREV_NODE_ADDRESS=miner_1:2001`
- Exit code: 0 (success)

### Step 8: Address Extraction and Validation

The entrypoint script extracts the address from the wait script output and validates it.

**File:** `docker-entrypoint.sh` (Lines 436-455)

```bash
if [ ${WAIT_EXIT_CODE} -eq 0 ]; then
    # Extract PREV_NODE_ADDRESS from output
    PREV_ADDR=$(echo "${WAIT_OUTPUT}" | grep "PREV_NODE_ADDRESS=" | cut -d'=' -f2)
    # PREV_ADDR="miner_1:2001"
    
    debug_log "Extracted PREV_ADDR from wait script: '${PREV_ADDR}'"
    
    # Validate extracted address - webservers should never get invalid addresses
    PREV_ADDR=$(validate_and_fix_miner_address "${PREV_ADDR}" "${NODE_IS_WEB_SERVER}")
    # validate_and_fix_miner_address ensures address validity
```

**What happens:**
- The script parses the wait script output to extract the node address
- Validation functions ensure the address is valid and appropriate for the service type
- Invalid addresses (such as `miner_0`) are corrected or rejected

**Result:**
- `PREV_ADDR="miner_1:2001"` (validated and confirmed valid)

### Step 9: DNS Resolution Preparation

The system converts instance names to service names for Docker Compose DNS resolution.

**File:** `docker-entrypoint.sh` (Lines 463-470)

```bash
# Resolve Docker service name to IP address
# For Docker Compose, "miner_1" doesn't resolve, but "miner" does
RESOLVE_ADDR="${PREV_ADDR}"  # RESOLVE_ADDR="miner_1:2001"

if [[ "${PREV_ADDR}" =~ ^miner_([0-9]+): ]]; then
    # Extract instance number and use "miner" service name for resolution
    INSTANCE_NUM="${BASH_REMATCH[1]}"  # INSTANCE_NUM=1
    PORT_PART="${PREV_ADDR##*:}"  # PORT_PART=2001
    RESOLVE_ADDR="miner:${PORT_PART}"  # RESOLVE_ADDR="miner:2001"
    debug_log "Converting miner_1:2001 to miner:2001 for Docker Compose DNS resolution"
fi
```

**What happens:**
- The system recognizes that Docker Compose DNS resolves service names, not instance names
- Instance names like `miner_1` are converted to service names like `miner` for DNS resolution
- The port number is preserved during this conversion
- This conversion enables reliable DNS resolution within Docker Compose networks

> **📖 Deep Dive:** For a detailed explanation of this DNS resolution mechanism, see [Section 12: DNS Resolution Mechanism](12-DNS-Resolution-Mechanism.md).

**Result:**
- `RESOLVE_ADDR="miner:2001"` (ready for DNS resolution)

### Step 10: Hostname to IP Resolution

The system resolves the service name to an IP address using Docker's internal DNS.

**File:** `docker-entrypoint.sh` (Lines 472-475)

```bash
if ! PREV_ADDR_RESOLVED=$(resolve_hostname_to_ip "${RESOLVE_ADDR}"); then
    echo "ERROR: Failed to resolve previous node address '${RESOLVE_ADDR}'" >&2
    exit 1
fi
```

**Inside `resolve_hostname_to_ip()`:**

```bash
resolve_hostname_to_ip() {
    local addr="miner:2001"
    local hostname="miner"
    local port="2001"
    
    # Try getent hosts (works with Docker's internal DNS)
    getent_output=$(getent hosts "miner" 2>&1)
    # getent_output="172.19.0.2    miner"
    
    if [ ${getent_exit} -eq 0 ]; then
        ip=$(echo "${getent_output}" | awk '{print $1}' | head -n1)
        # ip="172.19.0.2"
        echo "${ip}:${port}"  # Returns "172.19.0.2:2001"
        return 0
    fi
}
```

**What happens:**
- The function uses `getent hosts` to query Docker's internal DNS
- Docker Compose's DNS service resolves the service name to the container's IP address
- The IP address and port are combined into a format the Rust binary can use
- Multiple resolution methods are attempted with retry logic for reliability

**Result:**
- `PREV_ADDR_RESOLVED="172.19.0.2:2001"` (IP address resolved)

### Step 11: Webserver Connection Configuration

The system configures the webserver to connect to the resolved miner address.

**File:** `docker-entrypoint.sh` (Lines 494-502)

```bash
else
    # Webservers always connect to first miner (miner_1:2001)
    # Sequential startup ensures miner_1 is ready before webservers start
    # Use "miner:2001" for Docker Compose DNS resolution
    if ! NODE_CONNECT_NODES=$(resolve_hostname_to_ip "miner:2001"); then
        echo "ERROR: Failed to resolve miner:2001 for webserver" >&2
        exit 1
    fi
    # NODE_CONNECT_NODES="172.19.0.2:2001"
    echo "  Auto-configured webserver to connect to first miner: miner:2001 -> ${NODE_CONNECT_NODES}"
fi
```

**What happens:**
- Webservers are automatically configured to connect to the first miner
- The connection address is resolved to an IP address before being passed to the Rust binary
- This auto-configuration eliminates the need for manual connection setup
- The resolved IP address ensures reliable connection establishment

**Result:**
- `NODE_CONNECT_NODES="172.19.0.2:2001"` (IP address, ready for Rust)

### Step 12: Final Configuration and Startup

The system displays the final configuration and starts the blockchain node.

**File:** `docker-entrypoint.sh` (Lines 600-620)

```bash
echo "=========================================="
echo "Starting blockchain node"
echo "  Service Type: webserver"
echo "  Instance Number: 1"
echo "  Container Name: configs-webserver-1"
echo "  Mode: miner=no, webserver=yes"
echo "  P2P Port: 2101"
echo "  Web Port: 8080"
echo "  Data Directory: /app/data/data1"
echo "  Connect Nodes: 172.19.0.2:2001"  # IP address, not hostname
echo "=========================================="
```

**What happens:**
- All configuration is complete and validated
- The connection address is in IP format, which the Rust binary can parse directly
- The node starts with the correct configuration for its role and instance number

**Result:**
- All configuration is complete
- `NODE_CONNECT_NODES` is an IP address (Rust can parse it)
- Webserver will connect to miner successfully

---

## Scenario 2: Additional Miner Instance Startup

This scenario demonstrates how additional miner instances connect to the existing network. When `miner_2` starts, it must discover and connect to `miner_1`.

### Initial Container State

**Container:** `configs-miner-2`  
**Environment Variables:**
```bash
NODE_IS_MINER=yes
NODE_IS_WEB_SERVER=no
NODE_CONNECT_NODES=local  # Will be auto-configured
INSTANCE_NUMBER=2
HOSTNAME=configs-miner-2
```

### Step 1: Instance Number Extraction

The system extracts the instance number from the container name.

```bash
CONTAINER_NAME="configs-miner-2"
if [[ "${CONTAINER_NAME}" =~ _([0-9]+)$ ]]; then
    INSTANCE_NUMBER="${BASH_REMATCH[1]}"  # INSTANCE_NUMBER=2
fi
```

### Step 2: Port Calculation

Ports are calculated based on the instance number to avoid conflicts.

```bash
if [[ "${CONTAINER_NAME}" =~ miner ]]; then
    SERVICE_TYPE="miner"
    P2P_PORT=$((2001 + INSTANCE_NUMBER - 1))  # P2P_PORT=2002
fi
```

### Step 3: Sequential Startup Coordination

The miner waits for the previous miner instance to be ready.

**File:** `docker-entrypoint.sh` (Lines 400-430)

```bash
if [ "${SEQUENTIAL_STARTUP}" = "yes" ]; then
    # For miners, WAIT_INSTANCE_NUMBER = INSTANCE_NUMBER
    WAIT_INSTANCE_NUMBER=2
    
    WAIT_OUTPUT=$(
        /app/wait-for-node.sh "miner" "2" "2001" "no"
    )
```

**What happens:**
- Miners wait for the previous miner instance in the sequence
- `miner_2` waits for `miner_1` to be ready
- This ensures a chain of connections: `miner_2` → `miner_1`

### Step 4: Previous Instance Calculation

The wait script calculates which instance to wait for.

**File:** `wait-for-node.sh` (Lines 25-40)

```bash
INSTANCE_NUMBER=2
PREV_INSTANCE=$((INSTANCE_NUMBER - 1))  # PREV_INSTANCE=1

# CRITICAL: Ensure PREV_INSTANCE is never less than 1
if [ ${PREV_INSTANCE} -lt 1 ]; then
    # Validation prevents invalid instance numbers
fi
```

**Result:**
- `PREV_INSTANCE=1` (valid, >= 1)
- Will wait for `miner_1`

### Step 5: Node Discovery

The system discovers the previous miner instance.

**File:** `wait-for-node.sh` (Lines 68-146)

```bash
CHECK_INSTANCE=${PREV_INSTANCE}  # CHECK_INSTANCE=1

while [ ${CHECK_INSTANCE} -ge 1 ]; do
    CHECK_HOSTNAME="miner"  # Use service name
    CHECK_PORT=$((2001 + CHECK_INSTANCE - 1))  # CHECK_PORT=2001
    
    # Check if miner:2001 is ready
    if nc -z -w 1 "miner" "2001" 2>/dev/null; then
        READY=true
        break
    fi
done

# Output: PREV_NODE_ADDRESS=miner_1:2001
```

**Result:**
- `PREV_NODE_ADDRESS=miner_1:2001`

### Step 6: Address Resolution

The system converts and resolves the address to an IP.

**File:** `docker-entrypoint.sh` (Lines 463-475)

```bash
PREV_ADDR="miner_1:2001"

# Convert miner_1 to miner for DNS resolution
if [[ "${PREV_ADDR}" =~ ^miner_([0-9]+): ]]; then
    RESOLVE_ADDR="miner:2001"  # Converted for DNS
fi

# Resolve to IP
PREV_ADDR_RESOLVED=$(resolve_hostname_to_ip "miner:2001")
# PREV_ADDR_RESOLVED="172.19.0.2:2001"
```

### Step 7: Miner Connection Configuration

The miner is configured to connect to the previous miner.

**File:** `docker-entrypoint.sh` (Lines 480-492)

```bash
if [ "${NODE_IS_MINER}" = "yes" ]; then
    # Miners connect to previous miner
    if [ -z "${NODE_CONNECT_NODES}" ] || [ "${NODE_CONNECT_NODES}" = "local" ]; then
        NODE_CONNECT_NODES="${PREV_ADDR_RESOLVED}"  # NODE_CONNECT_NODES="172.19.0.2:2001"
        echo "  Auto-configured connect nodes: miner_1:2001 -> 172.19.0.2:2001"
    fi
fi
```

**Result:**
- `NODE_CONNECT_NODES="172.19.0.2:2001"`
- `miner_2` will connect to `miner_1`

---

## Scenario 3: Instance Number Validation

The system includes multiple validation layers to ensure instance numbers are always valid. This prevents errors that could occur from invalid configuration or edge cases.

### Case 1: Webserver Instance 1 (Edge Case)

When the first webserver instance starts, the system ensures it waits for `miner_1` rather than attempting to connect to a non-existent `miner_0`.

**Initial State:**
```bash
CONTAINER_NAME="configs-webserver-1"
INSTANCE_NUMBER=1
NODE_IS_WEB_SERVER=yes
```

**Wait Script Calculation:**

**File:** `wait-for-node.sh` (Lines 25-40)

```bash
# For webservers, WAIT_INSTANCE_NUMBER = INSTANCE_NUMBER + 1
# So WAIT_INSTANCE_NUMBER = 2

PREV_INSTANCE=$((INSTANCE_NUMBER - 1))  # PREV_INSTANCE = 2 - 1 = 1

# CRITICAL: Ensure PREV_INSTANCE is never less than 1
if [ ${PREV_INSTANCE} -lt 1 ]; then
    # Validation prevents invalid instance numbers
fi
```

**What happens:**
- The calculation ensures `PREV_INSTANCE` is always >= 1
- Webservers always wait for a valid miner instance
- This prevents connection attempts to non-existent nodes

**Result:** `PREV_INSTANCE=1` (valid, prevents `miner_0`)

### Case 2: Miner Instance 1 (Seed Node)

The first miner instance (seed node) doesn't need to wait for a previous node, so it exits early.

**Initial State:**
```bash
CONTAINER_NAME="configs-miner-1"
INSTANCE_NUMBER=1
NODE_IS_MINER=yes
```

**Address Construction Validation:**

**File:** `docker-entrypoint.sh` (Lines 195-219)

```bash
construct_prev_addr() {
    local instance_number=1
    local prev_instance=$((instance_number - 1))  # prev_instance=0
    
    if [ ${prev_instance} -lt 1 ]; then
        echo "ERROR: Cannot connect to miner_0 - miners start at instance 1" >&2
        return 1  # Exit with error
    fi
}
```

**What happens:**
- The function validates that instance numbers are always >= 1
- Attempts to construct addresses for `miner_0` are rejected
- This prevents invalid connection attempts

**Early Exit in Wait Script:**

**File:** `wait-for-node.sh` (Lines 42-47)

```bash
INSTANCE_NUMBER=1
IS_WEBSERVER="no"

# If this is instance 1 and NOT a webserver, no need to wait
if [ "${INSTANCE_NUMBER}" -eq 1 ] && [ "${IS_WEBSERVER}" != "yes" ]; then
    echo "Instance 1: First instance (not webserver), no need to wait"
    exit 0  # Exit early, no wait needed
fi
```

**What happens:**
- The first miner instance exits early since there's no previous node to wait for
- This optimizes startup time for the seed node
- Webservers always wait, even if they're the first instance

**Result:** `miner_1` exits early, doesn't attempt to connect to `miner_0`

### Case 3: Loop Bounds Validation

The discovery loop includes bounds checking to prevent checking invalid instance numbers.

**File:** `wait-for-node.sh` (Lines 79-144)

```bash
CHECK_INSTANCE=${PREV_INSTANCE}  # CHECK_INSTANCE=1

# Ensure we never check invalid instance numbers
if [ ${CHECK_INSTANCE} -lt 1 ]; then
    CHECK_INSTANCE=1  # Force to valid value
fi

while [ ${CHECK_INSTANCE} -ge 1 ]; do
    # ... check logic ...
    
    CHECK_INSTANCE=$((CHECK_INSTANCE - 1))
    
    # Never check invalid instances - stop at miner_1
    if [ ${CHECK_INSTANCE} -lt 1 ]; then
        echo "  ERROR: Reached invalid instance - miners start at instance 1" >&2
        break  # Exit loop, prevent checking invalid instances
    fi
done
```

**What happens:**
- The loop bounds ensure only valid instance numbers are checked
- The loop stops before checking invalid instances
- Multiple validation layers provide defense in depth

**Result:** Loop stops before checking invalid instances

---

## Scenario 4: DNS Resolution Process

The system uses a two-step DNS resolution process to work with Docker Compose's DNS behavior. This section explains how the system handles the difference between service names and instance names.

### Docker Compose DNS Behavior

Docker Compose's internal DNS system resolves service names (defined in `docker-compose.yml`) but not instance names (container-specific names).

**Docker Compose DNS:**
- ✅ Resolves `miner` → IP address (service name)
- ❌ Does NOT resolve `miner_1` → "Name or service not known" (instance name)

### Two-Step Resolution Process

**Step 1: Use Service Name for DNS Lookup**

**File:** `wait-for-node.sh` (Lines 80-82)

```bash
# Use service name "miner" for Docker Compose DNS resolution
CHECK_HOSTNAME="miner"  # Service name, not instance name
CHECK_PORT=$((2001 + CHECK_INSTANCE - 1))

# DNS lookup: getent hosts "miner"
# Result: 172.19.0.2    miner
```

**What happens:**
- The wait script uses the service name for DNS resolution
- Docker Compose DNS successfully resolves the service name
- Port checks verify the service is actually listening

**Step 2: Output Instance Name for Clarity**

**File:** `wait-for-node.sh` (Lines 131-132)

```bash
# Output uses instance name for clarity
OUTPUT_HOSTNAME="miner_${CHECK_INSTANCE}"  # OUTPUT_HOSTNAME="miner_1"
echo "PREV_NODE_ADDRESS=${OUTPUT_HOSTNAME}:${PREV_PORT}"  # miner_1:2001
```

**What happens:**
- The output uses instance names for clarity in logs and configuration
- Instance names are more descriptive than service names
- This format is easier to understand when reading logs

**Step 3: Convert Back to Service Name for Resolution**

**File:** `docker-entrypoint.sh` (Lines 463-470)

```bash
PREV_ADDR="miner_1:2001"  # From wait script output

# Convert miner_1 to miner for DNS resolution
if [[ "${PREV_ADDR}" =~ ^miner_([0-9]+): ]]; then
    INSTANCE_NUM="${BASH_REMATCH[1]}"  # INSTANCE_NUM=1
    PORT_PART="${PREV_ADDR##*:}"  # PORT_PART=2001
    RESOLVE_ADDR="miner:${PORT_PART}"  # RESOLVE_ADDR="miner:2001"
fi

# Now resolve using service name
PREV_ADDR_RESOLVED=$(resolve_hostname_to_ip "miner:2001")
# Result: 172.19.0.2:2001
```

**What happens:**
- The entrypoint script converts instance names back to service names
- This conversion enables successful DNS resolution
- The resolved IP address is used for the final connection configuration

> **📖 Deep Dive:** For a detailed explanation of this conversion mechanism, see [Section 12: DNS Resolution Mechanism](12-DNS-Resolution-Mechanism.md).

### Complete Flow Diagram

```
1. Wait Script:
   CHECK_HOSTNAME="miner" (service name)
   ↓
   DNS lookup: getent hosts "miner"
   ↓
   Result: 172.19.0.2
   ↓
   Port check: nc -z miner 2001 ✅
   ↓
   Output: PREV_NODE_ADDRESS=miner_1:2001 (instance name)

2. Entrypoint Script:
   PREV_ADDR="miner_1:2001" (from wait script)
   ↓
   Convert: miner_1:2001 → miner:2001 (for DNS)
   ↓
   DNS lookup: resolve_hostname_to_ip("miner:2001")
   ↓
   Result: 172.19.0.2:2001 (IP address)
   ↓
   NODE_CONNECT_NODES="172.19.0.2:2001" (ready for Rust)
```

---

## Scenario 5: Web UI Build and Deployment

The system uses a multi-stage Docker build process to compile both the Rust blockchain binary and the React web UI, then combines them into a single runtime image.

### Docker Build Process

**Stage 1: Rust Binary Build**

**File:** `Dockerfile` (Lines 1-29)

```dockerfile
FROM rust:1.91.1-slim as rust-builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY bitcoin/src ./bitcoin/src
# ... copy other source files ...
RUN cargo build --release -p blockchain
```

**What happens:**
- The Rust compiler builds the blockchain binary in release mode
- Dependencies are compiled and optimized
- The resulting binary is ready for deployment

**Result:** Rust binary built at `/app/target/release/blockchain`

**Stage 2: React Web UI Build**

**File:** `Dockerfile` (Lines 31-48)

```dockerfile
FROM node:20-slim as web-ui-builder
WORKDIR /app

# Copy package files
COPY bitcoin-web-ui/package.json bitcoin-web-ui/package-lock.json ./bitcoin-web-ui/

# Install dependencies
WORKDIR /app/bitcoin-web-ui
RUN npm ci
# Result: node_modules installed

# Copy source files
COPY bitcoin-web-ui/ ./

# Build React app
RUN npm run build
# Result: dist/ directory created with built files
```

**What happens during `npm run build`:**
```bash
# Runs: tsc && vite build
# 1. TypeScript compilation
# 2. Vite bundling and optimization
# 3. Creates dist/ directory:
#    dist/
#    ├── index.html
#    └── assets/
#        ├── index-*.js
#        └── index-*.css
```

**What happens:**
- Node.js dependencies are installed from the lock file
- TypeScript source files are compiled to JavaScript
- Vite bundles and optimizes the application
- Static assets are generated in the `dist/` directory

**Stage 3: Runtime Image Assembly**

**File:** `Dockerfile` (Lines 50-70)

```dockerfile
FROM debian:bookworm-slim
WORKDIR /app

# Copy binary from Rust builder
COPY --from=rust-builder /app/target/release/blockchain /app/blockchain

# Copy built React web UI from web-ui-builder
COPY --from=web-ui-builder /app/bitcoin-web-ui/dist /app/bitcoin-web-ui/dist
```

**What happens:**
- The final runtime image is based on a minimal Debian image
- The compiled Rust binary is copied from the builder stage
- The built React application is copied from the web UI builder stage
- Only the necessary runtime artifacts are included in the final image

**Result:**
- `/app/blockchain` (Rust binary)
- `/app/bitcoin-web-ui/dist/` (React app)

### Runtime: Serving the Web UI

The Rust web server serves the React application as static files.

**File:** `bitcoin/src/web/routes/web.rs` (Lines 9-20)

```rust
// Try to serve React app from bitcoin-web-ui/dist
let possible_paths = [
    "../bitcoin-web-ui/dist",
    "../../bitcoin-web-ui/dist",
    "bitcoin-web-ui/dist",  // This matches! (relative to /app/)
];

let react_app_path = possible_paths
    .iter()
    .find(|path| std::path::Path::new(path).exists())
    .copied();  // Finds "bitcoin-web-ui/dist"
```

**What happens:**
- The server checks multiple possible paths for the React application
- The first existing path is used for serving static files
- This allows flexibility in different deployment scenarios

**When request comes to `/`:**

```rust
if let Some(path) = react_app_path {
    let index_path = format!("{}/index.html", path);
    // index_path = "bitcoin-web-ui/dist/index.html"
    
    // Serve index.html
    Router::new()
        .route("/", get(serve_react_app))
        .nest_service("/assets", ServeDir::new("bitcoin-web-ui/dist/assets"))
}
```

**What happens:**
- Requests to `/` serve the React application's `index.html`
- Requests to `/assets/*` serve static assets (JavaScript, CSS, images)
- The React application loads and runs in the browser
- API calls from the React app connect to the Rust backend

**Result:**
- `GET /` → Serves `bitcoin-web-ui/dist/index.html`
- `GET /assets/*` → Serves files from `bitcoin-web-ui/dist/assets/`
- React app loads and runs in browser

---

## Summary: System Design Patterns

The deployment system uses several key design patterns to ensure reliable operation.

### Pattern 1: DNS Resolution Conversion

The system converts between instance names and service names to work with Docker Compose DNS:

```
Instance Name (miner_1) → Service Name (miner) → IP Address (172.19.0.2)
     ↑                           ↑                        ↑
  For clarity              For DNS lookup          For Rust binary
```

**Why this pattern:**
- Instance names are clearer for humans (logs, configuration)
- Service names work with Docker Compose DNS
- IP addresses are required by the Rust binary

### Pattern 2: Instance Number Validation

Multiple validation layers ensure instance numbers are always valid:

```
Validation Layers:
1. construct_prev_addr() checks prev_instance >= 1
2. wait-for-node.sh checks PREV_INSTANCE >= 1
3. Loop bounds check CHECK_INSTANCE >= 1
4. validate_and_fix_miner_address() ensures validity
```

**Why this pattern:**
- Defense in depth prevents invalid configurations
- Multiple checks catch edge cases
- Clear error messages help diagnose issues

### Pattern 3: Webserver Auto-Configuration

Webservers are automatically configured to connect to the first miner:

```
Webserver Startup:
1. Wait for miner_1 to be ready
2. Extract PREV_NODE_ADDRESS=miner_1:2001
3. Convert miner_1:2001 → miner:2001 (for DNS)
4. Resolve miner:2001 → 172.19.0.2:2001
5. Set NODE_CONNECT_NODES=172.19.0.2:2001
```

**Why this pattern:**
- Eliminates manual configuration
- Ensures webservers connect to available miners
- Reduces configuration errors

### Pattern 4: Multi-Stage Docker Build

The build process separates compilation from runtime:

```
Stage 1 (rust-builder): Build Rust binary
Stage 2 (web-ui-builder): Build React app
Stage 3 (runtime): Combine binary + React app
```

**Why this pattern:**
- Reduces final image size
- Separates build dependencies from runtime
- Enables efficient layer caching

---

## Testing the Deployment Flow

To verify the deployment flow works correctly:

```bash
# 1. Start containers
cd ci/docker-compose/configs
NODE_MINING_ADDRESS="1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa" docker compose up -d

# 2. Check webserver logs (should show auto-configuration)
docker compose logs webserver | grep "Auto-configured"

# Expected output:
#   Auto-configured webserver to connect to first miner: miner:2001 -> 172.19.0.2:2001

# 3. Check webserver connection configuration
docker compose logs webserver | grep "Connect Nodes"

# Expected output:
#   Connect Nodes: 172.19.0.2:2001

# 4. Verify web UI is accessible
curl http://localhost:8080/

# Expected: HTML with React app (not error message)
```

All these steps should succeed without errors, demonstrating that the deployment system is working correctly.

---

<div align="center">

**📚 [← Section 10: Deployment Guide](10-Deployment-Guide.md)** | **Section 11: Deployment Execution Walkthrough** | **[Section 12: DNS Resolution Mechanism →](12-DNS-Resolution-Mechanism.md)** 📚

</div>
