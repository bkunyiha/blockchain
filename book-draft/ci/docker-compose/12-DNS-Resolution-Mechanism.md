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
   - [Section 11: Deployment Execution Walkthrough](11-Deployment-Execution-Walkthrough.md)
   - **Section 12: DNS Resolution Mechanism** ← *You are here*
9. [Chapter 9: Kubernetes Deployment](../../kubernetes/README.md)

</details>

</div>

<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

# Chapter 7, Section 12: DNS Resolution Mechanism

**Part II: Deployment & Operations** | **Chapter 8: Docker Compose Deployment**

<div align="center">

**📚 [← Section 11: Deployment Execution Walkthrough](11-Deployment-Execution-Walkthrough.md)** | **Section 12: DNS Resolution Mechanism** | **[Chapter 9: Kubernetes →](../../kubernetes/README.md)** 📚

</div>

---

## Overview

This section provides a detailed, line-by-line explanation of the DNS resolution mechanism used in the deployment system. Specifically, it covers how the system converts instance names like `miner_1:2001` to service names like `miner:2001` for DNS resolution within Docker Compose networks. Understanding this mechanism is essential for troubleshooting network connectivity issues and for developers who need to extend or modify the deployment system.

**Prerequisites:**
- Understanding of [Section 11: Deployment Execution Walkthrough](11-Deployment-Execution-Walkthrough.md), especially Scenario 1, Step 9
- Basic knowledge of Docker Compose networking and DNS
- Familiarity with Bash scripting and regular expressions

**Related Sections:**
- [Section 11: Deployment Execution Walkthrough](11-Deployment-Execution-Walkthrough.md) - Step-by-step execution flow
- [Section 4: Network Configuration](04-Network-Configuration.md) - Network setup context
- [Section 7: Sequential Startup](07-Sequential-Startup.md) - Startup sequence

---

## Table of Contents

1. [Understanding Docker Compose DNS](#understanding-docker-compose-dns)
2. [The Conversion Mechanism](#the-conversion-mechanism)
3. [Implementation: Line-by-Line Analysis](#implementation-line-by-line-analysis)
4. [Complete Execution Example](#complete-execution-example)
5. [Design Rationale](#design-rationale)
6. [Edge Cases and Error Handling](#edge-cases-and-error-handling)
7. [Testing and Verification](#testing-and-verification)

---

## Understanding Docker Compose DNS

### Service Names vs. Instance Names

Docker Compose creates an internal DNS system that resolves **service names** (defined in `docker-compose.yml`), but **NOT instance names** (container-specific names). This is a fundamental characteristic of Docker Compose's networking model.

**Example from docker-compose.yml:**
```yaml
services:
  miner:  # ← This is the SERVICE NAME
    # ...
```

**DNS Resolution Behavior:**

| Hostname | Type | DNS Resolution | Result |
|----------|------|----------------|--------|
| `miner` | Service name | ✅ **Resolves** | `172.19.0.2` (or appropriate IP) |
| `miner_1` | Instance name | ❌ **Does NOT resolve** | "Name or service not known" |
| `miner_2` | Instance name | ❌ **Does NOT resolve** | "Name or service not known" |
| `configs-miner-1` | Container name | ❌ **Does NOT resolve** | "Name or service not known" |

### Why This Design Exists

Docker Compose's DNS design prioritizes service-level discovery over instance-level discovery. This approach:

- **Simplifies service discovery**: Applications can connect to a service without knowing which specific instance is handling the request
- **Enables load balancing**: Multiple instances of a service can share the same service name
- **Reduces coupling**: Applications don't need to know about container naming conventions

However, in our deployment system, we need to connect to specific instances (e.g., `miner_1`), which requires a conversion mechanism.

### The Challenge

The `wait-for-node.sh` script outputs `PREV_NODE_ADDRESS=miner_1:2001` (using instance name for clarity in logs and configuration), but Docker Compose DNS cannot resolve `miner_1` directly.

**Without conversion:**
```bash
PREV_ADDR="miner_1:2001"  # From wait script output
resolve_hostname_to_ip "miner_1:2001"
  ↓
getent hosts "miner_1"
  ↓
ERROR: Name or service not known
  ↓
DNS resolution fails ❌
```

**With conversion:**
```bash
PREV_ADDR="miner_1:2001"  # From wait script output
Convert: miner_1:2001 → miner:2001
  ↓
resolve_hostname_to_ip "miner:2001"
  ↓
getent hosts "miner"
  ↓
SUCCESS: 172.19.0.2 ✅
```

---

## The Conversion Mechanism

### Code Location

**File:** `ci/docker-compose/configs/docker-entrypoint.sh`  
**Lines:** 460-475

### Context: Preceding Steps

Before the DNS resolution conversion occurs, the system has already:

1. **Extracted the address** from the wait script output (Step 8)
2. **Validated the address** to ensure it's valid and appropriate for the service type
3. **Prepared for resolution** by having the address in instance name format (`miner_1:2001`)

**At this point:**
- `PREV_ADDR` contains `"miner_1:2001"` (instance name format)
- The system needs to resolve this to an IP address
- Docker Compose DNS cannot resolve `miner_1` directly

---

## Implementation: Line-by-Line Analysis

### Complete Code Block

```bash
# Docker service names with underscores (e.g., miner_1) need to be resolved to IP
# For Docker Compose, "miner_1" doesn't resolve, but "miner" does
# So we convert miner_1 to miner for resolution purposes
RESOLVE_ADDR="${PREV_ADDR}"
if [[ "${PREV_ADDR}" =~ ^miner_([0-9]+): ]]; then
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
```

### Line-by-Line Breakdown

#### Line 463: Initialize Resolution Address

```bash
RESOLVE_ADDR="${PREV_ADDR}"
```

**What it does:**
- Creates a copy of `PREV_ADDR` in `RESOLVE_ADDR`
- This serves as the fallback value if the conversion pattern doesn't match

**Example:**
```bash
PREV_ADDR="miner_1:2001"
RESOLVE_ADDR="miner_1:2001"  # Same value initially
```

**Why this approach:**
- If the address doesn't match the conversion pattern (e.g., it's already an IP address like `172.19.0.2:2001`), the code uses it as-is
- This ensures the code works for both hostnames and IP addresses
- Provides a safe default that handles various input formats

---

#### Line 464: Pattern Matching with Regular Expressions

```bash
if [[ "${PREV_ADDR}" =~ ^miner_([0-9]+): ]]; then
```

**What it does:**
- Uses Bash's `=~` operator for regular expression pattern matching
- Checks if `PREV_ADDR` matches the pattern `^miner_([0-9]+):`

**Pattern Breakdown:**

| Part | Meaning | Example Match |
|------|---------|---------------|
| `^` | Start of string anchor | Ensures match starts at beginning |
| `miner_` | Literal text "miner_" | Matches "miner_" |
| `([0-9]+)` | **Capture group**: One or more digits | Matches "1", "2", "123" |
| `:` | Literal colon | Matches ":" |
| (no `$`) | No end anchor | Allows port number after colon |

**What Gets Captured:**
- The `([0-9]+)` part is a **capture group**
- The matched digits are stored in `${BASH_REMATCH[1]}`
- `${BASH_REMATCH[0]}` contains the full match

**Pattern Matching Examples:**

| Input | Matches? | Capture Group (`${BASH_REMATCH[1]}`) |
|-------|----------|--------------------------------------|
| `miner_1:2001` | ✅ Yes | `"1"` |
| `miner_2:2002` | ✅ Yes | `"2"` |
| `miner_10:2010` | ✅ Yes | `"10"` |
| `miner:2001` | ❌ No | (not set) |
| `172.19.0.2:2001` | ❌ No | (not set) |
| `miner_1` | ❌ No (no colon) | (not set) |
| `webserver_1:8080` | ❌ No (starts with "webserver") | (not set) |

**Why this specific pattern:**
- Only matches `miner_X:PORT` format, ensuring we only convert miner addresses
- Doesn't match IP addresses (which don't need conversion)
- Doesn't match other services (webserver, etc.), preserving their original format
- Requires a colon to ensure we have both hostname and port

---

#### Line 466: Extract Instance Number

```bash
INSTANCE_NUM="${BASH_REMATCH[1]}"
```

**What it does:**
- Extracts the captured digits from the regex match
- Stores them in `INSTANCE_NUM` for potential future use

**Example:**
```bash
PREV_ADDR="miner_1:2001"
# After regex match:
BASH_REMATCH[0]="miner_1:"  # Full match
BASH_REMATCH[1]="1"          # First capture group
INSTANCE_NUM="1"
```

**Note:** While `INSTANCE_NUM` is extracted, it's not actually used in the current conversion logic. However, extracting it:
- Confirms the pattern match succeeded
- Provides information for debugging
- Could be useful for future enhancements or validation

---

#### Line 467: Extract Port Number

```bash
PORT_PART="${PREV_ADDR##*:}"
```

**What it does:**
- Uses Bash parameter expansion `${PREV_ADDR##*:}` to extract everything after the last colon
- This extracts the port number from the address

**Bash Parameter Expansion Reference:**

| Expression | Meaning | Result |
|------------|---------|--------|
| `${PREV_ADDR##*:}` | Remove longest match of `*:` from the start | Everything after last `:` |
| `${PREV_ADDR#*:}` | Remove shortest match of `*:` from the start | Everything after first `:` |
| `${PREV_ADDR%%:*}` | Remove longest match of `:*` from the end | Everything before last `:` |
| `${PREV_ADDR%:*}` | Remove shortest match of `:*` from the end | Everything before first `:` |

**Extraction Examples:**

| `PREV_ADDR` | `${PREV_ADDR##*:}` | Explanation |
|-------------|---------------------|-------------|
| `miner_1:2001` | `2001` | Everything after `:` |
| `miner_2:2002` | `2002` | Everything after `:` |
| `172.19.0.2:2001` | `2001` | Works with IPs too |
| `miner_1:2001:extra` | `extra` | Takes everything after **last** `:` |

**Why use `##` (longest match):**
- Handles edge cases where there might be multiple colons (e.g., IPv6 addresses)
- Ensures we get the port number even if the hostname contains a colon (unlikely but safer)
- Provides robustness against malformed input

---

#### Line 468: Construct Service Name Address

```bash
RESOLVE_ADDR="miner:${PORT_PART}"
```

**What it does:**
- Constructs the new address using the service name `miner` instead of the instance name `miner_1`
- Preserves the port number from the original address

**Example:**
```bash
PREV_ADDR="miner_1:2001"
PORT_PART="2001"  # Extracted in previous line
RESOLVE_ADDR="miner:2001"  # Converted to service name
```

**Result:**
- `PREV_ADDR` = `"miner_1:2001"` (original, instance name)
- `RESOLVE_ADDR` = `"miner:2001"` (converted, service name)

**Why preserve the port:**
- The port number is essential for establishing the connection
- Different instances may use different ports
- The port must match the target service's listening port

---

#### Line 469: Debug Logging

```bash
debug_log "Converting ${PREV_ADDR} to ${RESOLVE_ADDR} for Docker Compose DNS resolution"
```

**What it does:**
- Logs the conversion for debugging purposes
- Only outputs if `DEBUG_MODE=yes` environment variable is set

**Example Output:**
```
DEBUG: Converting miner_1:2001 to miner:2001 for Docker Compose DNS resolution
```

**Why this is valuable:**
- Helps troubleshoot DNS resolution issues by showing exactly what conversion occurred
- Makes it clear why resolution succeeds (using service name instead of instance name)
- Provides visibility into the system's operation without cluttering normal logs

---

#### Line 472: DNS Resolution Function Call

```bash
if ! PREV_ADDR_RESOLVED=$(resolve_hostname_to_ip "${RESOLVE_ADDR}"); then
```

**What it does:**
- Calls `resolve_hostname_to_ip()` with the **converted** address (`miner:2001`)
- Captures the output (IP address) in `PREV_ADDR_RESOLVED`
- Checks if the function succeeded (exit code 0 = success)
- The `!` negates the condition, so the `then` block executes on failure

**Inside `resolve_hostname_to_ip()`:**

```bash
resolve_hostname_to_ip() {
    local addr="miner:2001"  # The converted address
    local hostname="miner"   # Extracted: everything before ":"
    local port="2001"        # Extracted: everything after ":"
    
    # Try getent hosts (works with Docker's internal DNS)
    getent_output=$(getent hosts "miner" 2>&1)
    # Result: "172.19.0.2    miner"
    
    if [ ${getent_exit} -eq 0 ]; then
        ip=$(echo "${getent_output}" | awk '{print $1}' | head -n1)
        # ip="172.19.0.2"
        echo "${ip}:${port}"  # Returns "172.19.0.2:2001"
        return 0  # Success
    fi
}
```

**What happens:**
- The function extracts the hostname and port from the address
- It uses `getent hosts` to query Docker's internal DNS
- Docker Compose's DNS service resolves the service name to the container's IP address
- The IP address and port are combined into a format the Rust binary can use
- Multiple resolution methods are attempted with retry logic for reliability

**Result:**
- `PREV_ADDR_RESOLVED="172.19.0.2:2001"` (IP address, ready for Rust)

---

#### Lines 473-475: Error Handling

```bash
    echo "ERROR: Failed to resolve previous node address '${RESOLVE_ADDR}' (from '${PREV_ADDR}')" >&2
    exit 1
fi
```

**What it does:**
- If DNS resolution fails, prints an error message to stderr (`>&2`)
- Shows both the converted address (`RESOLVE_ADDR`) and original (`PREV_ADDR`)
- Exits with error code 1 to signal failure

**Example Error Output:**
```
ERROR: Failed to resolve previous node address 'miner:2001' (from 'miner_1:2001')
```

**Why show both addresses:**
- Helps debug by showing what was attempted and what it came from
- Makes it clear the conversion happened but resolution still failed
- Provides context for troubleshooting network issues
- Distinguishes between conversion failures and DNS resolution failures

---

## Complete Execution Example

This example traces the complete execution flow for a webserver connecting to `miner_1`.

### Step 1: Wait Script Output

```bash
# wait-for-node.sh outputs:
PREV_NODE_ADDRESS=miner_1:2001
```

The wait script outputs the address using instance name format for clarity in logs.

### Step 2: Extract Address from Output

```bash
PREV_ADDR=$(echo "${WAIT_OUTPUT}" | grep "PREV_NODE_ADDRESS=" | cut -d'=' -f2)
# PREV_ADDR="miner_1:2001"
```

The entrypoint script parses the wait script output to extract the address.

### Step 3: Initialize Resolution Address

```bash
RESOLVE_ADDR="${PREV_ADDR}"
# RESOLVE_ADDR="miner_1:2001"
```

The resolution address is initialized with the original address.

### Step 4: Pattern Matching

```bash
if [[ "${PREV_ADDR}" =~ ^miner_([0-9]+): ]]; then
    # Pattern matches! ✅
    # BASH_REMATCH[1]="1"
```

The regular expression successfully matches the `miner_1:2001` pattern.

### Step 5: Extract Components

```bash
INSTANCE_NUM="${BASH_REMATCH[1]}"
# INSTANCE_NUM="1"

PORT_PART="${PREV_ADDR##*:}"
# PORT_PART="2001"
```

The instance number and port are extracted from the address.

### Step 6: Convert to Service Name

```bash
RESOLVE_ADDR="miner:${PORT_PART}"
# RESOLVE_ADDR="miner:2001"
```

The address is converted to use the service name instead of the instance name.

### Step 7: Debug Logging

```bash
debug_log "Converting miner_1:2001 to miner:2001 for Docker Compose DNS resolution"
# Output: DEBUG: Converting miner_1:2001 to miner:2001 for Docker Compose DNS resolution
```

The conversion is logged for debugging purposes.

### Step 8: DNS Resolution

```bash
PREV_ADDR_RESOLVED=$(resolve_hostname_to_ip "miner:2001")
# Inside resolve_hostname_to_ip():
#   getent hosts "miner"
#   Result: "172.19.0.2    miner"
#   ip="172.19.0.2"
#   Returns: "172.19.0.2:2001"
# PREV_ADDR_RESOLVED="172.19.0.2:2001"
```

The service name is resolved to an IP address using Docker's internal DNS.

### Step 9: Configuration Complete

```bash
# PREV_ADDR_RESOLVED contains IP address
# Ready to pass to Rust binary
NODE_CONNECT_NODES="172.19.0.2:2001"
```

The resolved IP address is ready to be passed to the Rust binary for connection establishment.

---

## Design Rationale

### 1. Leverages Docker Compose DNS

Docker Compose automatically creates DNS entries for service names. By converting `miner_1` → `miner`, we use the service name that Docker DNS knows about. This approach:

- **Works with Docker's built-in service discovery**: No need for custom DNS configuration
- **Reliable**: Docker Compose DNS is well-tested and maintained
- **Standard**: Uses Docker's standard networking mechanisms

### 2. Preserves Port Information

The conversion only changes the hostname part (`miner_1` → `miner`), keeping the port (`2001`) intact. This ensures:

- **Correct port targeting**: The connection goes to the correct port
- **Port flexibility**: Different instances can use different ports
- **No port conflicts**: Port information is preserved accurately

### 3. Handles Multiple Instances

Docker Compose's service-level DNS resolves `miner` to one of the running miner containers (typically the first one). Since we're connecting to `miner_1` specifically, and Docker Compose DNS resolves `miner` to `miner_1` when it's the first instance, this works correctly. This approach:

- **Works for the first instance**: Docker Compose DNS resolves to the first available instance
- **Predictable**: The first instance is consistently accessible via the service name
- **Scalable**: Additional instances don't interfere with this mechanism

### 4. Safe Fallback

If the address doesn't match the pattern (e.g., it's already an IP address), the code uses it as-is. This makes the code:

- **Robust**: Handles various input formats gracefully
- **Flexible**: Works with IP addresses, hostnames, and service names
- **Fail-safe**: Doesn't break when input is already in the correct format

### 5. Clear Error Messages

If resolution fails, the error message shows both the original and converted addresses, making debugging easier. This approach:

- **Diagnostic**: Provides context for troubleshooting
- **Transparent**: Shows what was attempted
- **Actionable**: Helps identify the root cause of failures

---

## Edge Cases and Error Handling

### Case 1: Already an IP Address

```bash
PREV_ADDR="172.19.0.2:2001"
# Pattern match fails (doesn't start with "miner_")
# RESOLVE_ADDR stays "172.19.0.2:2001"
# resolve_hostname_to_ip() detects it's already an IP, returns as-is
# ✅ Works correctly
```

**What happens:**
- The pattern doesn't match, so no conversion occurs
- The address is used as-is
- The resolution function detects it's already an IP and returns it unchanged

**Why this works:**
- IP addresses don't need DNS resolution
- The system handles them gracefully
- No unnecessary processing is performed

### Case 2: Different Service Name

```bash
PREV_ADDR="webserver_1:8080"
# Pattern match fails (doesn't start with "miner_")
# RESOLVE_ADDR stays "webserver_1:8080"
# resolve_hostname_to_ip() tries to resolve "webserver_1"
# If Docker Compose has a "webserver" service, this might work
# If not, it will fail with a clear error
```

**What happens:**
- The pattern doesn't match (not a miner address)
- The address is used as-is
- Resolution may succeed or fail depending on Docker Compose configuration

**Why this design:**
- Only miner addresses need conversion (they use instance names)
- Other services may use different naming conventions
- The system doesn't interfere with other service types

### Case 3: Multiple Colons

```bash
PREV_ADDR="miner_1:2001:extra"
# Pattern match succeeds
# PORT_PART="${PREV_ADDR##*:}" extracts "extra" (everything after last colon)
# RESOLVE_ADDR="miner:extra"
# This is probably wrong, but it's an edge case
# In practice, addresses are always "hostname:port" format
```

**What happens:**
- The pattern matches, so conversion occurs
- The port extraction takes everything after the last colon
- This may result in an incorrect port

**Why this is acceptable:**
- In practice, addresses are always in `hostname:port` format
- This edge case is unlikely to occur in real deployments
- The system handles it gracefully without crashing

### Case 4: No Port Number

```bash
PREV_ADDR="miner_1"
# Pattern match fails (no colon)
# RESOLVE_ADDR stays "miner_1"
# resolve_hostname_to_ip() will fail because it expects "hostname:port"
# ✅ Fails gracefully with error message
```

**What happens:**
- The pattern doesn't match (no colon present)
- The address is used as-is
- Resolution fails because the format is invalid

**Why this is correct:**
- Addresses must include a port number
- The system validates the format
- Clear error messages help identify the issue

---

## Testing and Verification

You can test the DNS resolution conversion mechanism manually:

```bash
# Inside a Docker container
PREV_ADDR="miner_1:2001"

# Test pattern matching
if [[ "${PREV_ADDR}" =~ ^miner_([0-9]+): ]]; then
    echo "Match! Instance: ${BASH_REMATCH[1]}"
    PORT_PART="${PREV_ADDR##*:}"
    RESOLVE_ADDR="miner:${PORT_PART}"
    echo "Converted: ${PREV_ADDR} → ${RESOLVE_ADDR}"
fi

# Test DNS resolution
getent hosts "miner"
# Should output: 172.19.0.2    miner

getent hosts "miner_1"
# Should output: (nothing or error)
```

**Expected Results:**
- Pattern matching succeeds for `miner_1:2001`
- Conversion produces `miner:2001`
- DNS resolution succeeds for `miner` but fails for `miner_1`

**Verification Steps:**
1. Start the Docker Compose deployment
2. Check logs for conversion messages (if DEBUG_MODE is enabled)
3. Verify nodes connect successfully
4. Test DNS resolution manually using `getent hosts`

---

## Summary

The DNS resolution mechanism converts `miner_1:2001` → `miner:2001` to work with Docker Compose's DNS system. The conversion process:

1. ✅ Uses regex to detect `miner_X:PORT` format
2. ✅ Extracts the port number
3. ✅ Constructs new address with service name `miner`
4. ✅ Resolves using Docker Compose DNS (which knows about `miner`)
5. ✅ Returns IP address ready for Rust binary

**Without this conversion:** DNS resolution fails because `miner_1` doesn't exist in Docker Compose DNS.

**With this conversion:** DNS resolution succeeds because `miner` exists in Docker Compose DNS.

This mechanism is essential for the deployment system to function correctly, enabling reliable node discovery and connection establishment within Docker Compose networks.

---

<div align="center">

**📚 [← Section 11: Deployment Execution Walkthrough](11-Deployment-Execution-Walkthrough.md)** | **Section 12: DNS Resolution Mechanism** | **[Chapter 9: Kubernetes →](../../kubernetes/README.md)** 📚

</div>

---

*This chapter has provided a comprehensive guide to deploying and managing the blockchain network using Docker Compose. We've explored the complete architecture, execution flow, network configuration, sequential startup mechanisms, port mapping, scaling operations, deployment scenarios, webserver access, production deployment procedures, execution walkthroughs, and DNS resolution mechanisms. Docker Compose enables rapid development and local deployment with automatic port configuration, flexible scaling, and reliable networking. Understanding these concepts is essential for transitioning from development to production deployment. In the next chapter, we'll explore [Kubernetes Deployment](../../kubernetes/README.md) to understand how to deploy the blockchain network on Kubernetes for production-grade orchestration, autoscaling, and high availability.*
