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

8. [Chapter 8: Docker Compose Deployment](../docker-compose/01-Introduction.md)
9. [Chapter 9: Kubernetes Deployment](README.md) - Main chapter
   - [Section 1: Introduction & Quick Start](README.md)
   - **Section 2: Architecture & Core Concepts** ← *You are here*
   - [Section 3: Migration Guide](03-Migration.md)
   - [Section 4: Kubernetes Manifests](04-Manifests.md)
   - [Section 5: Deployment & Operations](05-Deployment.md)
   - [Section 6: Autoscaling](06-Autoscaling.md)
   - [Section 7: Production & Advanced Topics](07-Production.md)

</details>

</div>

<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

# Chapter 8, Section 2: Architecture & Core Concepts

**Part II: Deployment & Operations** | **Chapter 9: Kubernetes Deployment**

<div align="center">

**📚 [← Chapter 8: Docker Compose](../docker-compose/01-Introduction.md)** | **Chapter 9: Kubernetes** | **[End of Book →](#)** 📚

</div>

---

This section explains Kubernetes architecture, core concepts, and how they differ from Docker Compose.

## Architecture Overview

### Current Docker Compose Architecture

```
┌──────────────────────────────────────┐
│         Docker Host                  │
│                                      │
│  ┌──────────┐  ┌──────────┐          │
│  │ miner_1  │  │ miner_2  │          │
│  │ :2001    │  │ :2002    │          │
│  └────┬─────┘  └────┬─────┘          │
│       │             │                │
│  ┌────▼─────────────▼─────┐          │
│  │   webserver_1          │          │
│  │   :8080, :2101         │          │
│  └────────────────────────┘          │
│                                      │
└──────────────────────────────────────┘
```

### Kubernetes Architecture

```
┌────────────────────────────────────────────────────────────┐
│                    Kubernetes Cluster                      │
│                                                            │
│  ┌──────────────────┐      ┌──────────────────┐            │
│  │   Node 1         │      │   Node 2         │            │
│  │                  │      │                  │            │
│  │  ┌──────────┐    │      │  ┌──────────┐    │            │
│  │  │miner-0   │    │      │  │miner-1   │    │            │
│  │  │Pod       │    │      │  │Pod       │    │            │
│  │  └──────────┘    │      │  └──────────┘    │            │
│  │                  │      │                  │            │
│  │  ┌──────────┐    │      │  ┌──────────┐    │            │
│  │  │webserver │    │      │  │webserver │    │            │
│  │  │-0 Pod    │    │      │  │-1 Pod    │    │            │
│  │  └──────────┘    │      │  └──────────┘    │            │
│  └──────────────────┘      └──────────────────┘            │
│                                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              Service Layer                          │   │
│  │  miner-service (ClusterIP)                          │   │
│  │  webserver-service (LoadBalancer/NodePort)          │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │         HPA (Horizontal Pod Autoscaler)             │   │
│  │  Monitors CPU/Memory and scales pods automatically  │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

### Key Differences

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

## Kubernetes Core Concepts

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
  MINER_ADDRESS: "your-wallet-address-here"  # REQUIRED: Must be set to a valid wallet address
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

### StatefulSet

A StatefulSet manages stateful applications and provides:
- Stable, unique network identifiers
- Stable, persistent storage
- Ordered, graceful deployment and scaling
- Ordered, automated rolling updates

**Why StatefulSet for Miners:**
- Miners need stable pod names for chain topology
- Each miner needs its own persistent storage
- Ordered startup ensures proper connection chain

**Example:**
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: miner
spec:
  serviceName: miner-headless
  replicas: 3
  template:
    spec:
      containers:
      - name: blockchain-node
        image: blockchain-node:latest
```

### Deployment

A Deployment provides declarative updates for Pods and ReplicaSets.

**Why Deployment for Webservers:**
- Webservers are stateless (can be replaced)
- Need rolling updates and rollbacks
- Don't need stable pod names

**Example:**
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

### Service

A Service provides a stable network endpoint for pods.

**Types:**
- **ClusterIP**: Internal service (default)
- **NodePort**: Exposes service on each node's IP
- **LoadBalancer**: External load balancer (cloud providers)
- **Headless**: No cluster IP, direct pod DNS (for StatefulSets)

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

### HorizontalPodAutoscaler (HPA)

HPA automatically scales the number of pods based on CPU, memory, or custom metrics.

**Example:**
```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: webserver-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
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

## Service Discovery & Networking

**How Pods Connect:**

In Kubernetes, pods connect using **Service DNS names**:
```
<service-name>.<namespace>.svc.cluster.local
```

**Example: Webserver Connecting to Miner**
```bash
# Docker Compose
NODE_CONNECT_NODES=miner_1:2001

# Kubernetes
NODE_CONNECT_NODES=miner-service.blockchain.svc.cluster.local:2001
```

**StatefulSet with Headless Service:**

For miners, we use StatefulSet with headless service for chain topology:
- `miner-0`: Seed node (`NODE_CONNECT_NODES="local"`)
- `miner-1`: Connects to `miner-0.miner-headless.blockchain.svc.cluster.local:2001`
- `miner-2`: Connects to `miner-1.miner-headless.blockchain.svc.cluster.local:2001`

## Persistent Storage

**PersistentVolumeClaim (PVC):**

PVCs provide persistent storage for pods:

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

## Resource Management

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
