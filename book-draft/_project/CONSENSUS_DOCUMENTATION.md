# Blockchain Consensus Mechanism Documentation

This document provides comprehensive documentation of the blockchain consensus mechanisms implemented in this codebase, focusing on the core algorithms that ensure network-wide agreement on block acceptance and chain state.

## Overview

The blockchain implements a robust consensus mechanism based on Bitcoin's Nakamoto Consensus, with three hierarchical levels of decision-making to ensure all nodes reach identical conclusions about which blocks to accept and which chain to follow. This consensus mechanism is the foundation that enables distributed nodes to maintain a single, authoritative blockchain state without requiring a central authority.

## Network Synchronization and Consensus

### What is Network Synchronization?

Network synchronization in blockchain refers to the process by which multiple nodes in a distributed network maintain consistent blockchain state. This is crucial for maintaining consensus and preventing forks or inconsistencies.

### How Network Synchronization Works

#### 1. **Block Propagation**
When a node mines a new block, it broadcasts this block to all connected peers. Each receiving node validates the block using the same consensus rules and adds it to their local blockchain if it passes validation.

#### 2. **Consensus Validation**
Each node validates incoming blocks using the same three-level consensus hierarchy:
- **Height Check**: Higher height blocks are always accepted (longest chain rule)
- **Work Comparison**: When heights are equal, compare cumulative proof-of-work
- **Hash Tie-Breaking**: When work is equal, use deterministic hash comparison

#### 3. **State Synchronization**
All nodes must maintain identical blockchain state, including:
- Same blockchain height
- Same block sequence
- Same UTXO (Unspent Transaction Output) sets
- Same transaction history

## Consensus Algorithm Architecture

### Three-Level Consensus Hierarchy

The consensus mechanism operates on three hierarchical levels, each providing a specific type of decision-making capability:

1. **Level 1: Block Height Comparison (Longest Chain Rule)**
2. **Level 2: Cumulative Work Comparison (Proof-of-Work Strength)**
3. **Level 3: Deterministic Tie-Breaking (Hash Comparison)**

This hierarchical approach ensures that:
- **Deterministic Decisions**: All nodes reach identical conclusions
- **Network Convergence**: Temporary forks are resolved quickly
- **Security**: Prevents double-spending and maintains blockchain integrity
- **Scalability**: Efficient decision-making even with many competing blocks

## Level 1: Longest Chain Rule

### Primary Consensus Mechanism

The Longest Chain Rule is the fundamental principle of blockchain consensus, serving as the primary mechanism for determining which blockchain is the "correct" one when multiple competing chains exist.

#### **Core Principle**
- **Rule**: Blocks with higher height are always accepted
- **Rationale**: Higher height represents more cumulative proof-of-work
- **Implementation**: Simple integer comparison between block heights
- **Security**: Prevents chain splits and ensures strongest chain wins

#### **Why the Longest Chain Rule Works**

1. **Proof-of-Work Accumulation**: Each block represents computational work expended to mine it. A longer chain means more total work has been invested in creating it.

2. **Security Through Work**: The more work invested in a chain, the more secure it becomes. An attacker would need to expend more work than the honest network to create a longer chain.

3. **Network Convergence**: All nodes naturally converge on the longest chain because it represents the most work and is therefore the most secure.

#### **Mathematical Foundation**

The longest chain rule is based on the principle that:
```
Security ∝ Cumulative Proof-of-Work ∝ Chain Length
```

This means:
- **Longer chains = More work = Higher security**
- **Shorter chains = Less work = Lower security**
- **Network chooses the most secure option**

#### **Implementation Details**

```rust
// CONSENSUS LEVEL 1: Block Height Comparison (Longest Chain Rule)
match new_block.get_height().cmp(&current_height) {
    Ordering::Greater => {
        // Check if block extends our chain or is on a different branch
        if new_block.get_pre_block_hash() == current_tip {
            // Normal case: block extends our current chain
            self.set_tip_hash(new_block.get_hash()).await?;
            self.update_utxo_set(new_block).await?;
        } else {
            // FORK: block is on a different branch at a higher height.
            // Must reorganize to properly rollback old branch's UTXO.
            // Without this, the old branch's coinbase subsidies remain,
            // creating money from nothing.
            self.reorganize_chain(new_block.get_hash()).await?;
        }
    }
    Ordering::Equal => {
        // SAME HEIGHT: Competing blocks require deeper analysis
        // → Level 2 (cumulative work) and Level 3 (tie-breaking)
    }
    Ordering::Less => {
        // LOWER HEIGHT: Block stored in DB but not accepted as tip.
        // It remains available for future reorganizations when higher
        // blocks on this branch arrive and trigger Equal/Greater cases.
        info!(
            "Block {} rejected: height {} < current height {} - shorter chain",
            new_block.get_hash(), new_block.get_height(), current_height
        );
    }
}
```

> **Note (March 2026):** The `Ordering::Greater` case was updated to handle forks where a higher-height block arrives on a different branch. The `Ordering::Less` case no longer deletes blocks — they are kept in the database for future ancestor lookups during reorganization. See "Bug 5" and "Bug 6" in the Consensus Bug Fixes section below for details.

#### **Network Behavior**

When a node receives a block with higher height:
1. **Immediate Acceptance**: The block is accepted without further validation
2. **Chain Update**: The node's blockchain tip is updated to the new block
3. **UTXO Update**: The UTXO set is updated to reflect the new chain state
4. **Propagation**: The node may propagate this block to other peers

#### **Security Properties**

- **Attack Resistance**: An attacker cannot easily create a longer chain because they would need to outpace the entire honest network
- **Fork Resolution**: Temporary forks are resolved quickly as nodes converge on the longest chain
- **Double-Spending Prevention**: Transactions in shorter chains become invalid, preventing double-spending

## Level 2: Cumulative Work Comparison

### When Heights Are Equal

When multiple blocks exist at the same height (creating a fork), the blockchain must determine which chain to follow. This is where cumulative work comparison becomes crucial.

#### **Trigger Conditions**
- **Competing blocks at identical height**: Multiple miners find blocks simultaneously
- **Network propagation delays**: Blocks arrive at different nodes in different orders
- **Temporary forks**: Natural occurrence in distributed systems

#### **Core Principle**
- **Method**: Compare cumulative proof-of-work of entire chains
- **Calculation**: Sum of work values from genesis to block tip
- **Decision**: Chain with higher cumulative work wins
- **Rationale**: More work = more security = stronger chain

#### **Work Calculation Process**

The cumulative work calculation is a critical component that determines chain strength:

1. **Start from specified block hash**: Begin at the block in question
2. **Traverse backwards through blockchain to genesis**: Follow the chain backwards
3. **Sum work value of each block encountered**: Accumulate work values
4. **Return total cumulative work**: Provide the total work invested

#### **Mathematical Foundation**

```
Cumulative Work = Σ(Work of each block from genesis to tip)
```

Where:
- **Work of a block** = 2^(256 - difficulty) / (2^256)
- **Difficulty** = Target difficulty for that block
- **Higher difficulty** = More work required = Higher work value

#### **Implementation Details**

```rust
// CONSENSUS LEVEL 2: Cumulative Work Comparison for Competing Blocks
// When blocks have equal height, we compare their cumulative proof-of-work
// to determine which chain represents more computational effort and is stronger

// Calculate work for the new block's chain
let new_work = self.get_chain_work(new_block.get_hash()).await?;

// Calculate work for the current chain
let current_work = self.get_chain_work(&current_tip).await?;

match new_work.cmp(&current_work) {
    Ordering::Greater => {
        // HIGHER WORK: Reorganize to the stronger chain
        // The new block's chain has more cumulative proof-of-work,
        // so we reorganize our blockchain to follow the stronger chain
        info!(
            "Reorganizing chain: new work {} > current work {} - stronger competing chain",
            new_work, current_work
        );
        self.reorganize_chain(new_block.get_hash()).await?;
    }
    Ordering::Equal => {
        // EQUAL WORK: Need tie-breaking mechanism
        // This triggers Level 3 consensus (deterministic tie-breaking)
        if self.accept_new_block_tie_break(new_block, &current_tip).await? {
            info!(
                "Reorganizing chain via tie-breaking: new work {} == current work {}",
                new_work, current_work
            );
            self.reorganize_chain(new_block.get_hash()).await?;
        }
    }
    Ordering::Less => {
        // LOWER WORK: Keep current chain
        // The new block's chain has less work, so we reject it
        info!("New chain has less work, keeping current chain");
    }
}
```

#### **Work Calculation Algorithm**

The `get_chain_work()` function implements the cumulative work calculation:

```rust
/// Calculate cumulative proof-of-work for a blockchain chain
///
/// This function traverses the blockchain backwards from the specified block
/// to the genesis block, summing the work value of each block encountered.
/// The work value represents the computational effort required to mine each block,
/// and chains with higher cumulative work are considered stronger and more secure.
pub async fn get_chain_work(&self, block_hash: &str) -> Result<u64> {
    let mut work = 0u64;
    let mut current_hash = block_hash.to_string();

    while let Some(block) = self.get_block(current_hash.as_bytes()).await? {
        // Add this block's work
        work += block.get_work();
        current_hash = block.get_pre_block_hash();

        // Stop at genesis block
        if current_hash == GENESIS_BLOCK_PRE_BLOCK_HASH || current_hash.is_empty() {
            break;
        }
    }
    Ok(work)
}
```

#### **Why Cumulative Work Matters**

1. **Security Measurement**: Total work invested in a chain represents its security level
2. **Attack Cost**: An attacker would need to outpace the cumulative work of the honest network
3. **Network Consensus**: All nodes can independently calculate and agree on work values
4. **Deterministic Results**: Same inputs always produce same work calculations

#### **Network Behavior**

When nodes receive competing blocks at the same height:

1. **Work Calculation**: Each node calculates cumulative work for both chains
2. **Comparison**: Nodes compare work values using the same algorithm
3. **Decision**: All nodes reach the same conclusion about which chain is stronger
4. **Reorganization**: Nodes switch to the chain with higher cumulative work
5. **Convergence**: Network converges on the single strongest chain

#### **Security Properties**

- **Attack Resistance**: An attacker cannot easily create a chain with more cumulative work
- **Network Convergence**: All nodes independently reach the same decision
- **Fork Resolution**: Temporary forks are resolved based on objective work measurement
- **Double-Spending Prevention**: Transactions in weaker chains become invalid

## Level 3: Deterministic Tie-Breaking

### When Work Is Equal

In rare cases, two competing chains may have identical cumulative proof-of-work. This can happen when:
- **Identical difficulty**: Both chains have the same difficulty progression
- **Same mining patterns**: Similar work distribution across blocks
- **Network conditions**: Equal work investment in both chains

#### **Core Principle**
- **Trigger**: Chains with identical cumulative proof-of-work
- **Method**: Lexicographic hash comparison
- **Properties**: Deterministic, unbiased, consistent across all nodes
- **Guarantee**: All nodes reach identical decisions

#### **Why Tie-Breaking is Necessary**

1. **Network Convergence**: Without tie-breaking, nodes might never agree on which chain to follow
2. **Fork Resolution**: Ensures temporary forks are resolved even when work is equal
3. **Deterministic Behavior**: All nodes must reach the same decision
4. **Network Stability**: Prevents indefinite chain splits

#### **Hash Comparison Mechanism**

The tie-breaking mechanism uses lexicographic (alphabetical) comparison of block hashes:

- **Algorithm**: Lexicographic string comparison of block hashes
- **Deterministic**: Same inputs always produce same output
- **Unbiased**: No node has inherent advantage
- **Consistent**: All nodes reach same decision

#### **Mathematical Foundation**

```
Hash Comparison: hash1 < hash2 (lexicographically)
```

Where:
- **hash1** = Hash of the first block
- **hash2** = Hash of the second block
- **<** = Lexicographic ordering (alphabetical comparison)

#### **Implementation Details**

```rust
/// Consensus tie-breaking mechanism for blocks with equal work
///
/// This function implements the consensus mechanism when two blocks have the same
/// proof-of-work (chain work). The consensus must be deterministic to ensure all
/// nodes in the network reach the same decision about which block to accept.
///
/// ## Consensus Requirements:
/// 1. **Deterministic**: Same inputs must always produce the same output
/// 2. **Unbiased**: No node should have an inherent advantage
/// 3. **Consistent**: All nodes must reach the same decision
async fn accept_new_block_tie_break(
    &self,
    new_block: &Block,
    current_tip: &str,
) -> Result<bool> {
    // Get the current tip block for comparison
    let current_block = self
        .get_block(current_tip.as_bytes())
        .await?
        .ok_or_else(|| {
            BtcError::GetBlockchainError("Current tip block not found".to_string())
        })?;

    info!("Consensus tie-breaking between competing blocks:");
    info!(
        "  New block: hash={}, timestamp={}, nonce={}",
        new_block.get_hash(),
        new_block.get_timestamp(),
        new_block.get_nonce()
    );
    info!(
        "  Current block: hash={}, timestamp={}, nonce={}",
        current_block.get_hash(),
        current_block.get_timestamp(),
        current_block.get_nonce()
    );

    // CONSENSUS MECHANISM: Deterministic Hash-Based Tie-Breaking
    // This mechanism ensures all nodes reach identical consensus decisions regardless of:
    // - Network propagation timing and order
    // - Block processing sequence variations
    // - Which node performs the comparison
    // - Local blockchain state differences
    // - Mining timing and network topology
    let new_hash = new_block.get_hash_string();
    let current_hash = current_block.get_hash_string();

    // Deterministic Lexicographic Hash Comparison
    // This creates a consistent ordering that all nodes can independently compute
    // and agree upon, ensuring network-wide consensus convergence
    if new_hash > current_hash {
        info!(
            "  Consensus decision: New block wins (hash: {} > {})",
            new_hash, current_hash
        );
        Ok(true)
    } else {
        info!(
            "  Consensus decision: Current block wins (hash: {} <= {})",
            new_hash, current_hash
        );
        Ok(false)
    }
}
```

#### **Why Lexicographic Comparison Works**

1. **Deterministic**: Same inputs always produce same output
2. **Unbiased**: No node has inherent advantage
3. **Consistent**: All nodes reach same decision
4. **Simple**: Easy to implement and verify
5. **Efficient**: Fast comparison operation

#### **Network Behavior**

When nodes receive competing blocks with equal work:

1. **Hash Extraction**: Each node extracts the block hash
2. **Lexicographic Comparison**: Nodes compare hashes using string ordering
3. **Decision**: All nodes reach the same conclusion about which hash is "smaller"
4. **Reorganization**: Nodes switch to the chain with the lexicographically smaller hash
5. **Convergence**: Network converges on the single chosen chain

#### **Security Properties**

- **Attack Resistance**: An attacker cannot easily create a block with a smaller hash
- **Network Convergence**: All nodes independently reach the same decision
- **Fork Resolution**: Temporary forks are resolved based on objective hash comparison
- **Double-Spending Prevention**: Transactions in the rejected chain become invalid

#### **Edge Cases and Handling**

1. **Identical Hashes**: Extremely rare with proper hash functions, but handled gracefully
2. **Hash Collisions**: Practically impossible with SHA-256, but theoretically possible
3. **Network Partitions**: Nodes in different partitions may temporarily choose different chains
4. **Recovery**: Network converges when partitions heal

#### **Deterministic Properties**

The tie-breaking mechanism ensures that:
- **Same Inputs → Same Output**: Identical block hashes always produce identical decisions
- **Node Independence**: Each node can independently reach the same conclusion
- **Network Consensus**: All nodes converge on the same chain
- **Temporal Consistency**: Decisions are consistent over time

## Chain Reorganization Process

### When Consensus Determines Chain Switch

Chain reorganization is the process by which a node switches from one blockchain to another when consensus rules determine that the new chain is stronger. This is a critical mechanism that ensures network convergence and maintains blockchain integrity.

#### **Trigger Conditions**

Chain reorganization occurs when:
1. **Higher Height**: A new block has higher height than the current tip
2. **More Work**: A new chain has more cumulative proof-of-work
3. **Tie-Breaking**: A new chain wins the deterministic tie-break
4. **Network Convergence**: The network converges on a single canonical chain

#### **Reorganization Process**

The chain reorganization process follows a carefully orchestrated sequence:

1. **Find Common Ancestor**: Locate shared block between chains
2. **Rollback Current Chain**: Remove blocks from current tip to ancestor
3. **Apply New Chain**: Add blocks from ancestor to new tip
4. **Update UTXO Set**: Synchronize transaction state with new chain
5. **Update Blockchain Tip**: Point to new canonical chain

#### **Detailed Implementation**

```rust
/// Perform blockchain reorganization to switch to a stronger chain
///
/// This function implements the chain reorganization mechanism that allows the blockchain
/// to switch from one branch to another when a stronger chain (with more cumulative work)
/// is discovered. This is an essential part of the consensus mechanism that ensures
/// all nodes converge on the same canonical chain.
///
/// ## Reorganization Process:
/// 1. Find the common ancestor between current chain and new chain
/// 2. Rollback the UTXO set from current tip to common ancestor
/// 3. Apply the new chain from common ancestor to new tip
/// 4. Update the blockchain tip to point to the new chain
///
/// ## Consensus Integration:
/// This function is called when the consensus mechanism determines that a competing
/// chain has higher cumulative work and should become the new canonical chain.
pub async fn reorganize_chain(&mut self, new_tip_hash: &str) -> Result<()> {
    let current_tip = self.get_tip_hash().await?;

    info!(
        "Starting chain reorganization from {} to {}",
        current_tip, new_tip_hash
    );

    // Find common ancestor
    let common_ancestor = self
        .find_common_ancestor(&current_tip, new_tip_hash)
        .await?;

    if let Some(ancestor) = common_ancestor {
        info!("Common ancestor found: {}", ancestor);

        // Rollback from current tip to common ancestor
        self.rollback_to_block(&ancestor).await?;

        // Apply new chain from common ancestor to new tip
        self.apply_chain_from_ancestor(&ancestor, new_tip_hash)
            .await?;

        info!("Chain reorganization completed");
    } else {
        return Err(BtcError::InvalidValueForMiner(
            "No common ancestor found".to_string(),
        ));
    }

    Ok(())
}
```

#### **Step 1: Finding Common Ancestor**

The common ancestor is the most recent block that exists in both the current chain and the new chain:

```rust
/// Find common ancestor of two blocks
///
/// This function finds the most recent common ancestor between two blockchain chains.
/// It properly handles chains with different structures, which is required for 4+ node scenarios
/// where competing blocks can create chains of different lengths.
async fn find_common_ancestor(&self, hash1: &str, hash2: &str) -> Result<Option<String>> {
    let mut chain1 = self.get_block_chain_hashes(hash1).await?;
    let mut chain2 = self.get_block_chain_hashes(hash2).await?;

    // Reverse to start from genesis (oldest first)
    chain1.reverse();
    chain2.reverse();

    // Find the last (most recent) common block between the two chains
    // This handles cases where chains have different structures due to competing blocks
    let mut last_common_ancestor: Option<String> = None;

    // Check each block in chain1 against all blocks in chain2
    for hash1_block in &chain1 {
        for hash2_block in &chain2 {
            if hash1_block == hash2_block {
                last_common_ancestor = Some(hash1_block.clone());
                break;
            }
        }
    }

    info!(
        "Finding common ancestor between chains of length {} and {}: {:?}",
        chain1.len(),
        chain2.len(),
        last_common_ancestor
    );

    Ok(last_common_ancestor)
}
```

#### **Step 2: Rolling Back Current Chain**

Rollback removes blocks from the current tip back to the common ancestor:

```rust
/// Rollback blockchain to a specific block during chain reorganization
///
/// This method maintains balance consistency during reorganization.
/// It performs a complete rollback by:
/// 1. Rolling back UTXO set for each block (removes coinbase transactions, restores spent inputs)
/// 2. Removing blocks from blockchain database
/// 3. Updating the blockchain tip
///
/// # Arguments
/// * `target_hash` - The hash of the block to rollback to (common ancestor)
///
/// # Returns
/// * `Result<()>` - Ok if rollback succeeded, Err if any step failed
///
/// ## Safety Measures:
/// - Never deletes the genesis block (prevents complete blockchain corruption)
/// - Resets is_empty flag if blockchain becomes empty after rollback
/// - Prevents infinite rollback loops with maximum attempt limits
/// - Validates block heights to prevent accidental genesis deletion
async fn rollback_to_block(&mut self, target_hash: &str) -> Result<()> {
    let mut current_tip = self.get_tip_hash().await?;
    let mut rollback_count = 0;
    const MAX_ROLLBACK_ATTEMPTS: usize = 1000; // Prevent infinite loops

    // Rollback from current tip to target block
    while current_tip != target_hash && rollback_count < MAX_ROLLBACK_ATTEMPTS {
        if let Some(block) = self.get_block(current_tip.as_bytes()).await? {
            // SAFETY CHECK: Never delete the genesis block
            // Genesis block is identified by having pre_block_hash == "None"
            if block.get_pre_block_hash() == GENESIS_BLOCK_PRE_BLOCK_HASH {
                info!(
                    "Rollback reached genesis block, stopping rollback to prevent blockchain corruption"
                );
                break;
            }

            // Additional safety check: Don't rollback past blocks with height 1 (genesis is height 1)
            if block.get_height() <= 1 {
                info!(
                    "Rollback reached block at height {}, stopping to prevent blockchain corruption",
                    block.get_height()
                );
                break;
            }

            // Rollback UTXO set for this block
            // This ensures that:
            // 1. Coinbase transactions are removed from UTXO set
            // 2. Spent inputs are restored as available UTXOs
            // 3. UTXO state stays synchronized with blockchain state
            self.rollback_utxo_set(&block).await?;

            // IMPORTANT: Do NOT delete the block from the database.
            // Rolled-back blocks must remain in the DB so that find_common_ancestor()
            // can still walk the chain when a future reorganization references them.
            // Without this, a later block on the rolled-back branch triggers
            // "No common ancestor found" because the intermediate blocks were deleted.
            // This matches Bitcoin Core behavior: non-canonical blocks stay in the DB.
            let _block_tree = self
                .blockchain
                .db
                .open_tree(self.get_blocks_tree_path())
                .map_err(|e| BtcError::OpenBlockchainTreeError(e.to_string()))?;

            // Block is kept in DB (not deleted) — see comment above
            // block_tree.remove(current_tip.as_bytes())?;  // REMOVED

            // Move to previous block in chain
            current_tip = block.get_pre_block_hash();
            rollback_count += 1;
        } else {
            // Block not found, stop rollback
            info!("Block not found during rollback, stopping rollback");
            break;
        }
    }

    if rollback_count >= MAX_ROLLBACK_ATTEMPTS {
        return Err(BtcError::BlockchainDBconnection(
            "Rollback exceeded maximum attempts, possible infinite loop detected".to_string(),
        ));
    }

    // Update blockchain tip to target block
    self.set_tip_hash(target_hash).await?;
    Ok(())
}
```

#### **Step 3: Applying New Chain**

Apply the new chain from the common ancestor to the new tip:

```rust
async fn apply_new_chain(&mut self, ancestor_hash: &str, new_tip_hash: &str) -> Result<()> {
    let mut current_hash = new_tip_hash.to_string();
    let mut blocks_to_apply = Vec::new();
    
    // Collect blocks from new tip back to ancestor
    while current_hash != ancestor_hash {
        if let Some(block) = self.get_block_by_hash(&current_hash).await? {
            blocks_to_apply.push(block);
            current_hash = block.get_previous_hash();
        } else {
            return Err(BtcError::BlockNotFound("Block not found during chain application"));
        }
    }
    
    // Apply blocks in reverse order (from ancestor to tip)
    for block in blocks_to_apply.into_iter().rev() {
        self.add_block(&block).await?;
    }
    
    Ok(())
}
```

#### **Safety Measures**

The reorganization process includes multiple safety mechanisms:

1. **Genesis Block Protection**: Never delete the genesis block
2. **Maximum Rollback Limits**: Prevent infinite loops
3. **Height Validation**: Ensure valid block heights
4. **Atomic Operations**: Maintain consistent state
5. **Error Recovery**: Graceful handling of failures

```rust
// Safety checks before reorganization
if target_hash == GENESIS_HASH {
    return Err(BtcError::InvalidOperation("Cannot rollback to genesis block"));
}

if rollback_count > MAX_ROLLBACK_ATTEMPTS {
    return Err(BtcError::MaxRollbackExceeded("Too many rollback attempts"));
}

if new_height < current_height {
    return Err(BtcError::InvalidHeight("New chain height is lower than current"));
}
```

#### **UTXO Set Synchronization**

After chain reorganization, the UTXO set must be updated to reflect the new chain state:

```rust
async fn update_utxo_set_for_reorganization(&mut self) -> Result<()> {
    // Remove UTXOs from rolled-back blocks
    self.remove_utxos_from_rolled_back_blocks().await?;
    
    // Add UTXOs from new blocks
    self.add_utxos_from_new_blocks().await?;
    
    // Reindex UTXO set to ensure consistency
    self.utxo_set.reindex().await?;
    
    Ok(())
}
```

#### **Network Behavior During Reorganization**

1. **Block Validation**: Each block in the new chain is validated
2. **Transaction Processing**: Transactions are processed in the correct order
3. **UTXO Updates**: UTXO set is updated atomically
4. **State Consistency**: All nodes maintain consistent state
5. **Network Convergence**: All nodes converge on the same chain

#### **Performance Considerations**

- **Efficient Rollback**: Only necessary blocks are removed
- **Batch Operations**: Multiple operations are batched for efficiency
- **Caching**: Block data is cached to reduce database access
- **Parallel Processing**: Independent operations are parallelized

#### **Error Handling**

The reorganization process includes comprehensive error handling:

- **Database Errors**: Rollback on database failures
- **Block Not Found**: Graceful handling of missing blocks
- **Invalid Blocks**: Rejection of invalid blocks
- **Network Partitions**: Recovery from network issues
- **State Corruption**: Detection and recovery from state issues

## UTXO Set Management

### State Synchronization

The UTXO (Unspent Transaction Output) set is the core data structure that maintains the current state of all unspent coins in the blockchain. It is essential for consensus state management and must be kept synchronized across all nodes.

#### **Purpose and Importance**

- **Purpose**: Maintain transaction state consistency across nodes
- **Process**: Remove spent inputs, add new outputs
- **Integration**: Essential for consensus state management
- **Consistency**: Ensures all nodes have identical UTXO sets

#### **UTXO Set Structure**

The UTXO set tracks all unspent transaction outputs:

```rust
pub struct UTXOSet {
    pub utxos: HashMap<String, Vec<UTXO>>,
    pub blockchain_service: BlockchainService,
}

pub struct UTXO {
    pub tx_id: String,
    pub output_index: usize,
    pub amount: u64,
    pub address: String,
}
```

#### **UTXO Set Operations**

The UTXO set supports several key operations:

1. **Add UTXO**: Add a new unspent transaction output
2. **Remove UTXO**: Remove a spent transaction output
3. **Get Balance**: Calculate total balance for an address
4. **Reindex**: Rebuild the entire UTXO set from blockchain

#### **Processing Logic**

The UTXO set processing follows specific rules for different transaction types:

##### **Coinbase Transactions**
- **Only add outputs**: No inputs to process
- **Mining rewards**: Create new UTXOs for miners
- **No spending**: Cannot spend from coinbase transactions

```rust
// Process coinbase transaction
for output in coinbase_tx.get_outputs() {
    let utxo = UTXO {
        tx_id: coinbase_tx.get_id(),
        output_index: output.index,
        amount: output.amount,
        address: output.address,
    };
    utxo_set.add_utxo(&output.address, utxo);
}
```

##### **Regular Transactions**
- **Remove spent inputs**: Remove UTXOs that are being spent
- **Add new outputs**: Create new UTXOs for recipients
- **Balance validation**: Ensure sufficient funds are available

```rust
// Process regular transaction
// Remove spent inputs
for input in transaction.get_inputs() {
    utxo_set.remove_utxo(&input.address, &input.tx_id, input.output_index);
}

// Add new outputs
for output in transaction.get_outputs() {
    let utxo = UTXO {
        tx_id: transaction.get_id(),
        output_index: output.index,
        amount: output.amount,
        address: output.address,
    };
    utxo_set.add_utxo(&output.address, utxo);
}
```

#### **UTXO Set Synchronization**

After chain reorganization, the UTXO set must be synchronized with the new chain state:

```rust
/// Rollback UTXO set for a specific block during chain reorganization
///
/// This method is used to maintain UTXO consistency during chain reorganization.
/// It reverses the effects of a block on the UTXO set by:
/// 1. Removing all outputs created by transactions in the block (including coinbase)
/// 2. Restoring all inputs that were spent by non-coinbase transactions
/// 3. Processing transactions in reverse order to maintain consistency
///
/// # Arguments
/// * `block` - The block whose effects should be rolled back from the UTXO set
///
/// # Returns
/// * `Result<()>` - Ok if rollback succeeded, Err if any step failed
///
/// # Notes
/// - Coinbase transactions are handled correctly: outputs removed, no inputs to restore
/// - Regular transactions: outputs removed, spent inputs restored as UTXOs
/// - Must be called BEFORE removing blocks from blockchain database
pub async fn rollback_utxo_set(&self, block: &Block) -> Result<()> {
    // Open the UTXO database tree for modification
    let db = self.blockchain.db.clone();
    let utxo_tree = db
        .open_tree("chainstate")
        .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;

    // Process transactions in reverse order to maintain consistency
    let mut transactions = block.get_transactions().clone();
    transactions.reverse();

    for transaction in transactions {
        // For coinbase transactions: only remove outputs (no inputs to restore)
        if transaction.is_coinbase() {
            // Remove coinbase outputs from UTXO set
            for (output_index, output) in transaction.get_outputs().iter().enumerate() {
                let utxo_key = format!("{}:{}", transaction.get_id(), output_index);
                utxo_tree.remove(utxo_key.as_bytes())
                    .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
            }
        } else {
            // For regular transactions: remove outputs and restore inputs
            // Remove transaction outputs from UTXO set
            for (output_index, _output) in transaction.get_outputs().iter().enumerate() {
                let utxo_key = format!("{}:{}", transaction.get_id(), output_index);
                utxo_tree.remove(utxo_key.as_bytes())
                    .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
            }

            // Restore spent inputs as available UTXOs
            for input in transaction.get_inputs() {
                let utxo_key = format!("{}:{}", input.tx_id, input.output_index);
                let utxo_data = serde_json::to_vec(&UTXO {
                    tx_id: input.tx_id.clone(),
                    output_index: input.output_index,
                    amount: input.amount,
                    address: input.address.clone(),
                }).map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
                
                utxo_tree.insert(utxo_key.as_bytes(), utxo_data)
                    .map_err(|e| BtcError::UTXODBconnection(e.to_string()))?;
            }
        }
    }

    Ok(())
}
```

#### **UTXO Rollback and Update**

During chain reorganization, UTXOs must be rolled back and updated:

##### **Rollback Process**
1. **Remove UTXOs from rolled-back blocks**: Undo changes from removed blocks
2. **Restore previous state**: Return UTXO set to state before rolled-back blocks
3. **Validate consistency**: Ensure UTXO set matches blockchain state

```rust
async fn rollback_utxo_set(&mut self, target_height: usize) -> Result<()> {
    let current_height = self.get_best_height().await?;
    
    // Rollback UTXOs from blocks above target height
    for height in (target_height + 1)..=current_height {
        if let Some(block) = self.get_block_by_height(height).await? {
            // Remove UTXOs created by this block
            self.remove_utxos_from_block(&block).await?;
            
            // Restore UTXOs that were spent by this block
            self.restore_spent_utxos(&block).await?;
        }
    }
    
    // Reindex to ensure consistency
    self.reindex().await?;
    
    Ok(())
}
```

##### **Update Process**
1. **Add UTXOs from new blocks**: Process new blocks in the chain
2. **Update state**: Apply changes from new blocks
3. **Validate consistency**: Ensure UTXO set matches new chain state

```rust
async fn update_utxo_set_for_new_chain(&mut self, new_blocks: &[Block]) -> Result<()> {
    // Process each new block
    for block in new_blocks {
        // Add UTXOs from this block
        self.add_utxos_from_block(block).await?;
        
        // Remove UTXOs spent by this block
        self.remove_spent_utxos_from_block(block).await?;
    }
    
    // Reindex to ensure consistency
    self.reindex().await?;
    
    Ok(())
}
```

#### **UTXO Set Consistency**

The UTXO set must maintain consistency with the blockchain:

1. **Balance Validation**: Total UTXO value must equal total coin supply
2. **Transaction Validation**: All transactions must have sufficient funds
3. **State Synchronization**: All nodes must have identical UTXO sets
4. **Integrity Checks**: Regular validation of UTXO set integrity

```rust
pub async fn validate_utxo_consistency(&self) -> Result<bool> {
    // Check that total UTXO value equals total coin supply
    let total_utxo_value = self.get_total_utxo_value().await?;
    let total_coin_supply = self.get_total_coin_supply().await?;
    
    if total_utxo_value != total_coin_supply {
        return Ok(false);
    }
    
    // Check that all UTXOs are valid
    for (address, utxos) in &self.utxos {
        for utxo in utxos {
            if !self.is_valid_utxo(utxo).await? {
                return Ok(false);
            }
        }
    }
    
    Ok(true)
}
```

#### **Performance Optimization**

The UTXO set includes several performance optimizations:

1. **Caching**: Frequently accessed UTXOs are cached
2. **Batch Operations**: Multiple operations are batched together
3. **Incremental Updates**: Only changed UTXOs are updated
4. **Parallel Processing**: Independent operations are parallelized

#### **Error Handling**

The UTXO set includes comprehensive error handling:

- **Invalid UTXOs**: Detection and removal of invalid UTXOs
- **Balance Mismatches**: Detection and correction of balance issues
- **State Corruption**: Recovery from corrupted UTXO state
- **Network Partitions**: Handling of network-related issues

#### **Network Synchronization**

During network synchronization, UTXO sets must be kept consistent:

1. **Block Propagation**: UTXO changes are propagated with blocks
2. **State Validation**: Each node validates UTXO changes
3. **Consensus Application**: Consensus rules are applied to UTXO changes
4. **Network Convergence**: All nodes converge on identical UTXO sets

## Network Convergence Guarantees

### Deterministic Decision Making
- All nodes apply identical consensus rules
- All nodes reach same decisions about block acceptance
- Network converges on single canonical chain
- No bias toward any particular node or timing

### Consensus Convergence Process
1. **Rule Application**: All nodes apply same three-level hierarchy
2. **Decision Consistency**: Deterministic tie-breaking ensures agreement
3. **State Synchronization**: UTXO sets remain consistent across nodes
4. **Chain Unification**: Network converges on single authoritative chain

## Error Handling and Safety

### Consensus Failures
- Invalid blocks rejected immediately
- Database errors propagated appropriately
- Consensus failures logged without system crashes
- Graceful degradation under error conditions

### Database Transaction Safety
- Blocks added to database before consensus decisions
- Rejected blocks removed from database
- UTXO set updated only for accepted blocks
- Chain reorganization performed atomically

## Testing and Validation

### Consensus Testing Results (Updated March 2026)
- **Unit Tests**: 164/164 passing (all existing tests unaffected by consensus fixes)
- **Chain Reorganization Tests**: 3/3 passing
- **Tie-Breaking Mechanism Tests**: 3/3 passing with deterministic hash comparison
- **Work Calculation Tests**: Validating chain work computation
- **2-Node Scenario 2 (transactions to non-seed node)**: 15/15 passing — was ~50% before fixes
- **7-Node Final Scenario (6 miners, linear topology)**: 8/10 passing — was ~20% before fixes; remaining failures are the minority fork limitation with instant PoW

### Network Stability
- Deterministic tie-breaking mechanism ensures node agreement when blocks propagate
- Block relay ensures blocks reach all nodes in multi-hop topologies
- UTXO rollback correctly handles fully-spent transactions during reorganization
- Coin conservation (total supply) maintained in all test runs (100% of runs show correct total)
- Mining concurrency guard prevents duplicate blocks from same node
- Stale transaction detection prevents mining with already-spent inputs

## Implementation Details

### Core Functions
- `add_block()`: Main consensus entry point (3-level hierarchy with fork detection)
- `mine_block()`: Block creation with PoW (direct tip/UTXO update for local blocks)
- `get_chain_work()`: Cumulative work calculation
- `reorganize_chain()`: Chain switching via common ancestor → rollback → apply
- `find_common_ancestor()`: Walks both chain branches to find shared block
- `rollback_to_block()`: UTXO rollback without block deletion
- `apply_chain_from_ancestor()`: Applies new chain's blocks in chronological order
- `accept_new_block_tie_break()`: Deterministic lexicographic hash comparison
- `update_utxo_set()`: Forward UTXO processing (spend inputs, create outputs)
- `rollback_block()`: Reverse UTXO processing (restore inputs, remove outputs)
- `prepare_mining_utxo()`: Transaction validation against UTXO before mining
- `cancel_current_mining()`: Signal mining cancellation when competing block arrives
- `broadcast_new_block()`: Send `Inv` to known peers after mining

### Database Integration
- Sled embedded database with atomic transactions for state consistency
- Block insertion in Sled transaction, consensus logic after commit
- Rolled-back blocks retained in DB (not deleted) for future ancestor lookups
- UTXO set stored in `chainstate` sled tree
- Proper error handling and rollback mechanisms

## Security Properties

### Consensus Security
- Protection against double-spending attacks
- Resistance to network partitioning
- Prevention of chain splits and forks
- Guaranteed network convergence

### Cryptographic Security
- SHA256 hashing for block integrity
- Deterministic hash-based tie-breaking
- Secure proof-of-work validation
- Consistent cryptographic operations

## Performance Characteristics

### Computational Efficiency
- O(log n) work calculation with caching
- Efficient block storage and retrieval
- Optimized UTXO set updates
- Minimal consensus overhead

### Network Efficiency
- Deterministic decisions reduce network traffic
- Single-pass consensus evaluation
- Efficient chain reorganization
- Minimal state synchronization overhead

This consensus mechanism ensures that the blockchain maintains security, consistency, and network-wide agreement while providing robust protection against various attack vectors and network conditions.

---

## Consensus Bug Fixes and Protocol Enhancements (March 2026)

This section documents critical consensus bugs discovered during multi-node testing and the fixes applied. The bugs caused inconsistent balances between nodes, duplicate mining subsidies, and permanent forks. Testing covered 2-node (15/15 passed) and 7-node (8/10 passed) scenarios with 6 concurrent miners.

For the full bug analysis with step-by-step failure walkthroughs and test procedures, see `consensus_bug/CONSENSUS_BUG_ANALYSIS.md`.

### Bug 1: UTXO Rollback Fails for Fully-Spent Transactions

**File:** `src/chain/utxo_set.rs`, method `rollback_block()`

**Problem:** During chain reorganization, `rollback_block()` restores spent transaction outputs by looking up the original transaction's entry in the UTXO tree. When ALL outputs of a transaction had been spent, the entry was completely removed from the UTXO tree by `update()`. The rollback code only handled the case where some outputs still existed (the `if let Some` branch). When the entry was missing, `outs_to_restore` stayed as an empty `vec![]`, and an empty vector was stored — the output was permanently lost.

This directly caused incorrect balances after chain reorganization. Coins that should have been restored to the sender's wallet were silently dropped.

**Fix:** The `else` branch now creates a fresh vector and inserts the restored output at the correct `vout` position. This matches Bitcoin Core's behavior where `DisconnectTip` uses "undo data" (stored in `rev*.dat` files) to reverse UTXO changes. Our implementation achieves the same result by looking up the original transaction via `find_transaction` and restoring the specific output that was spent.

**Additional fix:** Transactions within `rollback_block()` are now processed in **reverse order** (newest first). This correctly handles intra-block dependencies where a later transaction spends an output created by an earlier transaction in the same block. Without reverse order, the earlier transaction's outputs would be removed before the later transaction's inputs could be restored, producing incorrect UTXO state. This matches Bitcoin Core's `DisconnectBlock` which processes transactions in reverse order.

### Bug 2: Mining with Stale (Already-Spent) Transaction Inputs

**File:** `src/node/miner.rs`, function `prepare_mining_utxo()`

**Problem:** When multiple miners competed to mine the same transactions, `prepare_mining_utxo()` would collect transactions from the memory pool without verifying their inputs were still unspent in the UTXO set. If a competing block had already been accepted (spending those inputs), the local miner would create a block with already-spent inputs. The `update_utxo_set()` method didn't validate input existence — it silently skipped missing inputs while still adding the coinbase transaction, creating money from nothing.

**Fix:** `prepare_mining_utxo()` now accepts a `BlockchainService` parameter and validates each transaction's inputs against the current UTXO set (`chainstate` sled tree). Transactions with already-spent inputs are filtered out and removed from the memory pool. If no valid transactions remain, mining is aborted with an error. This is analogous to Bitcoin Core's `TestBlockValidity` which validates the block template before mining begins.

**Additional protection:** A second validation pass runs inside `chainstate.rs::mine_block()` under the **write lock**, just before calling the inner `mine_block`. This catches the race condition where a competing block is accepted between `prepare_mining_utxo` (read lock) and the actual mining (write lock). This double-check ensures no stale transactions slip through regardless of timing.

### Bug 3: Mining Concurrency and Cancellation

**File:** `src/node/miner.rs`, function `process_mine_block()`

**Problem:** Multiple mining tasks could run concurrently when transactions arrived from different peers simultaneously. Additionally, when a competing block arrived from the network, there was no mechanism to signal the local miner to stop.

**Fix:** Two atomic flags (`MINING_IN_PROGRESS` and `MINING_CANCELLED`) prevent concurrent mining and allow cancellation:

- `MINING_IN_PROGRESS` uses `compare_exchange` to ensure only one mining task runs at a time. Subsequent attempts return immediately.
- `MINING_CANCELLED` is set by the `Package::Block` handler in `net_processing.rs` when a competing block arrives. It is checked before mining starts.
- **Critically, cancellation is NOT checked after `mine_block()` completes.** If the block was already created and added to the local chain, it MUST be broadcast. Skipping the broadcast would leave an unbroadcast block in the local chain, causing a permanent fork where this node has a block that no other node knows about.

This approach is simpler than Bitcoin Core's signal-based mining interruption but achieves the same effect for this implementation's single-threaded-per-batch mining model.

### Bug 4: Block Relay for Multi-Hop Networks

**File:** `src/net/net_processing.rs`, `Package::Block` handler

**Problem:** When a node received a block from the network and accepted it, it did NOT relay the block to its other peers. The `broadcast_new_block()` function (called only by the miner after mining) sends `Inv` to known peers, but receiving nodes were silent. In a linear topology (Node 1→2→3→4→5→6→7), a block mined by Node 4 only reached Nodes 3 and 5. Nodes 1, 2, 6, and 7 never saw it, causing different nodes to see different subsets of competing blocks and reaching different consensus decisions.

**Fix:** The `Package::Block` handler now relays new blocks to all peers (excluding the sender and self). A `block_is_new` check prevents infinite relay loops — if the block was already in the database (from a previous relay or the Sled transaction in `add_block`), it is not relayed again.

This matches Bitcoin Core's block relay behavior where nodes forward new blocks (via `inv` or direct `headers` messages per BIP 130) to all connected peers. In Bitcoin Core, the `sendheaders` protocol (BIP 130) allows even more efficient relay by sending block headers directly instead of requiring the `inv → getheaders → getdata` round trip.

### Bug 5: `Ordering::Greater` Fork Handling

**File:** `src/store/file_system_db_chain.rs`, `add_block()`, `Ordering::Greater` branch

**Problem:** When a block at a higher height arrived, the code directly set the tip and updated the UTXO set, assuming the block extended the current chain. With block relay, a higher-height block on a DIFFERENT branch could arrive (e.g., a height-7 block whose parent diverged from our chain at height 5). The old branch's UTXO (including coinbase subsidies) was never rolled back, creating money from nothing.

**Fix:** The `Ordering::Greater` case now checks `new_block.get_pre_block_hash() == current_tip`. If the block extends our chain (parent matches our tip), it's applied directly. If the parent doesn't match (different branch), `reorganize_chain()` is called to properly rollback the old branch and apply the new one. This ensures UTXO consistency regardless of block arrival order.

In Bitcoin Core, this scenario is handled by `ActivateBestChain()` which always considers the full chain with the most cumulative work, running `DisconnectTip()` and `ConnectTip()` as needed to switch branches.

### Bug 6: Rolled-Back Blocks Must Stay in Database

**File:** `src/store/file_system_db_chain.rs`, `rollback_to_block()`

**Problem:** During chain reorganization, `rollback_to_block()` deleted rolled-back blocks from the sled database. This made future reorganizations impossible — if a block on the deleted branch arrived later (via relay), `find_common_ancestor()` could not walk the chain history and failed with "No common ancestor found."

**Fix:** Blocks are no longer deleted from the database during rollback. Only the UTXO set is rolled back and the tip pointer is updated. Rolled-back blocks remain in storage as non-canonical blocks (they exist in the DB but are not on the tip chain). This matches Bitcoin Core's behavior where all received blocks are kept in `blocks/blk*.dat` files regardless of whether they are on the active chain. The `rev*.dat` undo files and the chainstate database track which blocks are canonical.

### Block Propagation and Synchronization Architecture

The current implementation uses the following P2P message flow for block propagation:

```
Miner creates block
    ↓
broadcast_new_block() sends Inv to known peers
    ↓
Peers receive Inv → send GetData → receive Block
    ↓
Peers call add_block() for consensus
    ↓
If block is NEW: relay Inv to other peers (block relay)
    ↓
Peers on different branches receive blocks at various heights
    ↓
Ordering::Greater (extends chain) → accept directly
Ordering::Greater (different branch) → reorganize_chain()
Ordering::Equal → tie-break → possibly reorganize_chain()
Ordering::Less → store in DB, wait for higher blocks on this branch
```

**Comparison with Bitcoin Core's protocol:**

| Feature | This Implementation | Bitcoin Core |
|---------|-------------------|--------------|
| Block announcement | `Inv` message | `Inv` or direct `headers` (BIP 130) |
| Block request | `GetData` | `GetData` (with compact blocks, BIP 152) |
| Fork detection | Pairwise comparison on arrival | `getheaders` with fork-point locator |
| Chain evaluation | Per-block height/work comparison | `ActivateBestChain` with full chain work |
| UTXO rollback | `rollback_block()` with `find_transaction` | `DisconnectBlock()` with undo data (`rev*.dat`) |
| Block storage | Sled embedded DB | LevelDB (chainstate) + flat files (blocks) |
| Non-canonical blocks | Kept in DB (no deletion) | Kept in `blk*.dat` files |

### Known Limitation: Minority Fork with Instant PoW

**Status:** Affects ~20% of 7-node test runs. Does not affect 2-node scenarios.

**Description:** With `TARGET_BITS=8` (very easy proof-of-work), mining completes in milliseconds — faster than block propagation across the network. When 6 miners compete simultaneously, a node can mine and advance its chain to height N+1 before receiving the winning block at height N from the rest of the network. The winning block arrives at `Ordering::Less` (lower height) and is stored but not accepted as the tip. The node is now on a minority fork.

The competing chain's blocks at height N, N+1, etc. eventually arrive via relay. When a block at the same height (Equal) arrives, the tie-breaking mechanism resolves the fork correctly. However, if the node mined MULTIPLE blocks before any competing blocks arrived, the competing blocks may all be at lower heights and get rejected, leaving the node permanently on its minority fork.

**Why this doesn't affect Bitcoin in production:** Bitcoin's proof-of-work takes ~10 minutes per block, while block propagation takes ~1-2 seconds. The PoW is orders of magnitude slower than propagation, so a node always receives competing blocks before it finishes mining its own. The scenario where a node mines multiple blocks before receiving any competing blocks is effectively impossible in Bitcoin's real network.

**Proper fix (not yet implemented):** Implement Bitcoin's headers-first synchronization protocol (BIP 130). This would allow nodes to:
1. Exchange lightweight block headers (80 bytes) before full blocks
2. Discover the full competing chain structure from headers alone
3. Evaluate whether a competing chain has more cumulative work without downloading all blocks
4. Request missing blocks only after confirming the competing chain is stronger

This is a significant protocol enhancement beyond the scope of the current consensus bug fixes.

### Testing Results

| Test | Scenario | Runs | Pass Rate | Notes |
|------|----------|------|-----------|-------|
| Scenario 2 (2-node) | Transactions sent to Node 2 | 15/15 | 100% | Was ~50% before fixes |
| Final (7-node) | 6 miners, transactions via Node 4 | 8/10 | 80% | Was ~20% before fixes; remaining 20% is the minority fork limitation |
| Coin conservation | Total BTC across all wallets | 10/10 | 100% | Always exactly 70 BTC (was 80+ before stale mining fix) |

### Files Modified

| File | Changes |
|------|---------|
| `src/chain/utxo_set.rs` | Fixed `rollback_block()` for fully-spent UTXOs; reversed transaction order |
| `src/chain/chainstate.rs` | Added stale mining protection (re-validate under write lock) |
| `src/node/miner.rs` | Added UTXO validation in `prepare_mining_utxo()`; mining concurrency guard; cancellation mechanism |
| `src/node/context.rs` | Updated `prepare_mining_utxo` call signature |
| `src/net/net_processing.rs` | Added block relay; mining cancellation on block receipt; log level fixes |
| `src/store/file_system_db_chain.rs` | Fork handling in `Ordering::Greater`; blocks retained in DB during rollback |

### References

- [Bitcoin P2P Network Protocol](https://developer.bitcoin.org/devguide/p2p_network.html) — Block propagation, headers-first sync, fork detection
- [BIP 130: sendheaders](https://github.com/bitcoin/bips/blob/master/bip-0130.mediawiki) — Direct header announcement for efficient block relay
- [Bitcoin Core net_processing.cpp](https://github.com/bitcoin/bitcoin/blob/master/src/net_processing.cpp) — Reference implementation of block processing and relay
- [Bitcoin Core DisconnectTip](https://github.com/bitcoin/bitcoin/pull/9208) — Chain reorganization with undo data and mempool management
- [Bitcoin Chain Reorganization](https://learnmeabitcoin.com/technical/blockchain/chain-reorganization/) — Visual explanation of reorg process
- [Nakamoto Consensus](https://blockonomi.com/nakamoto-consensus/) — Foundational consensus mechanism
