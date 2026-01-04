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

# Chapter 9, Section 4: Kubernetes Manifests

**Part II: Deployment & Operations** | **Chapter 9: Kubernetes Deployment**

<div align="center">

**📚 [← Chapter 8: Docker Compose](../docker-compose/01-Introduction.md)** | **Chapter 9: Kubernetes** | **[End of Book →](#)** 📚

</div>

---

This section provides complete Kubernetes manifest examples with detailed explanations. Each manifest file is numbered in deployment order.


## Overview of Manifest Files

### Why the ordering matters

Kubernetes is declarative, but resources still have **real dependencies**. Deploying in a sensible order makes the first rollout reliable and reduces confusing transient failures.

- **Namespace first**: most resources specify `namespace: blockchain`. If the namespace doesn’t exist yet, `kubectl apply` will fail.
- **ConfigMaps/Secrets before workloads**: pods reference them via `configMapKeyRef`, `secretKeyRef`, or mounted files. If they don’t exist, pods fail to start (CreateContainerConfigError) or crash at runtime due to missing config.
- **Storage before stateful workloads (where “volumes” fit)**:
  - In Kubernetes, “volumes” are just **mount points in a Pod spec**, but the underlying backing store is often a **Persistent Volume Claim (PVC)**.
  - For **StatefulSets**, the recommended pattern is `volumeClaimTemplates`, which means Kubernetes will create **one PVC per pod** automatically (e.g. `webserver-data-webserver-0`, `webserver-data-webserver-1`).
  - Those per-pod PVCs are created **when the StatefulSet creates pods**, but they still depend on your cluster having a working storage provisioner / default **StorageClass**. If storage can’t be provisioned, pods will sit in `Pending` with PVCs stuck in `Pending`.
  - If you are using **static/legacy PVC manifests** (like `04-pvc-miner.yaml` / `05-pvc-webserver.yaml`), apply them **before** the workloads that mount them, otherwise pods will fail to schedule/mount volumes.
- **Redis + rate-limit ConfigMap before webserver** (when rate limiting is enabled): the webserver uses an init container to wait for Redis, and it mounts `Settings.toml` from the rate-limit ConfigMap. If these aren’t present, webserver pods can stall in init or start without expected configuration.
- **Workloads before “operational” controllers**: HPA and PodDisruptionBudgets target an existing workload (StatefulSet/Deployment) by name/labels. Applying them after workloads avoids “target not found” confusion.
- **Services before clients rely on DNS/endpoints**: Services provide stable names and load balancing. Applying them early ensures DNS names exist before you start debugging connectivity (endpoints may still populate as pods become Ready).
- **NetworkPolicy last (optional)**: policies can unintentionally block traffic if applied too early. Apply after the basic cluster is healthy, then tighten ingress/egress with confidence.

### How `deploy.sh` wires everything up

The repository includes a deployment script at `ci/kubernetes/manifests/deploy.sh`. It is essentially an “opinionated `kubectl apply`” that:

- Runs **preflight checks** (verifies `kubectl` exists and you’re connected to a cluster via `kubectl cluster-info`)
- Applies manifests in a **known-safe dependency order**
- Handles **upgrade cleanup** when migrating from older setups (removes the legacy `deployment/webserver` so it can’t keep creating pods that share storage)
- Waits for pods to become Ready and prints a status summary + common follow-up commands

At a high level, it applies:

```bash
01-namespace.yaml
02-configmap.yaml
14-configmap-rate-limit.yaml
03-secrets.yaml
04-pvc-miner.yaml
15-redis.yaml
06-statefulset-miner.yaml
09-service-webserver-headless.yaml
# delete deployment/webserver (upgrade safety)
07-deployment-webserver.yaml
08-service-miner-headless.yaml
08-service-miner.yaml
09-service-webserver.yaml
10-hpa-webserver.yaml
11-hpa-miner.yaml
12-pod-disruption-budget.yaml
```

Then it performs readiness waits:

```bash
kubectl wait --for=condition=ready pod -l app=miner -n blockchain --timeout=300s
kubectl wait --for=condition=ready pod -l app=webserver -n blockchain --timeout=300s
```

Finally it prints “what next” helpers (logs and `kubectl port-forward ... svc/webserver-service 8080:8080`).

| File | Resource Type | Purpose | Key Configurations |
|------|--------------|---------|-------------------|
| `01-namespace.yaml` | Namespace | Creates isolated environment | Namespace name, labels |
| `02-configmap.yaml` | ConfigMap | Stores non-sensitive configuration | Node settings, connection strings |
| `14-configmap-rate-limit.yaml` | ConfigMap | Rate limiting settings (`Settings.toml`) | `redis_addr`, strategies, buckets |
| `03-secrets.yaml` | Secret | Stores sensitive data | API keys, base64 encoding |
| `04-pvc-miner.yaml` | PersistentVolumeClaim | (Legacy/optional) shared PVCs | Older Deployment-based storage |
| `05-pvc-webserver.yaml` | PersistentVolumeClaim | (Legacy/optional) shared PVCs | Older Deployment-based storage |
| `06-statefulset-miner.yaml` | StatefulSet | Defines miner pods | Replicas, volumeClaimTemplates, headless service |
| `07-deployment-webserver.yaml` | StatefulSet | Defines webserver pods | Replicas, init containers, **volumeClaimTemplates** |
| `08-service-miner-headless.yaml` | Service | Headless service for pod DNS | clusterIP: None, stable DNS per pod |
| `08-service-miner.yaml` | Service | Internal network endpoint | ClusterIP, session affinity |
| `09-service-webserver-headless.yaml` | Service | Headless service for webserver StatefulSet | clusterIP: None |
| `09-service-webserver.yaml` | Service | External network endpoint | LoadBalancer, port mappings |
| `10-hpa-webserver.yaml` | HorizontalPodAutoscaler | Auto-scales webservers | CPU/Memory thresholds, scaling policies |
| `11-hpa-miner.yaml` | HorizontalPodAutoscaler | Auto-scales miners | CPU threshold, conservative scaling |
| `12-pod-disruption-budget.yaml` | PodDisruptionBudget | Ensures minimum availability | Minimum available pods |
| `15-redis.yaml` | Deployment + Service | Redis backend for rate limiting | ClusterIP service, probes |
| `13-network-policy.yaml` | NetworkPolicy | (Optional) restrict traffic | Ingress/egress allow rules |
| `deploy.sh` | Script | Applies manifests in order | Safe, repeatable rollout |
| `undeploy.sh` | Script | Removes resources | Clean teardown |
| `kustomization.yaml` | Kustomize | Deploy all resources | `kubectl apply -k .` |

## 1. Namespace

**File**: `01-namespace.yaml`

Creates a Kubernetes namespace to isolate all blockchain-related resources.

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: blockchain
  labels:
    name: blockchain
    environment: production
```

**Key Points:**
- Namespace name must match `namespace:` field in all other manifests
- Labels used for resource organization and policies

## 2. ConfigMap

**File**: `02-configmap.yaml`

Stores all non-sensitive configuration data as key-value pairs.

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: blockchain-config
  namespace: blockchain
data:
  SEQUENTIAL_STARTUP: "no"
  CENTERAL_NODE: ""
  WALLET_FILE: "wallets/wallets.dat"
  MINER_NODE_IS_MINER: "yes"
  MINER_NODE_IS_WEB_SERVER: "no"
  MINER_NODE_CONNECT_NODES: "local"
  WEBSERVER_NODE_IS_MINER: "no"
  WEBSERVER_NODE_IS_WEB_SERVER: "yes"
  WEBSERVER_NODE_CONNECT_NODES: "miner-service.blockchain.svc.cluster.local:2001"
```

**Key Points:**
- `MINER_NODE_CONNECT_NODES: "local"` - Default for first miner (miner-0)
- `WEBSERVER_NODE_CONNECT_NODES` - Uses Kubernetes DNS format for service discovery

## 3. Secret

**File**: `03-secrets.yaml`

Stores sensitive data like API keys and passwords.

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

**Key Points:**
- `stringData` is plain text (Kubernetes encodes automatically)
- For production, use sealed-secrets or external secret management

## 4. `volumeClaimTemplates` (StatefulSet storage)

In the current repo, both **miners** and **webservers** run as **StatefulSets** and use `volumeClaimTemplates` to provision storage. This is the recommended Kubernetes pattern for stateful workloads because it guarantees **one PersistentVolumeClaim per replica** (per pod).

### What `volumeClaimTemplates` does:

`volumeClaimTemplates` is not a “volume” by itself. It is a *template* that the StatefulSet controller uses to create PVCs with deterministic names.

- You define a template named (for example) `webserver-data`.
- When the StatefulSet creates `webserver-0`, Kubernetes automatically creates a PVC named:
  - `webserver-data-webserver-0`
- When it creates `webserver-1`, it creates:
  - `webserver-data-webserver-1`

This matters because databases (like Sled) generally require **exclusive access** to their on-disk data directory. If multiple replicas share the same disk path, you can hit DB lock contention and corruption risks.

### How it connects to the Pod (mount wiring)

Inside the pod template you mount volumes by name using `volumeMounts`. For StatefulSets, the “volume” name you mount must match the template name:

- `volumeClaimTemplates[].metadata.name == volumeMounts[].name`

**What are `metadata` and `metadata.name`?**

- **`metadata`**: a standard field on *all* Kubernetes objects that holds “object metadata” (identity and organizational info). Common fields include:
  - `name`: the object’s name
  - `namespace`: where it lives (for namespaced objects)
  - `labels`: key/value tags used for selection and organization
  - `annotations`: arbitrary non-identifying key/value metadata

- **`metadata.name`**: the object’s name **within its scope**.
  - For top-level objects (Service/StatefulSet/ConfigMap), this is the name you reference elsewhere.
  - For a StatefulSet **`volumeClaimTemplates` entry**, `metadata.name` is the **template name** (e.g. `webserver-data`). Kubernetes uses it to generate per-pod PVC names (like `webserver-data-webserver-0`), and the pod uses that same template name as the `volumeMounts[].name`.

So a template named `webserver-data` is mounted by:

```yaml
volumeMounts:
  - name: webserver-data
    mountPath: /app/data
```

### Example (snippet)

```yaml
volumeClaimTemplates:
  - metadata:
      name: webserver-data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      resources:
        requests:
          storage: 50Gi
```

### Operational behavior to know

- **Provisioning depends on storage**: PVCs will remain `Pending` if your cluster has no default StorageClass / provisioner that can satisfy them.
- **Retention**: PVCs are typically retained even if pods restart (and often even if you scale down), which is usually what you want for stateful data.

## 5. PersistentVolumeClaim (PVC) (Legacy / optional)

PVC manifests are **legacy/optional** and were used in older Deployment-based setups where a single PVC was manually bound.

In the current repo, storage is primarily provisioned via `volumeClaimTemplates` (Section 4). You usually do **not** need to apply these PVCs unless you are experimenting with a static/shared-PVC architecture.

**File**: `04-pvc-miner.yaml` (legacy/optional)

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: miner-data-pvc
  namespace: blockchain
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 50Gi
```

**File**: `05-pvc-webserver.yaml` (legacy/optional)

## 6. StatefulSet (Miner)

**File**: `06-statefulset-miner.yaml`

Defines miner pods with stable names and chain topology.

**Key Features:**
- Stable pod names: `miner-0`, `miner-1`, `miner-2`
- Ordered startup
- Per-pod PVCs via `volumeClaimTemplates`
- Headless service for direct pod DNS

**Important Configurations:**
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: miner
  namespace: blockchain
spec:
  serviceName: miner-headless  # Links to headless service
  replicas: 3
  selector:
    matchLabels:
      app: miner
  template:
    metadata:
      labels:
        app: miner
    spec:
      containers:
      - name: blockchain-node
        image: blockchain-node:latest
        env:
        - name: POD_NAME
          valueFrom:
            fieldRef:
              fieldPath: metadata.name
        - name: NODE_IS_MINER
          valueFrom:
            configMapKeyRef:
              name: blockchain-config
              key: MINER_NODE_IS_MINER
        # ... more env vars ...
        volumeMounts:
        - name: miner-data
          mountPath: /app/data
        - name: miner-wallets
          mountPath: /app/wallets
  volumeClaimTemplates:
  - metadata:
      name: miner-data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      resources:
        requests:
          storage: 50Gi
  - metadata:
      name: miner-wallets
    spec:
      accessModes: [ "ReadWriteOnce" ]
      resources:
        requests:
          storage: 1Gi
```

## 7. StatefulSet (Webserver)

**File**: `07-deployment-webserver.yaml`

Defines **webserver StatefulSet** pods. Despite the filename, the current repo uses a StatefulSet so each webserver has isolated chain DB + wallets.

**Key Features:**
- Stable pod identities: `webserver-0`, `webserver-1`, ...
- Per-pod PVCs via `volumeClaimTemplates` (isolated chain DB + wallets)
- Init containers for dependency waiting (miner + Redis)
- Health checks (liveness and readiness probes)

**Important Configurations:**
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: webserver
  namespace: blockchain
spec:
  replicas: 2
  selector:
    matchLabels:
      app: webserver
  template:
    metadata:
      labels:
        app: webserver
    spec:
      initContainers:
      - name: wait-for-miner
        image: busybox:1.35
        command: ['sh', '-c', 'until nc -z miner-service.blockchain.svc.cluster.local 2001; do sleep 2; done;']
      containers:
      - name: blockchain-node
        image: blockchain-node:latest
        env:
        - name: NODE_IS_WEB_SERVER
          valueFrom:
            configMapKeyRef:
              name: blockchain-config
              key: WEBSERVER_NODE_IS_WEB_SERVER
        # ... more env vars ...
        livenessProbe:
          httpGet:
            path: /api/health/live
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /api/health/ready
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 5
        volumeMounts:
        - name: webserver-data
          mountPath: /app/data
        - name: webserver-wallets
          mountPath: /app/wallets
  volumeClaimTemplates:
  - metadata:
      name: webserver-data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      resources:
        requests:
          storage: 50Gi
  - metadata:
      name: webserver-wallets
    spec:
      accessModes: [ "ReadWriteOnce" ]
      resources:
        requests:
          storage: 1Gi
```

## 8. Services

### Headless Service (Miner)

**File**: `08-service-miner-headless.yaml`

Provides stable DNS per pod for direct pod-to-pod connections.

```yaml
apiVersion: v1
kind: Service
metadata:
  name: miner-headless
  namespace: blockchain
spec:
  clusterIP: None  # Headless service
  selector:
    app: miner
  ports:
  - port: 2001
    targetPort: 2001
```

### ClusterIP Service (Miner)

**File**: `08-service-miner.yaml`

Provides load-balanced internal network endpoint.

#### Relationship: `miner-headless` vs `miner-service` (why both exist)

Both Services typically select the **same set of pods** (here: `app: miner`). The difference is *how they expose those pods*:

- **`miner-headless` (headless Service, `clusterIP: None`)**
  - **Purpose**: per-pod identity for the **StatefulSet**.
  - **What you get**: stable DNS names for each replica, e.g.:
    - `miner-0.miner-headless.blockchain.svc.cluster.local:2001`
    - `miner-1.miner-headless.blockchain.svc.cluster.local:2001`
  - **Why it matters**: miners form a topology where connecting to a *specific* peer matters (seed node / upstream node). Headless DNS gives you deterministic addresses that survive rescheduling (DNS updates if pod IP changes).

- **`miner-service` (normal Service, `type: ClusterIP`)**
  - **Purpose**: a single stable “front door” DNS name for “talk to **a miner**”.
  - **What you get**: `miner-service.blockchain.svc.cluster.local:2001` load-balances to any Ready miner pod.
  - **Why it matters**: clients that don’t care which miner they hit (e.g. webservers syncing from “the miner set”, simple health checks, internal callers) can use one stable name.

**How they work together**

- The **StatefulSet** uses the headless Service to make replica identity usable in the network.
- The **ClusterIP Service** provides convenience and resiliency for “group” access.
- It is normal (and recommended) to have **both** for stateful peer systems: one for *identity* (headless), one for *load-balanced access* (ClusterIP).

```yaml
apiVersion: v1
kind: Service
metadata:
  name: miner-service
  namespace: blockchain
spec:
  type: ClusterIP
  selector:
    app: miner
  ports:
  - port: 2001
    targetPort: 2001
```

### LoadBalancer Service (Webserver)

**File**: `09-service-webserver.yaml`

Provides external network endpoint.

```yaml
apiVersion: v1
kind: Service
metadata:
  name: webserver-service
  namespace: blockchain
spec:
  type: LoadBalancer
  selector:
    app: webserver
  ports:
  - port: 8080
    targetPort: 8080
```

## 9. HorizontalPodAutoscaler

### Webserver HPA

**File**: `10-hpa-webserver.yaml`

Auto-scales webservers based on CPU and memory.

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: webserver-hpa
  namespace: blockchain
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
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
  behavior:
    scaleDown:
      stabilizationWindowSeconds: 300
      policies:
      - type: Percent
        value: 50
        periodSeconds: 60
    scaleUp:
      stabilizationWindowSeconds: 0
      policies:
      - type: Percent
        value: 100
        periodSeconds: 15
      - type: Pods
        value: 2
        periodSeconds: 15
      selectPolicy: Max
```

### Miner HPA

**File**: `11-hpa-miner.yaml`

Auto-scales miners based on CPU (conservative scaling).

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: miner-hpa
  namespace: blockchain
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: StatefulSet
    name: miner
  minReplicas: 1
  maxReplicas: 5
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 80
  behavior:
    scaleDown:
      stabilizationWindowSeconds: 600
    scaleUp:
      stabilizationWindowSeconds: 0
```

## 10. PodDisruptionBudget

**File**: `12-pod-disruption-budget.yaml`

Ensures minimum availability during disruptions.

### Why this is needed:

Kubernetes does “self-healing”, but there is an important distinction between:

- **Involuntary disruptions**: a node crashes, the kernel OOM-kills a container, a process segfaults, etc.
- **Voluntary disruptions**: an operator (or automation) intentionally evicts pods, usually due to:
  - `kubectl drain` (node maintenance / upgrades)
  - cluster autoscaler scale-down (node removal)
  - some forms of rolling infrastructure maintenance

A **PodDisruptionBudget (PDB)** protects you from *voluntary* disruptions taking down too many replicas at once. For a blockchain peer set, this matters because losing too many miners simultaneously can stall connectivity/propagation, and for stateful nodes it can increase recovery time as pods restart and resync.

### How it works

When a component tries to evict a pod, it typically uses the **Eviction API**. The Kubernetes disruption controller checks the matching PDB and decides whether the eviction is allowed:

- With `minAvailable: 1`, Kubernetes will block evictions that would take the number of **available** pods below 1 for the selected set.
- “Available” is based on readiness (Ready pods), not just “Running”.

### What it does NOT do

- It does **not** prevent involuntary failures (OOMKilled, node crash).
- It does **not** guarantee zero downtime by itself; it only constrains *how many can be voluntarily evicted at once*.
- It does **not** stop you from manually deleting pods (`kubectl delete pod ...`)—it is enforced on evictions, not all delete paths.

### How to verify it in a cluster

```bash
kubectl get pdb -n blockchain
kubectl describe pdb miner-pdb -n blockchain
```

```yaml
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: miner-pdb
  namespace: blockchain
spec:
  minAvailable: 1
  selector:
    matchLabels:
      app: miner
```

## Deployment Order

Deploy manifests in this order:

1. Namespace
2. ConfigMaps + Secrets (including rate limiting `Settings.toml`)
3. Redis (rate limiting backend, if enabled)
4. Workloads (StatefulSets)
5. Services (including headless services)
6. HPA
7. PodDisruptionBudget
8. NetworkPolicy (optional)

Or use **Kustomize** to deploy all at once:

```bash
kubectl apply -k .
```

### What is Kustomize?

**Kustomize** is a Kubernetes-native configuration tool for composing and modifying YAML without templates.

Technically, it works by taking a set of base manifests (listed under `resources:` in `kustomization.yaml`) and applying:

- **Patches** (strategic-merge or JSON6902) to change fields per environment
- **Name/label transformations** (common labels, prefixes/suffixes)
- **Generators** (e.g. create ConfigMaps/Secrets from files)

Kustomize is built into `kubectl`, which is why you can run `kubectl apply -k .`.

### Kustomize vs Helm (what’s the difference?)

- **Kustomize**:
  - **No templating** (no Go templates)
  - Keeps YAML “as YAML”, and changes it via overlays/patches
  - Great when you want minimal moving parts and manifests that stay close to raw Kubernetes

- **Helm**:
  - A **package manager** for Kubernetes with **templating**
  - Great when you need distributable charts, lots of configuration knobs, or third-party app installs
  - Adds a rendering step and chart conventions that can be overkill for small, repo-owned manifests

### Why this project uses Kustomize (and a script)

This repo’s manifests are **owned by the repo** (not a reusable “chart”), and we mainly need:

- a clean way to apply a consistent set of YAML files (`kustomization.yaml`)
- the option to add patches later (staging vs prod) without introducing templating
- a simple, readable workflow that matches the book-style documentation

That’s why we keep:

- **Kustomize** for “apply the whole set”
- **`deploy.sh`** for “apply in a safe order + do upgrade cleanup + wait for readiness”

Or use the script (recommended for repeatable ordering):

```bash
cd ci/kubernetes/manifests
./deploy.sh
```

For detailed explanations of each manifest, see the complete guide sections on each resource type.

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Migration Guide](03-Migration.md) | [↑ Table of Contents](#) | [Next Section: Deployment & Operations →](05-Deployment.md) |
|:---:|:---:|:---:|
| *Section 3* | *Current Section* | *Section 5* |

</div>

---
