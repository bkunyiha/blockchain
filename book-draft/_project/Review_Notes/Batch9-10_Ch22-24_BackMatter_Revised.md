# Editorial Revisions: Chapters 22–24 & Back Matter
## Only Changed Sections with Original and Revised Text

**Status**: Ready for copy/paste replacement
**Date**: 2026-03-21

---

## Chapter 22: Docker Compose

### File: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/ci/docker-compose/04-Deployment-Scenarios-and-Operations.md`

#### Revision 22-7: Production Deployment Section Enhancement

**Location**: After line 327 (Production Deployment section)

**Original Text** (lines 305–327):
```markdown
### Production Deployment

For production environments:

1. **Tag the image:**
   ```bash
   docker compose build
   docker tag <image-id> blockchain:latest
   docker tag <image-id> blockchain:v<version>
   ```

2. **Push to registry (if using one):**
   ```bash
   docker push blockchain:latest
   ```

3. **Deploy with zero downtime:**
   ```bash
   # Use rolling updates or blue-green deployment strategy
   docker compose up -d --scale webserver=2 --no-recreate
   # Then scale down old instances
   ```
```

**Revised Text**:
```markdown
### Production Deployment

For production environments:

> **⚠️ Warning: Registry Security** — Before pushing images to a registry, ensure:
> - The registry is private (not public)
> - Images are signed or validated with cryptographic checksums
> - Only authorized systems can pull images
> - Network policies restrict image pull operations
> A compromised image in your registry can compromise your entire blockchain network.

1. **Tag the image:**
   ```bash
   docker compose build
   docker tag <image-id> blockchain:latest
   docker tag <image-id> blockchain:v<version>
   ```

2. **Push to registry (if using one):**
   ```bash
   docker push blockchain:latest
   ```

3. **Deploy with zero downtime:**
   ```bash
   # Use rolling updates or blue-green deployment strategy
   docker compose up -d --scale webserver=2 --no-recreate
   # Then scale down old instances
   ```
```

**Rationale**: Security callout prevents naive production deployments where images are pushed to public registries. Critical for blockchain systems where code integrity matters.

---

### File: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/ci/docker-compose/03-Deployment-Topology.md`

#### Revision 22-6: Script Name Consistency

**Location**: Line 314 and throughout the file

**Original Instances**:
- Line 314: `./docker compose.scale.sh`
- Line 317: `./docker compose.scale.sh`
- Line 456: `./docker-compose.scale.sh`
- Line 541: `./docker compose.scale.sh`
- Line 629: `./docker-compose.scale.sh`

**Revised**: Change all instances to use hyphens consistently:
- `./docker-compose.scale.sh` (standard shell script naming convention)

**Specific Location Changes**:
- Line 314: Change `./docker compose.scale.sh` → `./docker-compose.scale.sh`
- Line 317: Change `./docker compose.scale.sh` → `./docker-compose.scale.sh`
- Line 541: Change `./docker compose.scale.sh` → `./docker-compose.scale.sh`

(Lines 456 and 629 are already correct.)

**Rationale**: Consistency; standard Unix convention uses hyphens for executable names.

---

### File: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/ci/docker-compose/01-Introduction.md`

#### Revision 22-2: Redis Context Forward Reference

**Location**: After line 172 (Rate Limiting Configuration section)

**Original Text**:
```markdown
**Rate limiting configuration:**
- The webserver loads `axum_rate_limiter` settings from `ci/docker-compose/configs/Settings.toml`.
```

**Revised Text**:
```markdown
**Rate limiting configuration:**
- The webserver loads `axum_rate_limiter` settings from `ci/docker-compose/configs/Settings.toml`.
- Redis is required as the shared state backend for rate limiting across multiple instances (explained in detail in Section 3).
```

**Rationale**: Introduces Redis early with a forward reference. Helps readers understand why Redis is mentioned in Quick Start without requiring full explanation upfront.

---

## Chapter 23: Kubernetes

### File: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/ci/kubernetes/README.md`

#### Revision 23-2: Metrics-Server Explanation Enhancement

**Location**: After line 256

**Original Text**:
```markdown
- **`metrics-server`**: enables `kubectl top ...` and provides CPU/memory metrics.
```

**Revised Text**:
```markdown
- **`metrics-server`**: enables `kubectl top ...` and provides the Metrics API that powers CPU/memory-based autoscaling (see Section 6: Autoscaling). Without this, HPA (Horizontal Pod Autoscaler) cannot measure resource usage and will not scale pods automatically.
```

**Rationale**: Connects metrics-server to autoscaling dependency, explaining why it matters functionally (not just "enables a command").

---

#### Revision 23-1: Deep Dive Box Guidance

**Location**: After line 77 (end of Prerequisites section, before "What You'll Build")

**Original Text**: (no intro note to deep-dive boxes)

**Revised Text** (add new paragraph):
```markdown
**Throughout this chapter, optional "Deep dive" boxes provide extra context for learning.** These sections are not required reading—if you're in a hurry, skip them and return later. They explain the "why" behind the "what," but the practical procedures work either way.

---

## Section 1: Introduction & Quick Start
```

**Rationale**: Sets expectations about optional content, reducing cognitive overload for readers in a hurry.

---

#### Revision 23-4: Port-Forward Alternative Ports

**Location**: After line 473 (end of "Accessing Webserver" section)

**Original Text**:
```markdown
### Accessing the Webserver (Port-Forward)

```bash
kubectl port-forward -n blockchain svc/webserver-service 8080:8080
```

Then open http://localhost:8080 in your browser.
```

**Revised Text**:
```markdown
### Accessing the Webserver (Port-Forward)

```bash
kubectl port-forward -n blockchain svc/webserver-service 8080:8080
```

Then open http://localhost:8080 in your browser.

**If port 8080 is already in use**, use a different local port:

```bash
# Map local 9090 to container 8080
kubectl port-forward -n blockchain svc/webserver-service 9090:8080
# Then open http://localhost:9090
```
```

**Rationale**: Addresses common issue (port conflict) without requiring troubleshooting search.

---

## Chapter 24: Rust Language Guide

### File: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/rust/README.md`

#### Revision 24-1: Section Numbering Clarity

**Location**: Lines 123–130 (Table of Contents intro)

**Original Text**:
```markdown
This guide is structured to build understanding progressively:

- we start by getting a working local toolchain (so every reader can run the code)
- then we build the memory model (ownership/borrowing) and the data model (structs/enums)
- then we move into abstraction boundaries (traits, generics) and failure paths (error handling)
- finally, we cover the pieces that matter for production systems (lifetimes, smart pointers, async, concurrency, modules, testing)
```

**Revised Text**:
```markdown
This guide is structured into seven sections that build understanding progressively:

1. **Foundations** (files 00–04): Get a working local toolchain, build the memory model (ownership/borrowing), and the data model (structs/enums)
2. **Error Handling & Type System** (files 05–07): Move into abstraction boundaries (traits, generics) and failure paths (error handling)
3. **Advanced Memory Management** (files 08–09): Smart pointers and pattern matching
4. **Code Organization** (files 10, 13): Derive macros and modules
5. **Concurrency & Async** (files 11–12): Async/await, threads, and locks
6. **Functional Programming** (files 14–15): Iterators, closures, and type conversions
7. **Production Patterns** (files 16–17): Testing and best practices
```

**Rationale**: Maps section titles to actual file numbers, reducing confusion. File numbers now match the physical structure.

---

### File: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/rust/02-Ownership-and-Borrowing.md`

#### Revision 24-5: Code Block Source Attribution

**Location**: Line 72 (start of first code example)

**Original Text**:
```rust
impl TXInput {
    pub fn new(txid: &[u8], vout: usize) -> TXInput {
```

**Revised Text**:
```rust
// From bitcoin/src/primitives/transaction.rs
impl TXInput {
    pub fn new(txid: &[u8], vout: usize) -> TXInput {
```

**Additional instances to update**:
- Line 93: Add `// From bitcoin/src/primitives/transaction.rs` before `pub fn uses_key`
- All subsequent code examples in this file and throughout Ch 24

**Rationale**: Grounds readers in the actual codebase. Helps them find the full implementation if they want to study it further. Professional textbook standard.

---

## Back Matter

### File: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/Glossary.md`

#### Revision BM-2: Add Deployment-Related Terms

**Location**: After line 112 (end of "Rust Language Terms" section), add new subsection before "Project-Specific Terms"

**Original Text**:
```markdown
---

## Project-Specific Terms
```

**Revised Text**:
```markdown
---

## Deployment & Infrastructure Terms

**Container** — A lightweight, isolated environment that packages application code, dependencies, and configuration. Containers are created from images and managed by Docker or Kubernetes. *(Ch 22, 23)*

**Container Registry** — A centralized storage for Docker images (e.g., Docker Hub, AWS ECR). Registries allow teams to push and pull images for deployment. *(Ch 22, 23)*

**Headless Service** — A Kubernetes Service without a ClusterIP, used for StatefulSets where pods need stable DNS names without load balancing. Each pod is directly addressable. *(Ch 23)*

**Volume** — A Docker storage mechanism for persisting data beyond container lifetime. Volumes are managed by Docker and mounted into containers at a path. *(Ch 22)*

**PersistentVolumeClaim (PVC)** — A Kubernetes request for storage. PVCs abstract underlying storage systems, allowing pods to request storage without knowing implementation details. *(Ch 23)*

---

## Project-Specific Terms
```

**Rationale**: Glossary now covers key deployment terms referenced in Chapters 22–23. Improves accessibility for readers unfamiliar with containerization.

---

### File: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/Bibliography.md`

#### Revision BM-3: Add URLs to Bibliography References

**Location**: Lines 60–64 (Deployment & Infrastructure section)

**Original Text**:
```markdown
## Deployment & Infrastructure

- Docker Compose Specification (v3). https://docs.docker.com/compose/compose-file/

- Kubernetes Official Documentation. https://kubernetes.io/docs/

- Wiggins, A. (2017). *The Twelve-Factor App*. https://12factor.net/
```

**Revised Text**:
```markdown
## Deployment & Infrastructure

- Docker Compose Specification (v3). [https://docs.docker.com/compose/compose-file/](https://docs.docker.com/compose/compose-file/)

- Kubernetes Official Documentation. [https://kubernetes.io/docs/](https://kubernetes.io/docs/)

- Kubernetes Service Discovery. [https://kubernetes.io/docs/concepts/services-networking/service/](https://kubernetes.io/docs/concepts/services-networking/service/)

- Kubernetes StatefulSets. [https://kubernetes.io/docs/concepts/workloads/controllers/statefulset/](https://kubernetes.io/docs/concepts/workloads/controllers/statefulset/)

- Wiggins, A. (2017). *The Twelve-Factor App*. [https://12factor.net/](https://12factor.net/)
```

**Rationale**: Converts URLs to Markdown links for better usability in digital versions. Adds two K8s-specific reference links for readers diving deeper into those chapters.

---

### File: `/sessions/blissful-upbeat-mccarthy/mnt/blockchain/book-draft/Appendix-Source-Reference.md`

#### Revision BM-5: Add Chapter 24 to Summary Table

**Location**: Line 456 (Summary table, after "— | Whitepaper" row)

**Original Table** (lines 444–456):
```markdown
| Chapter | Module | Framework | Repo Path |
|---------|--------|-----------|-----------|
| 16 | Desktop Admin | Iced (Rust GUI) | `bitcoin-desktop-ui-iced/` |
| 17 | Desktop Admin | Tauri (Rust + React) | `bitcoin-desktop-ui-tauri/` |
| 18 | Wallet UI | Iced (Rust GUI) | `bitcoin-wallet-ui-iced/` |
| 19 | Wallet UI | Tauri (Rust + React) | `bitcoin-wallet-ui-tauri/` |
| 20 | Embedded DB | SQLCipher | `(wallet UIs)` |
| 21 | Web Admin | React/TypeScript | `bitcoin-web-ui/` |
| 22 | Docker Compose | Docker | `ci/docker-compose/` |
| 23 | Kubernetes | K8s manifests | `ci/kubernetes/` |
| — | Whitepaper | Math/Encoding | `bitcoin-blockchain/whitepaper-rust/` |
```

**Revised Table**:
```markdown
| Chapter | Module | Framework | Repo Path |
|---------|--------|-----------|-----------|
| 16 | Desktop Admin | Iced (Rust GUI) | `bitcoin-desktop-ui-iced/` |
| 17 | Desktop Admin | Tauri (Rust + React) | `bitcoin-desktop-ui-tauri/` |
| 18 | Wallet UI | Iced (Rust GUI) | `bitcoin-wallet-ui-iced/` |
| 19 | Wallet UI | Tauri (Rust + React) | `bitcoin-wallet-ui-tauri/` |
| 20 | Embedded DB | SQLCipher | `(wallet UIs)` |
| 21 | Web Admin | React/TypeScript | `bitcoin-web-ui/` |
| 22 | Docker Compose | Docker | `ci/docker-compose/` |
| 23 | Kubernetes | K8s manifests | `ci/kubernetes/` |
| 24 | Rust Language | Reference Guide | `rust/` |
| — | Whitepaper | Math/Encoding | `bitcoin-blockchain/whitepaper-rust/` |
```

**Rationale**: Table now includes all 24 chapters. Readers can see Chapter 24 is a reference guide living in `rust/` directory.

---

#### Revision BM-4: Path Formatting Consistency

**Location**: Throughout Appendix (e.g., lines 15–30, 47–76, 262–273)

**Change Rule**: Standardize all directory paths to use relative notation without leading slash.

**Examples**:

**Original**:
```
```text
bitcoin-desktop-ui-iced/
├── Cargo.toml
└── src/
    ├── main.rs
```
```

**Revised**:
```
```text
bitcoin-desktop-ui-iced/
├── Cargo.toml
└── src/
    ├── main.rs
```
```
(No change in this case—already correct.)

**Original** (if any absolute paths exist):
```
/bitcoin-blockchain/primitives/
```

**Revised**:
```
bitcoin-blockchain/primitives/
```

**Scope**: Check all directory paths in lines 15–420. All should use relative notation (no leading slash).

**Rationale**: Consistency with modern documentation style. Relative paths are easier to understand as "from the repository root."

---

## Summary of Changes

| Document | Issue ID | Type | Lines | Change Type |
|----------|----------|------|-------|-------------|
| Docker Compose Ch 22 | 22-7 | Content | 327+ | Add security warning callout |
| Docker Compose Ch 22 | 22-6 | Consistency | 314, 317, 541 | Fix script naming |
| Docker Compose Ch 22 | 22-2 | Clarity | 172+ | Add forward reference for Redis |
| Kubernetes Ch 23 | 23-2 | Clarity | 256+ | Enhance metrics-server explanation |
| Kubernetes Ch 23 | 23-1 | Structure | 77+ | Add guidance about deep-dive boxes |
| Kubernetes Ch 23 | 23-4 | Completeness | 473+ | Add alternative port-forward example |
| Rust Ch 24 | 24-1 | Clarity | 123–130 | Map sections to file numbers |
| Rust Ch 24 | 24-5 | Attribution | 72+ | Add source file comments to code |
| Glossary | BM-2 | Completeness | 112+ | Add 5 deployment terms |
| Bibliography | BM-3 | Formatting | 60–64 | Convert URLs to Markdown links |
| Appendix | BM-5 | Completeness | 456+ | Add Chapter 24 to summary table |
| Appendix | BM-4 | Consistency | 15–420 | Standardize path formatting |

---

## Implementation Notes

### For Copy-Paste Replacement

All revisions are designed as drop-in replacements:
1. Locate the section marked by **Location**
2. Replace **Original Text** with **Revised Text**
3. Save file
4. No manual reformatting needed

### Testing After Revisions

- [ ] Verify all links in Bibliography render correctly
- [ ] Confirm Chapter 24 appears in Appendix summary table
- [ ] Spot-check that source file comments appear in 3–4 code blocks
- [ ] Visually confirm script names are consistent throughout Ch 22

---

## Sign-Off

**Editor**: Technical Editor
**Date**: 2026-03-21
**Status**: ✅ Ready for Implementation

All revisions are editorial (no technical changes) and can be applied immediately.

