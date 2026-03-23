<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../00-Quick-Start.md">Chapter 1: Quick Start</a>
2. <a href="../../01-Introduction.md">Chapter 2: Introduction & Overview</a>
3. <a href="../../bitcoin-blockchain/README.md">Chapter 3: Introduction to Blockchain</a>
4. <a href="../../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 4: Bitcoin Whitepaper</a>
5. <a href="../../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 5: Bitcoin Whitepaper in Rust</a>
6. <a href="../../bitcoin-blockchain/Rust-Project-Index.md">Chapter 6: Rust Blockchain Project</a>
7. <a href="../../bitcoin-blockchain/primitives/README.md">Chapter 7: Primitives</a>
8. <a href="../../bitcoin-blockchain/util/README.md">Chapter 8: Utilities</a>
9. <a href="../../bitcoin-blockchain/crypto/README.md">Chapter 9: Cryptography</a>
10. <a href="../../bitcoin-blockchain/chain/01-Domain-Model.md">Chapter 10: Domain Model</a>
11. <a href="../../bitcoin-blockchain/chain/02-Blockchain-State-Management.md">Chapter 11: Blockchain State Management</a>
12. <a href="../../bitcoin-blockchain/chain/03-Chain-State-and-Storage.md">Chapter 12: Chain State and Storage</a>
13. <a href="../../bitcoin-blockchain/chain/04-UTXO-Set.md">Chapter 13: UTXO Set</a>
14. <a href="../../bitcoin-blockchain/chain/05-Transaction-Lifecycle.md">Chapter 14: Transaction Lifecycle</a>
15. <a href="../../bitcoin-blockchain/chain/06-Block-Lifecycle-and-Mining.md">Chapter 15: Block Lifecycle and Mining</a>
16. <a href="../../bitcoin-blockchain/chain/07-Consensus-and-Validation.md">Chapter 16: Consensus and Validation</a>
17. <a href="../../bitcoin-blockchain/chain/08-Node-Orchestration.md">Chapter 17: Node Orchestration</a>
18. <a href="../../bitcoin-blockchain/chain/09-Transaction-To-Block.md">Chapter 18: Transaction to Block</a>
19. <a href="../../bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 19: Block Acceptance</a>
20. <a href="../../bitcoin-blockchain/store/README.md">Chapter 20: Storage Layer</a>
21. <a href="../../bitcoin-blockchain/net/README.md">Chapter 21: Network Layer</a>
22. <a href="../../bitcoin-blockchain/node/README.md">Chapter 22: Node Orchestration</a>
23. <a href="../../bitcoin-blockchain/wallet/README.md">Chapter 23: Wallet System</a>
24. <a href="../../bitcoin-blockchain/web/README.md">Chapter 24: Web API Architecture</a>
25. <a href="../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 25: Desktop Admin (Iced)</a>
26. <a href="../../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">25A: Code Walkthrough</a>
27. <a href="../../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">25B: Update Loop</a>
28. <a href="../../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">25C: View Layer</a>
29. <a href="../../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 26: Desktop Admin (Tauri)</a>
30. <a href="../../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">26A: Rust Backend</a>
31. <a href="../../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">26B: Frontend Infrastructure</a>
32. <a href="../../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">26C: Frontend Pages</a>
33. <a href="../../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 27: Wallet UI (Iced)</a>
34. <a href="../../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">27A: Code Listings</a>
35. <a href="../../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 28: Wallet UI (Tauri)</a>
36. <a href="../../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">28A: Rust Backend</a>
37. <a href="../../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">28B: Frontend Infrastructure</a>
38. <a href="../../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">28C: Frontend Pages</a>
39. <a href="../../embedded-database/06-Embedded-Database.md">Chapter 29: Embedded Database</a>
40. <a href="../../embedded-database/06A-Embedded-Database-Code-Listings.md">29A: Code Listings</a>
41. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 30: Web Admin Interface</a>
42. <a href="../../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">30A: Code Listings</a>
### Part II: Deployment & Operations

43. <a href="../docker-compose/01-Introduction.md">Chapter 31: Docker Compose Deployment</a>
44. <a href="../docker-compose/01A-Docker-Compose-Code-Listings.md">31A: Code Listings</a>
45. <a href="README.md">Chapter 32: Kubernetes Deployment</a>
46. <a href="01A-Kubernetes-Code-Listings.md">32A: Code Listings</a>
### Part III: Language Reference

47. <a href="../../rust/README.md">Chapter 33: Rust Language Guide</a>
### Appendices

48. <a href="../../Glossary.md">Glossary</a>
49. <a href="../../Bibliography.md">Bibliography</a>
50. <a href="../../Appendix-Source-Reference.md">Source Reference</a>

</details>

</div>

---
<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

## Chapter 23, Section 3: Migration Guide

**Part II: Deployment & Operations** | **Chapter 32: Kubernetes Deployment**

<div align="center">

**[← Chapter 31: Docker Compose](../docker-compose/01-Introduction.md)** | **Chapter 32: Kubernetes** | **[End of Book →](#)**

</div>

---

This section provides a step-by-step guide for migrating from Docker Compose to Kubernetes.

> **Methods involved:**
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
  WEBSERVER_NODE_CONNECT_NODES: >-
    miner-service.blockchain.svc.cluster.local:2001
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
  # Optional: if omitted/empty, the container entrypoint will
  # auto-create and persist it.
  # MINER_ADDRESS: ""
```

Apply:
```bash
cd ci/kubernetes/manifests
kubectl apply -f 03-secrets.yaml

# Secret values are injected as env vars at pod startup,
# so restart workloads after changes:
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

Create `11-hpa-miner.yaml` (similar structure, see [Chapter 20: Autoscaling](06-Autoscaling.md)).

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
kubectl exec -n blockchain <pod-name> -- \
  curl http://webserver-service:8080/api/health/ready
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

For more detailed troubleshooting, see [Chapter 21: Production & Advanced Topics](07-Production.md).
