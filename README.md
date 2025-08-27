# blockchain

---

## Running Multiple Nodes on the Same Server

Each node requires its own set of environment variables. Below are example configurations for five nodes. You will need five separate terminal windows, each with its own environment settings.

```sh
# Node 1
export CENTERAL_NODE=127.0.0.1:2001
export BLOCKS_TREE=blocks1
export TREE_DIR=data1
export NODE_ADDR=127.0.0.1:2001
export WALLET_FILE=wallet1.dat
export RUST_LOG=trace

# Node 2
export CENTERAL_NODE=127.0.0.1:2002
export BLOCKS_TREE=blocks2
export TREE_DIR=data2
export NODE_ADDR=127.0.0.1:2002
export WALLET_FILE=wallet2.dat
export RUST_LOG=trace

# Node 3
export CENTERAL_NODE=127.0.0.1:2003
export BLOCKS_TREE=blocks3
export TREE_DIR=data3
export NODE_ADDR=127.0.0.1:2003
export WALLET_FILE=wallet3.dat
export RUST_LOG=trace

# Node 4
export CENTERAL_NODE=127.0.0.1:2004
export BLOCKS_TREE=blocks4
export TREE_DIR=data4
export NODE_ADDR=127.0.0.1:2004
export WALLET_FILE=wallet4.dat
export RUST_LOG=trace

# Node 5
export CENTERAL_NODE=127.0.0.1:2005
export BLOCKS_TREE=blocks5
export TREE_DIR=data5
export NODE_ADDR=127.0.0.1:2005
export WALLET_FILE=wallet5.dat
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
cargo run startnode WALLET_ADDR yes local
```

Second Node:
```sh
cargo run startnode WALLET_ADDR yes 127.0.0.1:2001
```
Third Node:
```sh
cargo run startnode WALLET_ADDR yes 127.0.0.1:2002
```
- Since Node 1 is part of the blockchain, it can be used as the Seed node for the third node.
Repeat these steps in separate terminals for each node you want to run.

---

# TODO's: 

## 1. Add capability to have multiple database backends, currently only the filesystem is supported.

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
