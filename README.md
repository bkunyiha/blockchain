# blockchain

---
This is an attempt to re-write bitcoin blockchain using the Rust programming language.
The current opensource bitcoin blockchain is written in C++.
I will be working on a blog post with the technical details in the near future.

## Running Multiple Nodes on the Same Server

Each node requires its own set of environment variables. Below are example configurations for five nodes. You will need five separate terminal windows, each with its own environment settings.

```sh
# Node 1
export CENTERAL_NODE=127.0.0.1:2001
export BLOCKS_TREE=blocks1
export TREE_DIR=data1
export NODE_ADDR=127.0.0.1:2001
export RUST_LOG=trace

# Node 2
export CENTERAL_NODE=127.0.0.1:2002
export BLOCKS_TREE=blocks2
export TREE_DIR=data2
export NODE_ADDR=127.0.0.1:2002
export RUST_LOG=trace

# Node 3
export CENTERAL_NODE=127.0.0.1:2003
export BLOCKS_TREE=blocks3
export TREE_DIR=data3
export NODE_ADDR=127.0.0.1:2003
export RUST_LOG=trace

# Node 4
export CENTERAL_NODE=127.0.0.1:2004
export BLOCKS_TREE=blocks4
export TREE_DIR=data4
export NODE_ADDR=127.0.0.1:2004
export RUST_LOG=trace

# Node 5
export CENTERAL_NODE=127.0.0.1:2005
export BLOCKS_TREE=blocks5
export TREE_DIR=data5
export NODE_ADDR=127.0.0.1:2005
export RUST_LOG=trace
```

### Steps to Run a Node

1. Open a new terminal window for each node.
2. Export the environment variables for the node you want to run.
3. (Optional) Create a wallet:
   ```sh
   cargo run createwallet
   ```
   This returns `WALLET_ADDR`.
4. (Optional) Create a blockchain using your wallet address:
   ```sh
   cargo run createblockchain WALLET_ADDR
   ```
5. Start the node:
   ```sh
   cargo run startnode WALLET_ADDR IS_MINER SEED_NODE
   ```
   - `WALLET_ADDR`: The address returned from the wallet creation step.
   - `IS_MINER`: Use `yes` if this node should mine, otherwise `no`.
   - `SEED_NODE`: For the first (seed) node, use `local`. For others, use the address of the seed node (e.g., `127.0.0.1:2001`).

#### Example Commands

Seed Node:
```sh
cargo run startnode yes local -- WALLET_ADDR
```

Second Node:
```sh
cargo run startnode yes 127.0.0.1:2001 -- WALLET_ADDR
```
Third Node:
```sh
cargo run startnode yes 127.0.0.1:2002 -- WALLET_ADDR
```
- Since Node 1 is part of the blockchain, it can be used as the Seed node for the third node.
Repeat these steps in separate terminals for each node you want to run.

---

# TODO's: 

## ✅ **Completed Enhancements:**

### P2TR (Pay-to-Taproot) Implementation
- **✅ Replaced insecure RIPEMD160** with secure SHA256 for P2TR addresses
- **✅ Updated to P2TR address format** (version 0x01) for modern Bitcoin compatibility
- **✅ Implemented true Schnorr signatures** using secp256k1 for enhanced security
- **✅ Added comprehensive testing** with 55 tests including Schnorr roundtrip verification
- **✅ Maintained backward compatibility** with existing codebase

### Consensus Mechanism Implementation
- **✅ Bitcoin-compatible consensus algorithm** with longest chain rule
- **✅ Deterministic tie-breaking system** using hash comparison for network convergence
- **✅ Chain reorganization logic** for proper fork handling and network convergence
- **✅ Chain work calculation** for determining the canonical blockchain
- **✅ UTXO rollback support** for chain reorganization scenarios
- **✅ Fixed block processing order issues** that caused consensus failures
- **✅ Comprehensive consensus testing** with 10+ new test cases
- **✅ Fixed balance inconsistency issues** in multi-node mining scenarios

### Security Improvements
- **✅ Modern cryptographic standards** compatible with Bitcoin's Taproot upgrade
- **✅ Enhanced privacy features** through Schnorr signature implementation
- **✅ Secure random number generation** for cryptographic operations
- **✅ Production-ready implementation** with comprehensive error handling
- **✅ Robust consensus mechanisms** preventing double-spending and ensuring network consistency

## 🔄 **In Progress:**

## 1. Add capability to have multiple database backends, currently only the filesystem is supported.

---

## 🧪 **Testing & Quality Assurance:**

### Comprehensive Test Suite
- **✅ 130+ total tests** covering all major functionality
- **✅ Consensus mechanism tests** validating tie-breaking and chain reorganization
- **✅ Block processing order tests** ensuring network convergence
- **✅ Cryptographic tests** for Schnorr signatures and P2TR addresses
- **✅ Network operation tests** for P2P communication
- **✅ Blockchain persistence tests** for data integrity
- **✅ UTXO management tests** for transaction validation

### Consensus Testing Results
- **✅ Chain reorganization tests**: 3/3 passing
- **✅ Tie-breaking mechanism tests**: 3/3 passing with deterministic hash comparison
- **✅ Work calculation tests**: Validating chain work computation
- **✅ Multi-node scenario tests**: Ensuring network convergence
- **✅ Block processing order tests**: Verifying consensus convergence across nodes

*All consensus tests now pass consistently. The deterministic tie-breaking mechanism ensures all nodes reach the same decision about competing blocks.*

---

## 🏗️ **Architecture Overview:**

### Core Components
- **Block Management**: Genesis block creation, block validation, chain storage
- **Transaction System**: P2TR transactions, Schnorr signatures, UTXO management
- **Consensus Engine**: Multi-level tie-breaking, chain reorganization, work calculation
- **Network Layer**: P2P communication, message handling, node discovery
- **Cryptographic Layer**: ECDSA/Schnorr signatures, SHA256 hashing, address generation

### Consensus Algorithm
The blockchain implements a robust consensus mechanism based on Bitcoin's Nakamoto Consensus:

1. **Longest Chain Rule**: The chain with the most cumulative work is considered valid
2. **Cumulative Work Comparison**: When chains have equal height, compare total proof-of-work
3. **Deterministic Tie-Breaking**: When chains have equal work, use hash comparison:
   - **Lexicographic hash comparison** ensures all nodes reach the same decision
   - **Network convergence** is guaranteed regardless of block processing order
   - **No bias** toward any particular node or timing
4. **Chain Reorganization**: Automatic switching to stronger chains
5. **UTXO Rollback**: Proper state management during chain switches
6. **Mining Reward Distribution**: Only winning blocks receive mining rewards

📖 **Detailed Consensus Documentation**: For comprehensive information about the consensus mechanisms, implementation details, and testing results, see [CONSENSUS_DOCUMENTATION.md](CONSENSUS_DOCUMENTATION.md).

### Security Features
- **Cryptographic Security**: Schnorr signatures with secp256k1 curves
- **Consensus Security**: Protection against double-spending and network attacks
- **Data Integrity**: SHA256 hashing and block validation
- **Network Security**: P2P communication with message validation

### Recent Consensus Improvements
The blockchain has been enhanced with consensus mechanism fixes:

#### **Block Processing Order Issue Resolution**
- **Problem**: Nodes were keeping their own mining rewards due to biased tie-breaking
- **Root Cause**: Timestamp-based comparison created asymmetric decision-making
- **Solution**: Implemented deterministic hash-based tie-breaking
- **Result**: All nodes now converge on the same winning block

#### **Network Convergence Guarantee**
- **Deterministic Decision Making**: All nodes reach identical consensus decisions
- **Unbiased Tie-Breaking**: No node has inherent advantage in consensus
- **Proper Mining Reward Distribution**: Only winning blocks receive rewards
- **Multi-Node Stability**: Network maintains consistency across all nodes

#### **Consensus Algorithm Flow**
1. **Height Check**: Higher height blocks are always accepted (longest chain rule)
2. **Work Comparison**: When heights are equal, compare cumulative proof-of-work
3. **Hash Tie-Breaking**: When work is equal, use deterministic hash comparison
4. **Chain Reorganization**: Automatically switch to the winning chain
5. **UTXO Synchronization**: Ensure all nodes maintain consistent state

## 2. Version enhancements to ensure using the right blockchain db or filesystem, since multiple blockchain could be in existence.
When a Bitcoin node, especially a new one, joins the network (often referred to as a "cluster" in this context, although the term isn't standard in Bitcoin), it undergoes a crucial process called **Initial Block Download (IBD)** to ensure it's using the correct and most secure blockchain.

Here's how this verification process unfolds:

### 1. Finding Peers

*   The new node first needs to find other nodes (peers) on the Bitcoin network to connect with.
*   It might use pre-configured **DNS seeds** or other discovery methods to find initial peers.

### 2. Requesting Headers

*   Once connected to peers, the new node requests **block headers**.
*   Headers are small (80 bytes) summaries of blocks, containing enough information to verify the chain of blocks.
*   The node starts by sending the header hash of the **genesis block** (the very first block in the Bitcoin blockchain) and requests subsequent headers [3].
*   It repeats this process until it has all the block headers up to the current **chaintip** (the latest block known to the network).

### 3. Determining the "Best" Chain

*   As the node receives block headers, it calculates the **cumulative Proof-of-Work (PoW)** for each potential chain it encounters.
*   PoW is a measure of the computational effort expended to mine the blocks on a chain.
*   The node will choose the chain with the most cumulative PoW as the **valid and correct blockchain**. This is often referred to as the **longest chain rule**, but it's more accurately about the chain with the most work [4].

### 4. Downloading and Validating Blocks

*   After identifying the longest valid header chain, the node downloads the **actual blocks** from its peers, starting from the genesis block.
*   Each block is then **fully validated** against the network's consensus rules, including checks for:
    *   **Transaction validity:** Ensuring that all transactions within the block are valid (sender has enough funds, digital signatures are correct, no double-spending).
    *   **Proof-of-Work:** Verifying that the PoW calculations meet the current difficulty requirements.
    *   **Block structure:** Ensuring the block's size and format comply with protocol rules.
    *   **Chain continuity:** Checking that the block correctly links to the previous block in the chain [5].

### 5. Staying in Sync

*   Once the IBD is complete, the node is fully synchronized and stays updated by continuously receiving and validating new transactions and blocks as they are broadcast across the network.
*   This ongoing validation ensures the node maintains a consistent and secure copy of the Bitcoin blockchain, actively participating in the network's decentralized verification process.

This multi-step process, combined with the built-in economic incentives for miners and the decentralized nature of the network, ensures that Bitcoin nodes, even when starting fresh, can establish a trustworthy and accurate view of the blockchain without relying on any central authority.

## 🚀 **Future Enhancements:**

### Advanced P2TR Features
- **Scriptless Scripts**: Implement advanced privacy features using Schnorr signature aggregation
- **Taproot Script Path Spending**: Add support for complex spending conditions
- **Enhanced Privacy**: Implement additional privacy features beyond basic Schnorr signatures
- **Multi-signature Support**: Add support for multi-signature transactions using Schnorr

### Performance Optimizations
- **Schnorr Batch Verification**: Implement batch verification for improved performance
- **Optimized Signature Aggregation**: Add support for signature aggregation to reduce transaction sizes
- **Parallel Transaction Processing**: Implement parallel processing for transaction validation
- **Memory Pool Optimization**: Optimize memory pool management for better performance

### Security Enhancements
- **Threshold Signatures**: Implement threshold signature schemes for enhanced security
- **Advanced Key Derivation**: Add support for hierarchical deterministic wallets (BIP-32/44)
- **Hardware Wallet Integration**: Add support for hardware wallet integration
- **Multi-factor Authentication**: Implement additional authentication layers

### Network and Protocol Improvements
- **Lightning Network Support**: Add support for Lightning Network payment channels
- **SegWit Implementation**: Implement Segregated Witness for improved transaction efficiency
- **Compact Block Support**: Add support for compact block relay
- **Peer Discovery Enhancement**: Improve peer discovery and connection management

### Developer Experience
- **API Documentation**: Comprehensive API documentation with examples
- **CLI Modernization**: ✅ Successfully migrated from `structopt` to `clap` for modern CLI parsing
- **Configuration Management**: Improve configuration management and validation
- **Logging Enhancement**: Enhanced logging with structured data and metrics

### Testing and Quality Assurance
- **✅ Consensus Tests**: Comprehensive consensus mechanism testing implemented
- **✅ Block Processing Order Tests**: Validating network convergence across nodes
- **✅ Cryptographic Tests**: Complete Schnorr signature and P2TR testing suite
- **✅ Multi-Node Consensus Tests**: Ensuring proper mining reward distribution
- **Integration Tests**: Add comprehensive integration tests for network scenarios
- **Performance Benchmarks**: Add performance benchmarking suite
- **Security Audits**: Regular security audits and vulnerability assessments
- **Code Coverage**: Improve test coverage and add mutation testing

### Monitoring and Observability
- **Metrics Collection**: Add comprehensive metrics collection
- **Health Checks**: Implement health check endpoints
- **Alerting System**: Add alerting for critical system events
- **Dashboard**: Create monitoring dashboard for system metrics

### Documentation and Education
- **Architecture Documentation**: Detailed architecture documentation
- **Deployment Guides**: Comprehensive deployment and operation guides
- **Tutorial Series**: Step-by-step tutorials for common use cases
- **Best Practices**: Document best practices for blockchain development
