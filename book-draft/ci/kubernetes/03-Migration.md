<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a> - Book introduction, project structure, technical stack
2. <a href="../../bitcoin-blockchain/README.md">Chapter 1.2: Introduction to Bitcoin & Blockchain</a> - Bitcoin and blockchain fundamentals
3. <a href="../../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 1.3: Bitcoin Whitepaper</a> - Bitcoin Whitepaper
4. <a href="../../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 1.4: Bitcoin Whitepaper In Rust</a> - Bitcoin Whitepaper In Rust
5. <a href="../../bitcoin-blockchain/Rust-Project-Index.md">Chapter 2.0: Rust Blockchain Project</a> - Blockchain Project
6. <a href="../../bitcoin-blockchain/primitives/README.md">Chapter 2.1: Primitives</a> - Core data structures
7. <a href="../../bitcoin-blockchain/util/README.md">Chapter 2.2: Utilities</a> - Utility functions and helpers
8. <a href="../../bitcoin-blockchain/crypto/README.md">Chapter 2.3: Cryptography</a> - Cryptographic primitives and libraries
9. <a href="../../bitcoin-blockchain/chain/README.md">Chapter 2.4: Blockchain (Technical Foundations)</a> - Proof Of Work
10. <a href="../../bitcoin-blockchain/store/README.md">Chapter 2.5: Storage Layer</a> - Persistent storage implementation
11. <a href="../../bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 2.6: Block Acceptance (Whitepaper §5, Step 5)</a> - Proof Of Work
12. <a href="../../bitcoin-blockchain/net/README.md">Chapter 2.7: Network Layer</a> - Peer-to-peer networking and protocol
13. <a href="../../bitcoin-blockchain/node/README.md">Chapter 2.8: Node Orchestration</a> - Node context and coordination
14. <a href="../../bitcoin-blockchain/wallet/README.md">Chapter 2.9: Wallet System</a> - Wallet implementation and key management
15. <a href="../../bitcoin-blockchain/web/README.md">Chapter 3: Web API Architecture</a> - REST API implementation
16. <a href="../../bitcoin-desktop-ui-iced/03-Desktop-Admin-UI.md">Chapter 4: Desktop Admin Interface</a> - Iced framework architecture
17. <a href="../../bitcoin-wallet-ui-iced/04-Wallet-UI.md">Chapter 5: Wallet User Interface</a> - Wallet UI implementation
18. <a href="../../bitcoin-wallet-ui-iced/05-Embedded-Database.md">Chapter 6: Embedded Database & Persistence</a> - SQLCipher integration
19. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 7: Web Admin Interface</a> - React/TypeScript web UI

### Part II: Deployment & Operations

20. <a href="../docker-compose/01-Introduction.md">Chapter 8: Docker Compose Deployment</a> - Docker Compose guide
21. **Chapter 9: Kubernetes Deployment** ← *You are here*
22. <a href="../../rust/README.md">Chapter 10: Rust Language Guide</a> - Rust programming language reference

</details>

</div>

---
<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

## Chapter 9, Section 3: Migration Guide

**Part II: Deployment & Operations** | **Chapter 9: Kubernetes Deployment**

<div align="center">

**📚 [← Chapter 8: Docker Compose](../docker-compose/01-Introduction.md)** | **Chapter 9: Kubernetes** | **[End of Book →](#)** 📚

</div>

---

This section provides a step-by-step guide for migrating from Docker Compose to Kubernetes.

> **Methods involved**
> - `deploy.sh` / `undeploy.sh` ([Listings 9.1–9.2](01A-Kubernetes-Code-Listings.md))
> - Workload and service manifests: [Listings 9.11–9.16](01A-Kubernetes-Code-Listings.md)

## Migration Overview

Migrating from Docker Compose to Kubernetes involves:
1. Building and pushing Docker images
2. Creating Kubernetes resources (ConfigMaps, Secrets, PVCs)
3. Creating Deployments/StatefulSets
4. Creating Services
5. Configuring HPA
6. Testing and verification

## Step 1: Build and Push Docker Image

### For Registry (Cloud/Production)

```bash
# Build the image
cd /path/to/repo/root
docker build -t blockchain-node:latest -f Dockerfile .

# Tag for registry (replace with your registry)
docker tag blockchain-node:latest your-registry/blockchain-node:v1.0.0

# Push to registry
docker push your-registry/blockchain-node:v1.0.0
```

### For Minikube (Local Development)

```bash
# Use Minikube's Docker daemon
eval $(minikube docker-env)

# IMPORTANT: build from the repository root (build context),
# because the Dockerfile uses COPY paths like `ci/docker-compose/configs/...`.
cd /path/to/repo/root
docker build -t blockchain-node:latest -f Dockerfile .

# Restore Docker to your normal local daemon (recommended)
eval $(minikube docker-env -u)
```

## Step 2: Create Kubernetes Namespace

```bash
kubectl create namespace blockchain
```

Or use the manifest:

```yaml
# 01-namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: blockchain
  labels:
    name: blockchain
    environment: production
```

Apply:
```bash
kubectl apply -f 01-namespace.yaml
```

## Step 3: Create ConfigMaps and Secrets

### ConfigMap

Use `ci/kubernetes/manifests/02-configmap.yaml` (recommended). It contains the node settings used by miners and webservers.

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: blockchain-config
  namespace: blockchain
data:
  MINER_NODE_IS_MINER: "yes"
  MINER_NODE_CONNECT_NODES: "local"
  WEBSERVER_NODE_IS_WEB_SERVER: "yes"
  WEBSERVER_NODE_CONNECT_NODES: "miner-service.blockchain.svc.cluster.local:2001"
  CENTERAL_NODE: ""
  SEQUENTIAL_STARTUP: "no"
  WALLET_FILE: "wallets/wallets.dat"
```

Apply:
```bash
cd ci/kubernetes/manifests
kubectl apply -f 02-configmap.yaml
```

### Secrets

Use `ci/kubernetes/manifests/03-secrets.yaml` (recommended).

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: blockchain-secrets
  namespace: blockchain
type: Opaque
stringData:
  BITCOIN_API_ADMIN_KEY: "your-secure-admin-key"
  BITCOIN_API_WALLET_KEY: "your-secure-wallet-key"
  # Optional: if omitted/empty, the container entrypoint will auto-create and persist it.
  # MINER_ADDRESS: ""
```

Apply:
```bash
cd ci/kubernetes/manifests
kubectl apply -f 03-secrets.yaml

# Secret values are injected as env vars at pod startup, so restart workloads after changes:
kubectl rollout restart statefulset/miner -n blockchain
kubectl rollout restart statefulset/webserver -n blockchain
```

### Rate limiting configuration (ConfigMap + Redis)

If you are using rate limiting, Kubernetes stores the `Settings.toml` text inside a ConfigMap and mounts it into the webserver pod:

- `ci/kubernetes/manifests/14-configmap-rate-limit.yaml` creates `rate-limit-settings` and embeds `Settings.toml`
- `ci/kubernetes/manifests/15-redis.yaml` deploys Redis as the backend (`redis:6379`)

Apply:

```bash
cd ci/kubernetes/manifests
kubectl apply -f 14-configmap-rate-limit.yaml
kubectl apply -f 15-redis.yaml
```

## Step 4: Create PersistentVolumeClaims

In the current Kubernetes setup for this repo, **miners and webservers are StatefulSets** and use `volumeClaimTemplates`, which means Kubernetes will create **one PVC per pod** automatically.

This is critical because both miners and webservers store a disk-backed chain DB and wallets, and sharing the same PVC/path across replicas can cause DB lock issues.

You can observe the generated PVC names like:

- `miner-data-miner-0`, `miner-data-miner-1`, ...
- `webserver-data-webserver-0`, `webserver-data-webserver-1`, ...

Verify:

```bash
kubectl get pvc -n blockchain
```

## Step 5: Create Deployments

### StatefulSet for Miners

Create `06-statefulset-miner.yaml` (see [Chapter 4: Kubernetes Manifests](04-Manifests.md) for complete example).

### StatefulSet for Webservers

Create `07-deployment-webserver.yaml` (note: despite the filename, it defines a **StatefulSet** for the webserver in the current repo).

Apply:
```bash
kubectl apply -f 06-statefulset-miner.yaml
kubectl apply -f 07-deployment-webserver.yaml
```

## Step 6: Create Services

### Miner Service (Headless)

Create `08-service-miner-headless.yaml`:

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

### Miner Service (ClusterIP)

Create `08-service-miner.yaml`:

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

### Webserver Service

Create `09-service-webserver.yaml`:

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

Apply:
```bash
kubectl apply -f 08-service-miner-headless.yaml
kubectl apply -f 08-service-miner.yaml
kubectl apply -f 09-service-webserver-headless.yaml
kubectl apply -f 09-service-webserver.yaml
```

## Step 7: Configure HPA

### Webserver HPA

Create `10-hpa-webserver.yaml`:

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
```

### Miner HPA

Create `11-hpa-miner.yaml` (similar structure, see [Chapter 6: Autoscaling](06-Autoscaling.md)).

Apply:
```bash
kubectl apply -f 10-hpa-webserver.yaml
kubectl apply -f 11-hpa-miner.yaml
```

## Step 8: Deploy and Verify

### Option 1: Using Kustomize (Recommended)

```bash
cd ci/kubernetes/manifests
kubectl apply -k .

# Verify deployment
kubectl get pods -n blockchain
kubectl get svc -n blockchain
kubectl get hpa -n blockchain
```

### Option 2: Using kubectl apply

```bash
cd ci/kubernetes/manifests
kubectl apply -f .

# Verify deployment
kubectl get pods -n blockchain
kubectl get svc -n blockchain
```

Note: `kubectl apply -f .` does not guarantee ordering. Prefer `./deploy.sh` or Kustomize for repeatability.

### Option 3: Using Deployment Script

```bash
cd ci/kubernetes/manifests
./deploy.sh
```

## Migration Checklist

**Pre-Migration:**
- [ ] Set up Kubernetes cluster (local or cloud)
- [ ] Install kubectl and configure access
- [ ] Build and push Docker image to registry
- [ ] Review current Docker Compose configuration
- [ ] Plan resource requirements (CPU, memory, storage)
- [ ] Plan network topology and service discovery

**Migration Steps:**
- [ ] Create namespace
- [ ] Create ConfigMaps with environment variables
- [ ] Create Secrets for sensitive data
- [ ] Create PersistentVolumeClaims
- [ ] Adapt entrypoint script for Kubernetes (if needed)
- [ ] Create Deployments (or StatefulSets)
- [ ] Create Services
- [ ] Configure HPA
- [ ] Test with single replica
- [ ] Scale up gradually
- [ ] Verify connectivity between pods
- [ ] Test autoscaling

**Post-Migration:**
- [ ] Monitor pod health and logs
- [ ] Verify HPA is working
- [ ] Test rolling updates
- [ ] Set up monitoring and alerting
- [ ] Document access methods
- [ ] Plan backup strategy
- [ ] Set up CI/CD for Kubernetes deployments

## Key Differences from Docker Compose

| Aspect | Docker Compose | Kubernetes |
|--------|---------------|------------|
| **Service Discovery** | `miner_1:2001` | `miner-service.blockchain.svc.cluster.local:2001` |
| **Scaling** | `--scale miner=3` | `kubectl scale statefulset miner --replicas=3` |
| **Port Mapping** | Manual in compose file | Service types (ClusterIP, LoadBalancer, NodePort) |
| **Storage** | Docker volumes | PersistentVolumeClaims |
| **Configuration** | Environment variables | ConfigMaps and Secrets |
| **Updates** | Manual restart | Rolling updates |

## Troubleshooting Migration

### Pods Not Starting

```bash
# Check pod status
kubectl get pods -n blockchain

# Describe pod for details
kubectl describe pod <pod-name> -n blockchain

# Check logs
kubectl logs <pod-name> -n blockchain
```

### Services Not Accessible

```bash
# Check service endpoints
kubectl get endpoints -n blockchain

# Test connectivity
kubectl exec -n blockchain <pod-name> -- curl http://webserver-service:8080/api/health/ready
```

### Configuration Issues

```bash
# Check ConfigMap
kubectl get configmap blockchain-config -n blockchain -o yaml

# Check rate limiting Settings.toml ConfigMap (if enabled)
kubectl get configmap rate-limit-settings -n blockchain -o yaml

# Check Secrets
kubectl get secret blockchain-secrets -n blockchain -o yaml
```

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Architecture & Core Concepts](02-Architecture.md) | [↑ Table of Contents](#) | [Next Section: Kubernetes Manifests →](04-Manifests.md) |
|:---:|:---:|:---:|
| *Section 2* | *Current Section* | *Section 4* |

</div>

For more detailed troubleshooting, see [Chapter 7: Production & Advanced Topics](07-Production.md).
