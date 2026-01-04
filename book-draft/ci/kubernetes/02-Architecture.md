<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../01-Introduction.md) - Book introduction, project structure, technical stack
2. [Chapter 1.2: Introduction to Bitcoin & Blockchain](../../bitcoin-blockchain/README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 1.3: Bitcoin Whitepaper](../../bitcoin-blockchain/00-Bitcoin-Whitepaper-Summary.md) - Bitcoin Whitepaper
4. [Chapter 1.4: Bitcoin Whitepaper In Rust](../../bitcoin-blockchain/whitepaper-rust/README.md) - Bitcoin Whitepaper In Rust
5. [Chapter 2.0: Rust Blockchain Project](../../bitcoin-blockchain/Rust-Project-Index.md) - Blockchain Project
6. [Chapter 2.1: Primitives](../../bitcoin-blockchain/primitives/README.md) - Core data structures
7. [Chapter 2.2: Utilities](../../bitcoin-blockchain/util/README.md) - Utility functions and helpers
8. [Chapter 2.3: Cryptography](../../bitcoin-blockchain/crypto/README.md) - Cryptographic primitives and libraries
9. [Chapter 2.4: Blockchain(POW & Block Acceptance)](../../bitcoin-blockchain/chain/01-Technical-Foundations.md) - Proof Of Work
10. [Chapter 2.5: Storage Layer](../../bitcoin-blockchain/store/README.md) - Persistent storage implementation
11. [Chapter 2.6: Blockchain(POW & Block Acceptance)](../../bitcoin-blockchain/chain/02-Block-Acceptance-Whitepaper-Step-5.md) - Proof Of Work
12. [Chapter 2.7: Network Layer](../../bitcoin-blockchain/net/README.md) - Peer-to-peer networking and protocol
13. [Chapter 2.8: Node Orchestration](../../bitcoin-blockchain/node/README.md) - Node context and coordination
14. [Chapter 2.9: Wallet System](../../bitcoin-blockchain/wallet/README.md) - Wallet implementation and key management
15. [Chapter 3: Web API Architecture](../../bitcoin-blockchain/web/README.md) - REST API implementation
16. [Chapter 4: Desktop Admin Interface](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md) - Iced framework architecture
17. [Chapter 5: Wallet User Interface](../../bitcoin-wallet-ui/04-Wallet-UI.md) - Wallet UI implementation
18. [Chapter 6: Embedded Database & Persistence](../../bitcoin-wallet-ui/05-Embedded-Database.md) - SQLCipher integration
19. [Chapter 7: Web Admin Interface](../../bitcoin-web-ui/06-Web-Admin-UI.md) - React/TypeScript web UI

### Part II: Deployment & Operations

20. [Chapter 8: Docker Compose Deployment](../docker-compose/01-Introduction.md) - Docker Compose guide
21. **Chapter 9: Kubernetes Deployment** ← *You are here*
22. [Chapter 10: Rust Language Guide](../../rust/README.md) - Rust programming language reference

</details>

</div>

---
<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

# Chapter 9, Section 2: Architecture & Core Concepts

**Part II: Deployment & Operations** | **Chapter 9: Kubernetes Deployment**

<div align="center">

**📚 [← Chapter 8: Docker Compose](../docker-compose/01-Introduction.md)** | **Chapter 9: Kubernetes** | **[End of Book →](#)** 📚

</div>

---

This section explains Kubernetes architecture, core concepts, and how they differ from Docker Compose.

## How to Read This Section

This chapter is easiest to understand if you read it in three parts:

- **Part 1 (mental model)**: read the architecture diagram and the “relationships” section to understand how the building blocks connect.
- **Part 2 (primitives)**: read the core concepts (Namespace, ConfigMap, Secret, StatefulSet/Deployment, Service, HPA).
- **Part 3 (operations)**: read storage and resource management to understand what makes the system reliable in production.

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [Key Differences between Docker Compose and Kubernetes](#key-differences-between-docker-compose-and-kubernetes)
- [Layered Architecture Model (Recommended Mental Model)](#layered-architecture-model-recommended-mental-model)
- [Layer 0: Scope & Isolation (Namespace)](#layer-0-scope--isolation-namespace)
- [Layer 1: Configuration (ConfigMaps + Secrets)](#layer-1-configuration-configmaps--secrets)
- [Layer 2: Workloads (StatefulSet + Deployment)](#layer-2-workloads-statefulset--deployment)
- [Layer 3: Networking (Services + DNS)](#layer-3-networking-services--dns)
- [Layer 4: Scaling (HPA)](#layer-4-scaling-hpa)
- [Layer 5: Persistence (PVCs + StorageClasses)](#layer-5-persistence-pvcs--storageclasses)
- [Layer 6: Resource Governance (Requests/Limits)](#layer-6-resource-governance-requestslimits)
- [Kubernetes Core Concepts (Reference)](#kubernetes-core-concepts-reference)
- [Service Discovery & Networking (Deep Dive)](#service-discovery--networking-deep-dive)
- [Persistent Storage (Deep Dive)](#persistent-storage-deep-dive)
- [Resource Management (Deep Dive)](#resource-management-deep-dive)
- [Summary](#summary)

## Architecture Overview

### Current Docker Compose Architecture

```
┌──────────────────────────────┐
│         Docker Host          │
│                              │
│  ┌──────────┐  ┌──────────┐  │
│  │ miner_1  │  │ miner_2  │  │
│  │ :2001    │  │ :2002    │  │
│  └────┬─────┘  └────┬─────┘  │
│       │             │        │
│  ┌────▼─────────────▼─────┐  │
│  │   webserver_1          │  │
│  │   :8080, :2101         │  │
│  └────────────────────────┘  │
│                              │
└──────────────────────────────┘
```

### Kubernetes Architecture

```
┌───────────────────────────────────────────────────────────-─┐
│                    Kubernetes Cluster                       │
│                                                             │
│  ┌──────────────────┐                 ┌──────────────────┐  │
│  │   Node 1         │                 │   Node 2         │  │
│  │                  │                 │                  │  │
│  │  ┌──────────┐    │                 │  ┌──────────┐    │  │
│  │  │miner-0   │    │                 │  │miner-1   │    │  │
│  │  │Pod       │    │                 │  │Pod       │    │  │
│  │  └──────────┘    │                 │  └──────────┘    │  │
│  │                  │                 │                  │  │
│  │  ┌──────────┐    │                 │  ┌──────────┐    │  │
│  │  │webserver │    │                 │  │webserver │    │  │
│  │  │-0 Pod    │    │                 │  │-1 Pod    │    │  │
│  │  └──────────┘    │                 │  └──────────┘    │  │
│  │                  │                 │                  │  │
│  │  ┌──────────┐    │                 │                  │  │
│  │  │redis     │    │                 │                  │  │
│  │  │Pod       │    │                 │                  │  │
│  │  └──────────┘    │                 │                  │  │
│  └──────────────────┘                 └──────────────────┘  │
│                                                             │
│  ┌────────────────────────────────────────────────────--─┐  │
│  │                    Service Layer                      │  │
│  │  miner-headless (Headless, per-pod DNS for miners)    │  │
│  │  miner-service  (ClusterIP, stable “miner” endpoint)  │  │
│  │  webserver-headless (Headless, per-pod DNS)           │  │
│  │  webserver-service  (LoadBalancer/NodePort/port-fwd)  │  │
│  │  redis (ClusterIP, backend for rate limiting)         │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
│  ┌─────────────────────────────────────────────────--────┐  │
│  │         HPA (Horizontal Pod Autoscaler)               │  │
│  │  Monitors CPU/Memory and scales pods automatically    │  │
│  └───────────────────────────────────────────────────--──┘  │
│                                                             │
│  ┌──────────────────────────────────────────────────--───┐  │
│  │         Config & Secrets                              │  │
│  │  ConfigMap: `blockchain-config` (node settings)       │  │
│  │  Secret: API keys, optional MINER_ADDRESS             │  │
│  │  ConfigMap: `rate-limit-settings` (Settings.toml)     │  │
│  └────────────────────────────────────────────────────--─┘  │
│                                                             │
│  ┌────────────────────────────────────────────────────--─┐  │
│  │         Persistent Storage                            │  │
│  │  StatefulSet volumeClaimTemplates → per-pod PVCs      │  │
│  │  miner: chain DB + wallets (per miner)                │  │
│  │  webserver: chain DB + wallets (per webserver)        │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
└──────────────────────────────────────────────────────────-──┘
```

### Key Differences between Docker Compose and Kubernetes

**Docker Compose:**
- Single host deployment
- Manual port mapping
- Container-based networking
- Manual scaling

**Kubernetes:**
- Multi-node cluster
- Service-based networking
- Automatic load balancing
- Native autoscaling
- First-class stateful primitives (StatefulSets + per-pod PVCs)

## Layered Architecture Model (Easier To Understand - Mental Model)

Kubernetes can feel like “a lot of YAML” until you adopt the right mental model. In practice, most systems (including this repo) can be understood as **layers** that build on each other:

- **Layer 0 (Scope)**: where do objects live? (Namespace)
- **Layer 1 (Configuration)**: what do pods read at startup? (ConfigMaps, Secrets, mounted files)
- **Layer 2 (Workloads)**: what creates and maintains pods? (StatefulSet, Deployment)
- **Layer 3 (Networking)**: how do clients/peers reach pods? (Services, DNS, headless vs normal)
- **Layer 4 (Scaling)**: who changes replica counts automatically? (HPA + metrics)
- **Layer 5 (Persistence)**: where is state stored? (PVCs + StorageClasses; per-pod PVCs for StatefulSets)
- **Layer 6 (Resource governance)**: how do we prevent noisy neighbors? (requests/limits, quotas)

If you ever feel lost, you can usually “locate the problem” by asking:

- Which **workload** created the pod?
- Which **Service/DNS name** is used to reach it?
- Which **ConfigMap/Secret** is it reading?
- Where is its **PVC** mounted (if stateful)?

## Layer 0: Scope & Isolation (Namespace)

In this repo, almost everything is created inside the `blockchain` namespace. Namespaces provide scoping, isolation, and a place to apply RBAC/quotas.

## Layer 1: Configuration (ConfigMaps + Secrets)

Pods consume configuration in two common ways:

- **Environment variables** (good for small key/value settings)
- **Mounted files** (good for structured config like TOML)

In this repo:

- `blockchain-config` provides node settings (mostly env vars)
- `rate-limit-settings` provides `Settings.toml` (mounted as a file)
- `blockchain-secrets` provides API keys and optional mining address (env vars)

## Layer 2: Workloads (StatefulSet + Deployment)

Workloads are controllers that create and reconcile pods:

- **StatefulSet**: stable identities + per-pod PVCs (used for miners and webservers here)
- **Deployment**: interchangeable replicas (used for stateless workloads; Redis is a Deployment here)

## Layer 3: Networking (Services + DNS)

Services give stable names/ports:

- **Normal Service**: stable front-door + load balancing to any ready pod
- **Headless Service**: stable per-pod identity (DNS to endpoints), commonly used as the governing service for StatefulSets

## Layer 4: Scaling (HPA)

HPA adjusts replica counts automatically based on metrics (typically CPU/memory via `metrics-server`).

## Layer 5: Persistence (PVCs + StorageClasses)

Stateful pods write to PVC-backed storage. StatefulSets typically create per-pod PVCs to avoid shared disk locks for databases.

## Layer 6: Resource Governance (Requests/Limits)

Requests/limits control scheduling and resource isolation. They interact with HPA (scale) and cluster autoscaling (add nodes).

## Kubernetes Core Concepts (Reference)

This section describes the primitives Kubernetes gives you. The key is understanding how they compose:

- **Namespace** scopes everything to `blockchain`.
- **ConfigMaps** and **Secrets** provide configuration that pods consume via env vars or mounted files.
- **StatefulSets/Deployments** create and manage pods.
- **Services** provide stable networking (either load-balanced “group” access, or headless per-pod identity).
- **HPA** can scale workloads (including StatefulSets) based on metrics.
- **PVCs + StorageClasses** provide persistence; for StatefulSets we use per-pod PVCs to avoid shared-disk locking.

**If you ever feel lost, ask yourself:** 
- What creates pods (controller eg StatefulSet and Deployment)? controller chain is (Deployment → ReplicaSet → Pods)
- What config do pods read (ConfigMap/Secret)? 
- How do other things reach those pods (Service)? 
- Where is state stored (PVC)?

### Namespace

A Namespace in Kubernetes is a way to divide cluster resources between multiple users, teams, or applications. Think of it as a virtual cluster within your physical Kubernetes cluster.

**Key Characteristics:**
1. **Resource Isolation**: Namespaces provide a scope for names
2. **Resource Quotas**: Set resource quotas per namespace
3. **Access Control**: RBAC policies can be scoped to namespaces
4. **Logical Grouping**: Organize and group related resources

**Example:**
```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: blockchain
  labels:
    name: blockchain
    environment: production
```

**Common Commands:**
```bash
# List all namespaces
kubectl get namespaces

# Create a namespace
kubectl create namespace blockchain

# View resources in a namespace
kubectl get pods -n blockchain
```

### ConfigMap

A ConfigMap is a Kubernetes object used to store non-confidential configuration data in key-value pairs.

**Key Characteristics:**
1. **Non-Sensitive Data**: Use Secrets for sensitive data
2. **Key-Value Pairs**: Store configuration as key-value pairs
3. **Multiple Formats**: Literal values, files, or directories
4. **Immutable**: Can be made immutable (Kubernetes 1.19+)

**Example:**
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: blockchain-config
  namespace: blockchain
data:
  MINER_NODE_IS_MINER: "yes"
  MINER_NODE_CONNECT_NODES: "local"
  WEBSERVER_NODE_CONNECT_NODES: "miner-service.blockchain.svc.cluster.local:2001"
```

**How Pods Use ConfigMaps:**
```yaml
env:
- name: NODE_IS_MINER
  valueFrom:
    configMapKeyRef:
      name: blockchain-config
      key: MINER_NODE_IS_MINER
```

**How this repo uses ConfigMaps (two roles):**

- **`blockchain-config`**: general node settings used by miners and webservers (connect targets, feature toggles, wallet path). These are mostly consumed as **environment variables**.
- **`rate-limit-settings`**: contains a `Settings.toml` document used by the webserver’s rate limiter. This is consumed as a **mounted file** (e.g. `/app/Settings.toml`) and pointed to via `RL_SETTINGS_PATH`.

### Secret

A Secret is similar to a ConfigMap but designed for sensitive data like API keys and passwords.

**Key Points:**
1. **Base64 Encoded**: Values are base64 encoded (not encrypted by default)
2. **Type: Opaque**: Generic secret type for arbitrary user-defined data
3. **Security**: For production, use sealed-secrets or external secret management

**Example:**
```yaml
apiVersion: v1
kind: Secret
metadata:
  name: blockchain-secrets
  namespace: blockchain
type: Opaque
stringData:
  BITCOIN_API_ADMIN_KEY: "admin-secret"
  BITCOIN_API_WALLET_KEY: "wallet-secret"
  # Optional: if omitted/empty, the container entrypoint will auto-create and persist it.
  # MINER_ADDRESS: ""
```

**How It's Used:**
```yaml
env:
- name: BITCOIN_API_ADMIN_KEY
  valueFrom:
    secretKeyRef:
      name: blockchain-secrets
      key: BITCOIN_API_ADMIN_KEY
```

**Important runtime detail:**

- When Secrets are injected as **environment variables**, pods typically need to be **restarted** to pick up changes, because env vars are read at process start.

### StatefulSet

A **StatefulSet** is a workload controller for **stateful pods**. It is responsible for creating and managing pods in a way that preserves **identity** and **storage** across restarts and reschedules.

At a high level, a StatefulSet provides:

- **Stable pod identity**: pods are named with ordinals (`miner-0`, `miner-1`, …) and keep those names.
- **Stable network identity**: each pod can be addressed consistently via DNS when used with a “governing” Service (typically headless).
- **Stable storage**: each pod gets its own PersistentVolumeClaims via `volumeClaimTemplates` (e.g. `miner-data-miner-0`, `miner-data-miner-1`, …).
- **Ordered operations**: optional ordered startup/termination and rolling updates (important when peers must come up in sequence).

StatefulSets are usually paired with a **headless Service** (the “governing service”) to enable stable per-pod DNS identities like `miner-0.miner-headless...`.
The deeper networking details—headless vs normal Services, DNS behavior, and why we use both—are covered in **Service Discovery & Networking** below.

**Example:**
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: webserver
  namespace: blockchain
spec:
  # Governing service (typically headless) used for stable per-pod DNS identities:
  #   webserver-0.webserver-headless.blockchain.svc.cluster.local
  serviceName: webserver-headless
  replicas: 2
  selector:
    matchLabels:
      app: webserver
  template:
    metadata:
      labels:
        app: webserver
    spec:
      containers:
      - name: blockchain-node
        image: blockchain-node:latest
        ports:
        - name: web
          containerPort: 8080
        - name: p2p
          containerPort: 2001
        volumeMounts:
        - name: webserver-data
          mountPath: /app/data
        - name: webserver-wallets
          mountPath: /app/wallets
  volumeClaimTemplates:
  - metadata:
      name: webserver-data
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 50Gi
  - metadata:
      name: webserver-wallets
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 1Gi
```

**How this relates to storage and Services:**

- The StatefulSet creates the pods (`webserver-0`, `webserver-1`, …).
- The `volumeClaimTemplates` create **one PVC per pod**, ensuring each webserver/miner has isolated disk state.
- A **headless Service** is used to give each pod a stable DNS identity; a **normal Service** is used for load-balanced access to “the group”.

### Deployment

A Deployment is a controller (more precisely, a Kubernetes workload/controller object) that provides declarative updates for Pods and ReplicaSets.

Deployments are ideal for **stateless** workloads where any replica is interchangeable and there is no per-replica disk state (e.g. many frontends, API gateways, background workers).

In this repo, **miners and webservers are stateful** (disk-backed chain DB + wallets), so they are modeled as **StatefulSets** instead of Deployments.

**Example (generic):**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: webserver
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: blockchain-node
        image: blockchain-node:latest
```

### Deployment vs StatefulSet

This is one of the most important “modeling choices” you make in Kubernetes. A **Deployment** and a **StatefulSet** both run multiple pod replicas, but they make very different guarantees.

#### What a Deployment guarantees (and what it doesn’t)

A **Deployment** manages a ReplicaSet, which creates pods that are treated as **interchangeable**:

- **Identity**: pods have *no stable identity*. A pod name is essentially disposable.
- **Networking**: you typically access Deployments through a normal Service (load-balanced). There is no concept of “pod 0” being special.
- **Storage**: you *can* mount storage, but Deployments don’t have first-class per-replica PVC templating. If multiple replicas share the same PVC/path, you can hit **file-locking/data corruption** issues for databases.
- **Rollouts**: Deployments shine at rolling updates for stateless services (swap pods gradually behind a Service).

**Best use cases**:
- Web frontends
- Stateless REST APIs
- Workers where any replica can pick up any job

#### What a StatefulSet guarantees

A **StatefulSet** is built for workloads where **replica identity matters**:

- **Stable pod identity**: deterministic names with ordinals (`miner-0`, `miner-1`, `webserver-0`, …).
- **Stable storage per replica**: `volumeClaimTemplates` creates a *separate PVC per pod* (e.g. `webserver-data-webserver-0`, `webserver-data-webserver-1`).
- **Stable network identity**: when paired with a headless “governing” Service, each replica gets a stable DNS name (`pod.service.namespace.svc.cluster.local`).
- **Ordered rollout/scaling semantics**: StatefulSets can create/terminate pods in order, which is useful when replicas form a topology (seed nodes, upstream/downstream).

**Best use cases**:
- Databases (Postgres, Redis with persistence, etc.)
- Consensus/peer-to-peer systems
- Anything that stores state on disk and must not be shared between replicas

#### Why this repo uses StatefulSet for both miners and webservers

In many architectures, “webserver” implies stateless. In *this* repo, the webserver is a full blockchain node with:

- a disk-backed **chain database** (Sled),
- a disk-backed **wallet** directory,
- and peer connectivity.

That makes it **stateful**, and running multiple replicas as a Deployment would tempt you to share storage paths—causing errors like Sled DB lock contention (`could not acquire lock on .../db`). A StatefulSet with per-pod PVCs avoids that by construction.

Miners are also stateful for the same reasons (disk-backed chain DB/wallets), and they also benefit from stable identity for forming a predictable topology.

#### Quick “when to choose what?”

- Choose a **Deployment** when: replicas are interchangeable and you can replace any pod at any time with no data loss.
- Choose a **StatefulSet** when: replicas need stable identity, stable storage, or stable per-replica DNS—and especially when running databases or peer-to-peer nodes.

### Service

A Service provides a stable network endpoint for pods.

**Types:**
- **ClusterIP**: Internal service (default)
- **NodePort**: Exposes service on each node's IP
- **LoadBalancer**: External load balancer (cloud providers)
- **Headless**: No cluster IP, direct pod DNS (for StatefulSets)

**Relationship to workloads:**

- Services do not create pods. They select existing pods via **labels** (e.g. `app: webserver`) and provide a stable name/port in front of them.
- A normal Service is ideal for “talk to the group” use cases (HTTP APIs).
- A headless Service is ideal when “which replica” matters (StatefulSet per-pod DNS identities).

**Example:**
```yaml
apiVersion: v1
kind: Service
metadata:
  name: webserver-service
spec:
  type: LoadBalancer
  selector:
    app: webserver
  ports:
  - port: 8080
    targetPort: 8080
```

## Service Discovery & Networking (Deep Dive)

**How Pods Connect:**

In Kubernetes, pods connect using **Service DNS names**:
```
<service-name>.<namespace>.svc.cluster.local
```

### Normal Service vs Headless Service (and why you often see *both*)

In practice, it’s common to have **two Services** for the same app:

1) a **headless Service** (`clusterIP: None`) for stable *per-pod identity* (DNS),
2) a **normal Service** (ClusterIP/NodePort/LoadBalancer) for stable *front-door access* (load balancing).

#### 1. Normal Service (ClusterIP/NodePort/LoadBalancer)

A normal Service assigns a **virtual IP** (ClusterIP) and load-balances connections to **any Ready backend** behind its selector.

##### Config example (normal Service / load-balanced):

```yaml
apiVersion: v1
kind: Service
metadata:
  name: webserver-service
  namespace: blockchain
spec:
  # Common options:
  # - ClusterIP (default): internal-only access inside the cluster
  # - NodePort: expose on every node IP:port (local clusters)
  # - LoadBalancer: provision a cloud LB (or use minikube tunnel)
  type: ClusterIP
  selector:
    app: webserver
  ports:
    - name: web
      port: 8080        # Service port (what clients connect to)
      targetPort: 8080  # Pod/container port (what the pod listens on)
      protocol: TCP
```

Technically:

- `kube-proxy` programs iptables/IPVS rules so traffic to the Service VIP is routed to one of the endpoints.
- DNS for the Service name resolves to the **Service virtual IP** (for ClusterIP services).
- You get a stable “front door” that **does not preserve per-pod identity**.

Use this when the caller wants “talk to *the set*”:

- HTTP APIs (`webserver-service:8080`)
- Port-forwarding a single name (`kubectl port-forward svc/webserver-service ...`)
- Any request where replica identity doesn’t matter

#### 2. Headless Service (`clusterIP: None`)

A headless Service disables the Service VIP and load-balancing. Its primary purpose is **DNS identity**, not routing.

##### Config example (headless Service / per-pod identity):

```yaml
apiVersion: v1
kind: Service
metadata:
  name: webserver-headless
  namespace: blockchain
spec:
  clusterIP: None   # This is what makes it "headless"
  selector:
    app: webserver
  ports:
    - name: web
      port: 8080
      targetPort: 8080
      protocol: TCP
    - name: p2p
      port: 2001
      targetPort: 2001
      protocol: TCP
```

Technically:

- DNS for the Service name resolves to the **pod IPs directly** (A records for endpoints), instead of a single ClusterIP.
- For StatefulSets, the headless Service is also the **governing service** (`spec.serviceName`). That enables stable per-pod DNS names like:
  - `miner-0.miner-headless.blockchain.svc.cluster.local`
  - `miner-1.miner-headless.blockchain.svc.cluster.local`

This matters for stateful systems because peers often need to connect to **a specific node** (seed node, leader, or a specific replica), not “any node”.

##### Example: Webserver Connecting to Miner
```bash
# Docker Compose
NODE_CONNECT_NODES=miner_1:2001

# Kubernetes
NODE_CONNECT_NODES=miner-service.blockchain.svc.cluster.local:2001
```

### Why miners use a headless Service (technically)

Miners form a peer-to-peer topology where identity matters:

- `miner-0` acts as the seed node, and other miners connect to a specific upstream miner.
- With a headless Service, `miner-1` can consistently resolve `miner-0.miner-headless...` to miner-0’s pod IP.
- This survives reschedules because the **pod name stays the same** and the per-pod DNS record updates if the underlying IP changes.

### Why webservers use a headless Service (technically)

In this repo, webservers are also **stateful** (each has its own blockchain DB + wallets), so we run them as a StatefulSet. The headless Service is used as the StatefulSet’s **governing service** (`spec.serviceName`), which enables:

- **Stable DNS per replica** (`webserver-0.webserver-headless...`, `webserver-1.webserver-headless...`) for targeted debugging and any future replica-aware behavior.
- An identity model that matches the storage model (each replica has its own PVCs).

Even if clients typically use the normal `webserver-service`, the headless Service is what gives the StatefulSet its per-pod DNS identities.

### StatefulSet with Headless Service: miner chain topology

For miners, we use StatefulSet with headless service for chain topology:
- `miner-0`: Seed node (`NODE_CONNECT_NODES="local"`)
- `miner-1`: Connects to `miner-0.miner-headless.blockchain.svc.cluster.local:2001`
- `miner-2`: Connects to `miner-1.miner-headless.blockchain.svc.cluster.local:2001`

## Persistent Storage (Deep Dive)

**PersistentVolumeClaim (PVC):**

PVCs provide persistent storage for pods:

**Relationship to StatefulSets and databases:**

- PVCs are how pods get **durable disk**. Without PVCs, pod filesystems are ephemeral.
- For **StatefulSets**, we typically use `volumeClaimTemplates` so each replica gets its *own* PVC (avoids shared-disk locking for databases like Sled).
- Storage is provisioned via a **StorageClass** (cloud-specific), and the PVC binds to a PersistentVolume behind the scenes.

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: miner-data-pvc
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 50Gi
```

**Storage Classes:**

Storage classes define different storage types:

```yaml
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: fast-ssd
provisioner: kubernetes.io/aws-ebs
parameters:
  type: gp3
```

### HorizontalPodAutoscaler (HPA)

HPA automatically scales the number of pods based on CPU, memory, or custom metrics.

**Relationship to workloads and metrics:**

- HPA targets a **workload controller** (Deployment or StatefulSet) via `scaleTargetRef`.
- HPA needs a metrics source (typically the Kubernetes Metrics API via `metrics-server`) for CPU/memory-based scaling.
- Scaling a **StatefulSet** increases/decreases replicas by creating/deleting ordinal pods (e.g. adding `webserver-2`).

**Example:**
```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: webserver-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: StatefulSet
    name: webserver
  minReplicas: 1
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
```

## Resource Management (Deep Dive)

**Requests and Limits:**

```yaml
resources:
  requests:
    cpu: "500m"
    memory: "512Mi"
  limits:
    cpu: "2000m"
    memory: "2Gi"
```

- **Requests**: Guaranteed resources, used for scheduling
- **Limits**: Maximum resources, prevents resource exhaustion

**Relationship to scheduling and autoscaling:**

- The scheduler places pods based on **requests** (not limits).
- HPA uses **metrics** (CPU/memory usage) and scales replicas, but it cannot create capacity on its own. If the cluster has no room to schedule new pods, they remain `Pending`.
- In production, combine HPA with a **Cluster Autoscaler** (cloud provider feature) to add nodes when the cluster is full.

## Summary

Kubernetes provides:
- **Declarative Model**: Describe desired state, Kubernetes achieves it
- **Controllers**: Continuously reconcile desired vs actual state
- **Service Discovery**: DNS-based service discovery
- **Autoscaling**: Native HPA for automatic scaling
- **Self-Healing**: Automatic restart and replacement
- **Rolling Updates**: Zero-downtime updates

For detailed information about each concept, see the complete guide sections on Namespace, ConfigMap, Secret, StatefulSet, Deployment, Service, and HPA.

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Introduction & Quick Start](README.md) | [↑ Table of Contents](#) | [Next Section: Migration Guide →](03-Migration.md) |
|:---:|:---:|:---:|
| *Section 1* | *Current Section* | *Section 3* |

</div>

---
