<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../01-Introduction.md)
2. **Chapter 2: Introduction to Bitcoin & Blockchain** ← *You are here*
3. [Chapter 2.1: Cryptography](crypto/README.md) - Cryptographic primitives and libraries
4. [Chapter 2.2: Transaction System](02-Transaction-System.md) - Transaction ID format guide
5. [Chapter 2.3: Blockchain State Management](chain/README.md) - Chain state and UTXO management
6. [Chapter 2.4: Network Layer](net/README.md) - Peer-to-peer networking and protocol
7. [Chapter 2.5: Node Orchestration](node/README.md) - Node context and coordination
8. [Chapter 2.6: Primitives](primitives/README.md) - Core data structures
9. [Chapter 2.7: Storage Layer](store/README.md) - Persistent storage implementation
10. [Chapter 2.8: Utilities](util/README.md) - Utility functions and helpers
11. [Chapter 2.9: Wallet System](wallet/README.md) - Wallet implementation and key management
12. [Chapter 3: Web API Architecture](web/README.md) - REST API implementation
13. [Chapter 4: Desktop Admin Interface](../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)
14. [Chapter 5: Wallet User Interface](../bitcoin-wallet-ui/04-Wallet-UI.md)
15. [Chapter 6: Embedded Database & Persistence](../bitcoin-wallet-ui/05-Embedded-Database.md)
16. [Chapter 7: Web Admin Interface](../bitcoin-web-ui/06-Web-Admin-UI.md)

### Part II: Deployment & Operations

17. [Chapter 8: Docker Compose Deployment](../ci/docker-compose/01-Introduction.md)
18. [Chapter 9: Kubernetes Deployment](../ci/kubernetes/README.md)

</details>

</div>

<div align="right">

**[← Back to Main Book](../README.md)** | **[Next: Cryptography →](crypto/README.md)** | **[Blockchain State Management →](chain/README.md)** | **[Network Layer →](net/README.md)** | **[Node Orchestration →](node/README.md)** | **[Primitives →](primitives/README.md)** | **[Storage Layer →](store/README.md)** | **[Utilities →](util/README.md)** | **[Wallet System →](wallet/README.md)**

</div>

---

<div align="center">

# 📚 Building a Full-Stack Bitcoin Blockchain With Rust

## Chapter 2: Introduction to Bitcoin & Blockchain

**Part I: Core Blockchain Implementation**

**[← Back to Main Book](../README.md)** | **Introduction to Bitcoin & Blockchain** | **[Next: Cryptography →](crypto/README.md)** 📚

</div>

---

## What Is Bitcoin?

Bitcoin is a decentralized digital currency and payment system that operates without a central authority or intermediary. It was introduced in 2008 through a whitepaper published by an individual or group using the pseudonym Satoshi Nakamoto. Bitcoin enables peer-to-peer transactions over the internet, allowing users to send and receive value directly without relying on traditional financial institutions.

At its core, Bitcoin is three things:

1. **A Digital Currency**: Bitcoin exists purely in digital form, represented as entries in a distributed ledger. Unlike traditional currencies, there is no physical coin or bill—only cryptographic records.

2. **A Payment Network**: Bitcoin operates as a global payment network that processes transactions 24/7 without borders, bank holidays, or intermediaries. Transactions are verified by network participants and recorded on a public ledger.

3. **A Store of Value**: Many people view Bitcoin as "digital gold"—a scarce asset that can serve as a store of value and hedge against inflation, independent of any government or central bank.

## What Is Blockchain?

Blockchain is the underlying technology that powers Bitcoin and many other cryptocurrencies. At its simplest, a blockchain is a distributed, immutable ledger that records transactions in a secure and verifiable way.

### Core Components

**Blocks**: A blockchain consists of a series of blocks, each containing a collection of transactions. Each block includes:
- A list of transactions
- A reference to the previous block (creating a chain)
- A cryptographic hash that uniquely identifies the block
- A timestamp
- Other metadata depending on the blockchain implementation

**Chain Structure**: Blocks are linked together sequentially, with each block containing the hash of the previous block. This creates an unbreakable chain—if any block is modified, its hash changes, breaking the chain and making tampering immediately detectable.

**Distributed Network**: Unlike traditional databases stored in a single location, blockchain data is replicated across many nodes (computers) in a peer-to-peer network. Each node maintains a complete copy of the blockchain, ensuring redundancy and resilience.

**Consensus Mechanism**: Network participants must agree on which transactions are valid and in what order they occurred. Bitcoin uses a consensus mechanism called Proof of Work (PoW), where miners compete to solve cryptographic puzzles to validate transactions and create new blocks.

### Key Characteristics

- **Decentralization**: No single entity controls the network. Power is distributed among all participants.
- **Immutability**: Once recorded, transactions cannot be easily altered or deleted.
- **Transparency**: All transactions are publicly visible (though identities are pseudonymous).
- **Security**: Cryptographic techniques ensure that only authorized parties can spend their funds.
- **Trustlessness**: Participants don't need to trust each other or a central authority—the protocol enforces rules.

## The Origin of Bitcoin and Blockchain

### The Pre-Bitcoin Era

The concept of digital cash and cryptographic currencies predates Bitcoin by decades. In the 1980s and 1990s, researchers explored various approaches:

- **David Chaum's DigiCash (1989)**: An early attempt at anonymous digital cash using cryptographic protocols, but it required a central authority.
- **B-Money (1998)**: Wei Dai proposed a distributed electronic cash system, introducing concepts like proof-of-work and decentralized consensus.
- **Bit Gold (1998)**: Nick Szabo's proposal for a decentralized digital currency, though it was never fully implemented.

These early attempts faced fundamental challenges: the double-spending problem (how to prevent someone from spending the same digital coin twice) and the Byzantine Generals' Problem (how to achieve consensus in a distributed system with potentially malicious actors).

### Satoshi Nakamoto's Breakthrough

On October 31, 2008, a paper titled "Bitcoin: A Peer-to-Peer Electronic Cash System" was published to a cryptography mailing list. The author, Satoshi Nakamoto (whose true identity remains unknown), solved the double-spending problem without requiring a trusted third party.

**Key Innovations**:

1. **Proof of Work**: A mechanism where participants (miners) expend computational resources to validate transactions and create blocks. This makes attacking the network economically unfeasible.

2. **Blockchain Structure**: Linking blocks cryptographically creates an immutable history. Any attempt to modify past transactions requires redoing all subsequent work, making tampering computationally impractical.

3. **Incentive Structure**: Miners are rewarded with newly created bitcoins and transaction fees, aligning their economic interests with network security.

4. **Peer-to-Peer Network**: A distributed network where all nodes participate in validation and consensus, eliminating single points of failure.

### The Genesis Block

On January 3, 2009, the first Bitcoin block (the "Genesis Block") was mined by Satoshi Nakamoto. Embedded in this block was a message referencing a headline from The Times newspaper: "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks." This message is often interpreted as a commentary on the financial system that Bitcoin was designed to replace or improve upon.

### Evolution and Growth

Since 2009, Bitcoin has grown from an experimental system used by cryptography enthusiasts to a global financial asset with:
- A market capitalization in the hundreds of billions of dollars
- Millions of users worldwide
- Acceptance by major corporations and institutions
- Recognition as legal tender in some countries
- A thriving ecosystem of exchanges, wallets, and applications

The blockchain technology underlying Bitcoin has also inspired thousands of other cryptocurrencies and blockchain projects, each exploring different use cases, consensus mechanisms, and technical approaches.

## Advantages of Decentralized Systems

Decentralization—the distribution of power and control away from a central authority—is one of blockchain's most revolutionary aspects. This fundamental shift enables new possibilities across many domains.

### Financial Systems

**Traditional Banking Limitations**:
- Single points of failure (bank outages affect all customers)
- Geographic restrictions and limited access
- High transaction fees, especially for cross-border transfers
- Slow settlement times (often days for international transfers)
- Exclusion of unbanked populations
- Censorship and account freezes

**Blockchain Advantages**:
- **24/7 Availability**: No bank holidays or maintenance windows
- **Global Access**: Anyone with internet access can participate
- **Lower Fees**: Direct peer-to-peer transfers reduce intermediary costs
- **Faster Settlement**: Transactions can be confirmed in minutes rather than days
- **Financial Inclusion**: Enables participation without traditional banking infrastructure
- **Censorship Resistance**: No single entity can freeze accounts or prevent transactions
- **Programmable Money**: Smart contracts enable automated, conditional payments

### Voting Systems

**Current Voting Challenges**:
- Voter fraud and ballot tampering
- Low voter turnout due to inconvenience
- Lack of transparency in vote counting
- Difficulty verifying individual votes were counted correctly
- Expensive and time-consuming processes

**Blockchain Voting Benefits**:
- **Transparency**: All votes are recorded on a public ledger, verifiable by anyone
- **Immutability**: Votes cannot be altered once recorded
- **Verifiability**: Voters can verify their vote was counted without revealing their choice
- **Accessibility**: Remote voting becomes more secure and feasible
- **Auditability**: Complete audit trail of all voting activity
- **Reduced Costs**: Automation reduces administrative overhead

**Real-World Examples**:
- Estonia's e-Residency program uses blockchain for secure digital identity
- Several U.S. states have piloted blockchain voting systems
- Various organizations use blockchain for transparent governance voting

### Governance and DAOs

Decentralized Autonomous Organizations (DAOs) represent a new form of organizational structure enabled by blockchain technology.

**Traditional Governance Issues**:
- Centralized decision-making power
- Lack of transparency in organizational decisions
- Slow bureaucratic processes
- Limited stakeholder participation
- Difficulty in tracking and auditing decisions

**DAO Advantages**:
- **Democratic Participation**: Token holders vote on proposals directly
- **Transparent Treasury**: All financial transactions are publicly visible
- **Automated Execution**: Smart contracts execute decisions automatically when conditions are met
- **Global Participation**: Members can participate from anywhere
- **Reduced Corruption**: Transparent, auditable decision-making processes
- **Rapid Iteration**: Faster decision-making compared to traditional organizations

**Use Cases**:
- Investment funds where members vote on investments
- Open-source project governance
- Community-owned platforms and services
- Decentralized venture capital

### Social Media and Content Platforms

**Current Platform Problems**:
- Centralized control over content moderation
- Data ownership by platforms, not users
- Algorithmic manipulation and echo chambers
- Censorship of controversial but legal content
- Limited monetization options for creators
- Privacy concerns and data harvesting

**Decentralized Social Media Benefits**:
- **User Ownership**: Users control their data and content
- **Censorship Resistance**: No single entity can remove content arbitrarily
- **Direct Monetization**: Creators can receive payments directly from fans
- **Transparent Algorithms**: Open-source algorithms reduce manipulation
- **Data Portability**: Users can move their data between platforms
- **Community Governance**: Users participate in platform decisions

**Emerging Platforms**:
- Decentralized social networks built on blockchain
- NFT-based content ownership
- Cryptocurrency tipping and payments for creators
- Community-moderated platforms with transparent rules

### Supply Chain Management

**Traditional Supply Chain Challenges**:
- Lack of transparency in product origins
- Difficulty tracking products through complex supply chains
- Counterfeit products
- Inefficient recall processes
- Limited traceability of raw materials

**Blockchain Supply Chain Benefits**:
- **End-to-End Traceability**: Track products from origin to consumer
- **Provenance Verification**: Verify authenticity and origin of products
- **Automated Compliance**: Smart contracts enforce regulatory requirements
- **Reduced Fraud**: Immutable records prevent tampering
- **Efficient Recalls**: Quickly identify affected products
- **Consumer Trust**: Transparent information builds confidence

**Real-World Applications**:
- Food safety tracking (e.g., Walmart's food traceability system)
- Diamond and luxury goods authentication
- Pharmaceutical supply chain verification
- Fair trade and ethical sourcing verification

### Digital Identity

**Current Identity System Problems**:
- Fragmented identity across multiple services
- Privacy concerns with centralized identity providers
- Identity theft and fraud
- Limited control over personal data
- Difficulty proving identity without physical documents

**Blockchain Identity Benefits**:
- **Self-Sovereign Identity**: Users control their own identity data
- **Selective Disclosure**: Share only necessary information
- **Cross-Platform Portability**: Use identity across multiple services
- **Reduced Fraud**: Cryptographic verification prevents identity theft
- **Privacy-Preserving**: Zero-knowledge proofs enable verification without revealing data
- **Global Access**: Digital identity accessible from anywhere

## Technical Foundations

Understanding blockchain requires familiarity with several key technical concepts. While we'll explore these in detail throughout this book, here's an introduction to the fundamental building blocks.

### Cryptographic Hash Functions

Hash functions are mathematical algorithms that take input data of any size and produce a fixed-size output (hash). Key properties:

- **Deterministic**: Same input always produces same output
- **Fast to Compute**: Hash calculation is computationally efficient
- **One-Way**: Cannot reverse-engineer input from hash
- **Avalanche Effect**: Small input changes produce completely different hashes
- **Collision Resistant**: Extremely difficult to find two inputs with same hash

Bitcoin uses SHA-256 (Secure Hash Algorithm 256-bit), which produces 256-bit (32-byte) hashes. These hashes are used for:
- Creating unique transaction IDs
- Linking blocks together
- Proof of Work mining
- Creating addresses

### Digital Signatures

Digital signatures provide cryptographic proof that a transaction was authorized by the owner of the funds. The process involves:

1. **Key Pair Generation**: Each user generates a public/private key pair
2. **Signing**: Private key is used to sign transactions
3. **Verification**: Public key is used to verify signatures

Bitcoin uses the Elliptic Curve Digital Signature Algorithm (ECDSA) with the secp256k1 curve. This ensures:
- Only the private key holder can authorize transactions
- Anyone can verify transaction authenticity
- Signatures cannot be forged
- Signatures are tied to specific transaction data

### Public Key Cryptography

Bitcoin uses asymmetric cryptography, where:
- **Private Key**: Secret key that must be kept secure. Used to sign transactions.
- **Public Key**: Derived from private key, can be shared publicly. Used to verify signatures.
- **Address**: Derived from public key, used as a destination for payments.

The mathematical relationship ensures that:
- Public keys can be derived from private keys, but not vice versa
- Signatures created with private keys can be verified with corresponding public keys
- Losing a private key means losing access to funds permanently

### Merkle Trees

Merkle trees are data structures that enable efficient verification of large datasets. In Bitcoin:

- Each transaction is hashed
- Transaction hashes are paired and hashed together
- This process continues until a single root hash (Merkle root) is created
- The Merkle root is included in the block header

Benefits:
- **Efficient Verification**: Verify a transaction's inclusion without downloading entire block
- **Tamper Detection**: Any transaction change alters the Merkle root
- **Scalability**: Proof size grows logarithmically with number of transactions

### Proof of Work Consensus

Proof of Work (PoW) is Bitcoin's consensus mechanism:

1. Miners collect pending transactions
2. They create a candidate block
3. They repeatedly modify a nonce value and hash the block header
4. They search for a hash below a target difficulty threshold
5. First miner to find valid hash broadcasts block to network
6. Other nodes verify and accept valid blocks

**Security Properties**:
- Requires significant computational work to create blocks
- Makes attacking the network economically unfeasible
- Aligns miner incentives with network security
- Difficulty adjusts automatically to maintain consistent block times

**Energy Considerations**: PoW requires substantial energy consumption, which has led to exploration of alternative consensus mechanisms like Proof of Stake (PoS) in other blockchains.

### UTXO Model

Bitcoin uses the Unspent Transaction Output (UTXO) model:

- **Transaction Outputs**: When you receive Bitcoin, you receive UTXOs
- **Transaction Inputs**: When you spend Bitcoin, you reference UTXOs as inputs
- **Balance Calculation**: Your balance is the sum of all UTXOs you can spend
- **No Account Balances**: Unlike traditional banking, there are no account balances—only UTXOs

**Advantages**:
- Parallel transaction processing
- Better privacy (addresses can be reused or changed)
- Simpler verification logic
- Natural support for complex transaction types

### Network Protocol

Bitcoin nodes communicate using a peer-to-peer protocol:

- **Discovery**: Nodes discover peers through DNS seeds or manual connections
- **Handshaking**: Nodes exchange version information
- **Block Propagation**: New blocks are broadcast to all peers
- **Transaction Propagation**: Transactions are broadcast and relayed
- **Synchronization**: New nodes download blockchain history from peers

The protocol is designed to be:
- **Resilient**: No single point of failure
- **Scalable**: Handles network growth
- **Efficient**: Minimizes bandwidth usage
- **Secure**: Validates all received data

## Real-World Use Cases and Advantages

Blockchain technology has moved beyond cryptocurrency to enable innovative solutions across industries. Here are prominent real-world applications demonstrating blockchain's practical value.

### Cross-Border Payments

**Problem**: Traditional international wire transfers are slow (3-5 days), expensive ($25-50 per transaction), and require multiple intermediaries.

**Blockchain Solution**: Cryptocurrencies enable near-instant transfers with minimal fees, operating 24/7 without intermediaries.

**Examples**:
- **Ripple (XRP)**: Used by financial institutions for cross-border payments, reducing settlement time from days to seconds
- **Stellar**: Focuses on financial inclusion, enabling low-cost remittances
- **Central Bank Digital Currencies (CBDCs)**: Countries exploring blockchain-based national currencies

**Advantages**:
- Settlement in minutes instead of days
- Lower fees (often under $1 regardless of amount)
- 24/7 availability
- Increased transparency and traceability

### Remittances

**Problem**: Migrant workers sending money home face high fees (often 5-10% of transaction value) and slow processing.

**Blockchain Solution**: Direct peer-to-peer transfers eliminate intermediaries, reducing costs and increasing speed.

**Impact**:
- Billions saved annually in fees
- Faster access to funds for recipients
- Financial inclusion for unbanked populations
- Reduced dependency on traditional remittance services

### Smart Contracts

Smart contracts are self-executing programs stored on blockchain that automatically execute when predetermined conditions are met.

**Use Cases**:
- **Insurance**: Automated claims processing when conditions are verified
- **Escrow Services**: Automatic fund release when goods are delivered
- **Derivatives**: Automated settlement of financial contracts
- **Supply Chain**: Automatic payments when delivery milestones are met

**Platforms**:
- **Ethereum**: First platform to enable general-purpose smart contracts
- **Cardano**: Focuses on formal verification and security
- **Solana**: High-performance smart contract platform

### Non-Fungible Tokens (NFTs)

NFTs represent unique digital assets on blockchain, enabling:
- **Digital Art**: Artists can sell and prove ownership of digital works
- **Collectibles**: Digital trading cards, virtual items in games
- **Identity**: Certificates, diplomas, licenses
- **Real Estate**: Property ownership records and fractional ownership

**Advantages**:
- Proven ownership and authenticity
- Royalty mechanisms for creators
- Interoperability across platforms
- Reduced fraud and counterfeiting

### Decentralized Finance (DeFi)

DeFi recreates traditional financial services using blockchain and smart contracts:

**Services**:
- **Lending/Borrowing**: Peer-to-peer loans without banks
- **Trading**: Decentralized exchanges (DEXs) for cryptocurrency trading
- **Stablecoins**: Cryptocurrencies pegged to fiat currencies
- **Yield Farming**: Earning returns on cryptocurrency holdings
- **Insurance**: Decentralized insurance protocols

**Advantages**:
- No intermediaries or gatekeepers
- Global access regardless of location
- Transparent and auditable protocols
- Composable services (building new services from existing ones)
- 24/7 operation

**Challenges**:
- Smart contract vulnerabilities
- Regulatory uncertainty
- User experience complexity
- Scalability limitations

### Tokenization of Assets

Blockchain enables representing real-world assets as digital tokens:

**Asset Types**:
- **Real Estate**: Fractional ownership of properties
- **Commodities**: Gold, oil, agricultural products
- **Securities**: Stocks, bonds as tokens
- **Art**: High-value artwork ownership shares
- **Intellectual Property**: Royalties and licensing

**Benefits**:
- Increased liquidity for illiquid assets
- Fractional ownership enabling smaller investments
- 24/7 trading markets
- Reduced transaction costs
- Automated compliance and reporting

### Healthcare

**Applications**:
- **Medical Records**: Secure, interoperable patient records
- **Drug Traceability**: Tracking pharmaceuticals through supply chain
- **Clinical Trials**: Transparent and auditable trial data
- **Insurance Claims**: Automated processing and fraud prevention
- **Research**: Secure sharing of anonymized medical data

**Advantages**:
- Patient data ownership and control
- Interoperability between healthcare providers
- Reduced fraud and errors
- Improved research capabilities through data sharing

### Energy Trading

**Applications**:
- **Peer-to-Peer Energy Trading**: Direct trading between energy producers and consumers
- **Renewable Energy Credits**: Tracking and trading carbon credits
- **Grid Management**: Decentralized energy grid coordination
- **Microgrids**: Local energy networks with blockchain coordination

**Benefits**:
- Reduced energy costs through direct trading
- Increased renewable energy adoption
- More efficient grid utilization
- Transparent energy source tracking

### Intellectual Property and Royalties

**Applications**:
- **Music Royalties**: Automatic distribution to artists and rights holders
- **Patent Management**: Transparent patent ownership and licensing
- **Content Licensing**: Automated licensing and payment for digital content
- **Trademark Protection**: Immutable trademark records

**Advantages**:
- Fair compensation for creators
- Reduced disputes through transparent records
- Automated royalty distribution
- Global protection and enforcement

## Challenges and Limitations

While blockchain offers significant advantages, it's important to understand current limitations:

### Scalability

**Challenge**: Bitcoin processes approximately 7 transactions per second, compared to thousands for traditional payment systems.

**Solutions Being Explored**:
- Layer 2 solutions (Lightning Network for Bitcoin)
- Alternative consensus mechanisms (Proof of Stake)
- Sharding (dividing blockchain into smaller pieces)
- Optimized block structures

### Energy Consumption

**Challenge**: Proof of Work requires substantial energy, raising environmental concerns.

**Solutions**:
- Transition to Proof of Stake (Ethereum's approach)
- Renewable energy for mining operations
- More efficient consensus algorithms
- Layer 2 solutions reducing on-chain transactions

### Regulatory Uncertainty

**Challenge**: Evolving regulations create uncertainty for businesses and users.

**Status**:
- Different countries have varying approaches
- Some countries embrace blockchain, others restrict it
- Regulatory clarity improving gradually
- Compliance requirements evolving

### User Experience

**Challenge**: Blockchain applications can be complex for non-technical users.

**Improvements**:
- Better wallet interfaces
- Simplified transaction processes
- Educational resources
- Integration with familiar applications

### Interoperability

**Challenge**: Different blockchains operate independently, limiting cross-chain functionality.

**Solutions**:
- Cross-chain bridges
- Interoperability protocols
- Multi-chain platforms
- Standardized protocols

## The Future of Blockchain

Blockchain technology continues to evolve rapidly. Emerging trends include:

- **Improved Scalability**: Solutions enabling thousands of transactions per second
- **Better Privacy**: Zero-knowledge proofs enabling private transactions
- **Interoperability**: Seamless interaction between different blockchains
- **Sustainability**: More energy-efficient consensus mechanisms
- **Regulatory Clarity**: Clearer frameworks enabling broader adoption
- **Enterprise Adoption**: More businesses integrating blockchain solutions
- **Web3**: Decentralized internet built on blockchain foundations

## Conclusion

Bitcoin and blockchain represent a fundamental shift in how we think about trust, value transfer, and digital systems. By eliminating the need for trusted intermediaries and enabling direct peer-to-peer interactions, blockchain technology opens new possibilities across finance, governance, social media, supply chains, and beyond.

As we progress through this book, we'll build a complete Bitcoin blockchain implementation, understanding not just what these systems do, but how they work at the deepest technical levels. We'll explore the cryptographic primitives, data structures, network protocols, and consensus mechanisms that make blockchain possible.

Whether you're interested in building blockchain applications, understanding the technology's potential, or simply learning how these systems work, the knowledge you'll gain will provide a solid foundation for working with blockchain technology in any context.

The journey from understanding these concepts to implementing them is challenging but rewarding. By the end of this book, you'll have built a working blockchain system and gained insights that will serve you whether you're developing new blockchain applications, evaluating blockchain solutions, or contributing to the growing ecosystem of decentralized technologies.

---

## Related Implementation Chapters

Now that you understand the foundational concepts of Bitcoin and blockchain, explore how we implement these concepts:

### Core Implementation Modules

- **[Cryptography](crypto/README.md)**: See how cryptographic primitives (hash functions, digital signatures, key pairs) are implemented
- **[Transaction System](02-Transaction-System.md)**: Understand how transactions are structured, validated, and stored
- **[Blockchain State Management](chain/README.md)**: Learn how blockchain state, UTXO set, and chain operations are managed
- **[Network Layer](net/README.md)**: Explore peer-to-peer networking, message processing, and network synchronization
- **[Node Orchestration](node/README.md)**: Understand how node context coordinates blockchain, mempool, mining, and network operations
- **[Primitives](primitives/README.md)**: Study the core data structures (Block, Transaction, Blockchain) that form the foundation
- **[Storage Layer](store/README.md)**: Learn about persistent storage using file system databases and block persistence
- **[Utilities](util/README.md)**: Discover utility functions and helpers used throughout the system
- **[Wallet System](wallet/README.md)**: Explore wallet creation, key management, address generation, and transaction signing

### Interface Layers

- **[Web API Architecture](web/README.md)**: Learn how we build a REST API interface to interact with the blockchain, implementing the concepts covered in this chapter

---

---

<div align="center">

**📚 [← Back to Main Book](../README.md)** | **Chapter 2: Introduction to Bitcoin & Blockchain** | **[Next: Cryptography →](crypto/README.md)** 📚

</div>

---

*This chapter has provided comprehensive foundational knowledge about Bitcoin and blockchain technology, covering their origins, core concepts, advantages of decentralized systems across various domains, and real-world use cases. We've explored the fundamental principles that make blockchain technology revolutionary: decentralization, immutability, transparency, security, and trustlessness. Understanding these concepts is essential before diving into implementation details, as they inform every design decision we make in building our blockchain system. In the next chapter, we'll examine [Cryptography](crypto/README.md) to understand the cryptographic primitives that secure our blockchain, or you can explore the [implementation chapters](#related-implementation-chapters) to see how these concepts are implemented in our Rust-based blockchain system.*
