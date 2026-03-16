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

## Chapter 22, Section 12: DNS Resolution Mechanism

**Part II: Deployment & Operations** | **Chapter 22: Docker Compose Deployment**

<div align="center">

**[← Section 4: Deployment Scenarios & Operations](04-Deployment-Scenarios-and-Operations.md)** | **Section 12: DNS Resolution Mechanism** | **[Chapter 23: Kubernetes →](../kubernetes/README.md)** 

</div>

---

## Overview

This section provides a detailed, line-by-line explanation of the DNS resolution mechanism used in the deployment system. Specifically, it covers how the system converts instance names like `miner_1:2001` to service names like `miner:2001` for DNS resolution within Docker Compose networks. Understanding this mechanism is essential for troubleshooting network connectivity issues and for developers who need to extend or modify the deployment system.

> **Methods involved**
> - `resolve_hostname_to_ip` (`ci/docker-compose/configs/docker-entrypoint.sh`, [Listing 8.2](01A-Docker-Compose-Code-Listings.md#listing-82-cidocker-composeconfigsdocker-entrypointsh))
> - Conversion of `miner_N` → `miner` for Docker DNS (`ci/docker-compose/configs/docker-entrypoint.sh`, [Listing 8.2](01A-Docker-Compose-Code-Listings.md#listing-82-cidocker-composeconfigsdocker-entrypointsh))

**Prerequisites:**
- Understanding of [Section 4: Deployment Scenarios & Operations](04-Deployment-Scenarios-and-Operations.md), especially Scenario 1, Step 9
- Basic knowledge of Docker Compose networking and DNS
- Familiarity with Bash scripting and regular expressions

**Related Sections:**
- [Section 4: Deployment Scenarios & Operations](04-Deployment-Scenarios-and-Operations.md) - Step-by-step execution flow
- [Section 3: Deployment Topology](03-Deployment-Topology.md) - Network setup and topology context

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
# Docker service names with underscores (e.g., miner_1)
# need to be resolved to IP
# For Docker Compose, "miner_1" doesn't resolve, but "miner" does
# So we convert miner_1 to miner for resolution purposes
RESOLVE_ADDR="${PREV_ADDR}"
if [[ "${PREV_ADDR}" =~ ^miner_([0-23]+): ]]; then
    # Extract instance number and use "miner" service name for resolution
    INSTANCE_NUM="${BASH_REMATCH[1]}"
    PORT_PART="${PREV_ADDR##*:}"
    RESOLVE_ADDR="miner:${PORT_PART}"
    debug_log "Converting ${PREV_ADDR} to " \
             "${RESOLVE_ADDR} for Docker Compose " \
             "DNS resolution"
fi

if ! PREV_ADDR_RESOLVED=$(resolve_hostname_to_ip \
   "${RESOLVE_ADDR}"); then
    echo "ERROR: Failed to resolve previous node " \
         "address '${RESOLVE_ADDR}' (from " \
         "'${PREV_ADDR}')" >&2
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
if [[ "${PREV_ADDR}" =~ ^miner_([0-23]+): ]]; then
```

**What it does:**
- Uses Bash's `=~` operator for regular expression pattern matching
- Checks if `PREV_ADDR` matches the pattern `^miner_([0-23]+):`

**Pattern Breakdown:**

| Part | Meaning | Example Match |
|------|---------|---------------|
| `^` | Start of string anchor | Ensures match starts at beginning |
| `miner_` | Literal text "miner_" | Matches "miner_" |
| `([0-23]+)` | **Capture group**: One or more digits | Matches "1", "2", "123" |
| `:` | Literal colon | Matches ":" |
| (no `$`) | No end anchor | Allows port number after colon |

**What Gets Captured:**
- The `([0-23]+)` part is a **capture group**
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
debug_log "Converting ${PREV_ADDR} to " \
         "${RESOLVE_ADDR} for Docker Compose " \
         "DNS resolution"
```

This logs the conversion for debugging when
`DEBUG_MODE=yes`.

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

The authoritative implementation is printed in full in [Listing 8.2](01A-Docker-Compose-Code-Listings.md#listing-82-cidocker-composeconfigsdocker-entrypointsh). Conceptually, the function does three things:

1. **Parse** `hostname:port` into parts (with input validation and trimming).
2. **Resolve** `hostname` using multiple strategies (preferring `getent hosts`, with retry/backoff).
3. **Return** `ip:port` so the Rust binary receives a parseable `SocketAddr`.

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
    echo "ERROR: Failed to resolve previous node " \
         "address '${RESOLVE_ADDR}' (from " \
         "'${PREV_ADDR}')" >&2
    exit 1
fi
```

**What it does:**
- If DNS resolution fails, prints an error message to stderr (`>&2`)
- Shows both the converted address (`RESOLVE_ADDR`) and original (`PREV_ADDR`)
- Exits with error code 1 to signal failure

**Example Error Output:**
```text
ERROR: Failed to resolve previous node address
  'miner:2001' (from 'miner_1:2001')
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
if [[ "${PREV_ADDR}" =~ ^miner_([0-23]+): ]]; then
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
debug_log "Converting miner_1:2001 to " \
         "miner:2001 for Docker Compose DNS " \
         "resolution"
# Output: DEBUG: Converting miner_1:2001 to
# miner:2001 for Docker Compose DNS resolution
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

The conversion mechanism uses five key principles:

1. **Leverages Docker Compose DNS**: Service names Docker knows about (no custom config)
2. **Preserves Port Information**: Converts hostname but keeps port (`miner_1:2001` → `miner:2001`)
3. **Handles Multiple Instances**: First instance accessible via service name
4. **Safe Fallback**: Uses addresses as-is if they don't match the pattern (handles IPs)
5. **Clear Error Messages**: Shows both original and converted addresses for debugging

---

## Edge Cases and Error Handling

### Case 1: Already an IP Address

```bash
PREV_ADDR="172.19.0.2:2001"
# Pattern doesn't match → RESOLVE_ADDR stays unchanged
# Resolution function detects it's an IP, returns as-is ✅
```

### Case 2: Different Service Name

```bash
PREV_ADDR="webserver_1:8080"
# Pattern doesn't match → uses address as-is
# System doesn't interfere with other service types
```

### Case 3: No Port Number

```bash
PREV_ADDR="miner_1"
# Pattern doesn't match (no colon)
# Resolution fails gracefully with error ✅
```

---

## Testing and Verification

Test the DNS resolution conversion mechanism:

```bash
PREV_ADDR="miner_1:2001"

if [[ "${PREV_ADDR}" =~ ^miner_([0-23]+): ]]; then
    PORT_PART="${PREV_ADDR##*:}"
    RESOLVE_ADDR="miner:${PORT_PART}"
    echo "Converted: ${PREV_ADDR} → ${RESOLVE_ADDR}"
fi

getent hosts "miner"      # Should resolve to IP
getent hosts "miner_1"    # Should fail
```

Verification:
1. Start the Docker Compose deployment
2. Check logs for conversion messages (DEBUG_MODE)
3. Verify nodes connect successfully

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

**[← Section 4: Deployment Scenarios & Operations](04-Deployment-Scenarios-and-Operations.md)** | **Section 12: DNS Resolution Mechanism** | **[Chapter 23: Kubernetes →](../kubernetes/README.md)** 

</div>

---


