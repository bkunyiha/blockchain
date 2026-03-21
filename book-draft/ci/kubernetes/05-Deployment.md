<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../../bitcoin-blockchain/README.md">Chapter 2: Introduction to Blockchain</a>
3. <a href="../../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="../../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. <a href="../../bitcoin-blockchain/Rust-Project-Index.md">Chapter 5: Rust Blockchain Project</a>
6. <a href="../../bitcoin-blockchain/primitives/README.md">Chapter 6: Primitives</a>
7. <a href="../../bitcoin-blockchain/util/README.md">Chapter 7: Utilities</a>
8. <a href="../../bitcoin-blockchain/crypto/README.md">Chapter 8: Cryptography</a>
9. <a href="../../bitcoin-blockchain/chain/README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. <a href="../../bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 10: Block Acceptance</a>
11. <a href="../../bitcoin-blockchain/store/README.md">Chapter 11: Storage Layer</a>
12. <a href="../../bitcoin-blockchain/net/README.md">Chapter 12: Network Layer</a>
13. <a href="../../bitcoin-blockchain/node/README.md">Chapter 13: Node Orchestration</a>
14. <a href="../../bitcoin-blockchain/wallet/README.md">Chapter 14: Wallet System</a>
15. <a href="../../bitcoin-blockchain/web/README.md">Chapter 15: Web API Architecture</a>
16. <a href="../../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 16: Desktop Admin (Iced)</a>
17. <a href="../../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">16A: Code Walkthrough</a>
18. <a href="../../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">16B: Update Loop</a>
19. <a href="../../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">16C: View Layer</a>
20. <a href="../../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 17: Desktop Admin (Tauri)</a>
21. <a href="../../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">17A: Rust Backend</a>
22. <a href="../../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">17B: Frontend Infrastructure</a>
23. <a href="../../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">17C: Frontend Pages</a>
24. <a href="../../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 18: Wallet UI (Iced)</a>
25. <a href="../../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">18A: Code Listings</a>
26. <a href="../../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 19: Wallet UI (Tauri)</a>
27. <a href="../../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">19A: Rust Backend</a>
28. <a href="../../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">19B: Frontend Infrastructure</a>
29. <a href="../../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">19C: Frontend Pages</a>
30. <a href="../../embedded-database/06-Embedded-Database.md">Chapter 20: Embedded Database</a>
31. <a href="../../embedded-database/06A-Embedded-Database-Code-Listings.md">20A: Code Listings</a>
32. <a href="../../bitcoin-web-ui/06-Web-Admin-UI.md">Chapter 21: Web Admin Interface</a>
33. <a href="../../bitcoin-web-ui/06A-Web-Admin-UI-Code-Listings.md">21A: Code Listings</a>

### Part II: Deployment & Operations

34. <a href="../docker-compose/01-Introduction.md">Chapter 22: Docker Compose Deployment</a>
35. <a href="../docker-compose/01A-Docker-Compose-Code-Listings.md">22A: Code Listings</a>
36. <a href="README.md">Chapter 23: Kubernetes Deployment</a>
37. <a href="01A-Kubernetes-Code-Listings.md">23A: Code Listings</a>

### Part III: Language Reference

38. <a href="../../rust/README.md">Chapter 24: Rust Language Guide</a>

</details>

</div>

---
<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

## Chapter 23, Section 5: Deployment & Operations

**Part II: Deployment & Operations** | **Chapter 23: Kubernetes Deployment**

<div align="center">

**[← Chapter 22: Docker Compose](../docker-compose/01-Introduction.md)** | **Chapter 23: Kubernetes** | **[End of Book →](#)**

</div>

---

This section is a practical, production-style guide to deploying and operating the blockchain network on Kubernetes. It focuses on repeatable commands, how to verify a healthy rollout, and how to perform common operational tasks safely.

> **Methods involved:**
> - `deploy.sh` / `undeploy.sh` ([Listings 9.1–9.2](01A-Kubernetes-Code-Listings.md))
> - Workload + service manifests: [Listings 9.10–9.16](01A-Kubernetes-Code-Listings.md)
> - HPA + PDB resources: [Listings 9.17–9.19](01A-Kubernetes-Code-Listings.md)

## How to Read This Section

- If you want the fastest path to a working cluster, read **Deployment Process** and use `./deploy.sh`.
- If you are operating a running cluster, jump to **Verification**, **Accessing Services**, **Monitoring**, and **Updates**.
- If something is broken, jump to **Troubleshooting** and use the commands there to narrow the failure mode quickly.

## Table of Contents

- [Deployment Process (Step-by-Step)](#deployment-process-step-by-step)
  - [Prerequisites](#prerequisites)
  - [Cluster Setup](#cluster-setup)
  - [Image Preparation](#image-preparation)
  - [Configuration](#configuration)
  - [Deploy](#deploy)
- [Verification](#verification)
- [Accessing Services](#accessing-services)
- [Monitoring](#monitoring)
- [Scaling](#scaling)
- [Updates](#updates)
- [Common Operations](#common-operations)
- [Troubleshooting](#troubleshooting)
- [Cleanup](#cleanup)

## Deployment Process (Step-by-Step)

### Prerequisites

You need:

- **`kubectl`** (talk to the cluster)
- **A cluster** (Minikube is recommended for local development)
- **Docker** (build the `blockchain-node` image)

### Cluster Setup

#### Option 1: Minikube (recommended for local development)

```bash
minikube start --cpus=4 --memory=3072mb --addons=metrics-server
```

Optional (handy for exploration):

```bash
minikube addons enable dashboard
minikube dashboard --url
```

#### Option 2: Managed Kubernetes (production)

Examples: AWS EKS, Google GKE, Azure AKS, etc. In production you typically build and push images to a registry and run with `imagePullPolicy: Always` and pinned tags.

For production hardening, CI/CD, and cloud-provider specifics, see **[Section 7: Production & Advanced Topics](07-Production.md)**.

### Image Preparation

#### Build for Minikube (build “into” the cluster node)

```bash
# Point your Docker CLI at Minikube’s internal Docker daemon
# (this terminal only)
eval $(minikube docker-env)

# IMPORTANT: build from the repository root (build context),
# because the Dockerfile uses COPY paths like
# `ci/docker-compose/configs/...`.
cd /path/to/repo/root
docker build -t blockchain-node:latest -f Dockerfile .

# Restore Docker to your normal local daemon
eval $(minikube docker-env -u)
```

#### Build for a registry (typical production workflow)

```bash
docker build -t blockchain-node:latest -f Dockerfile .
docker tag blockchain-node:latest your-registry/blockchain-node:v1.0.0
docker push your-registry/blockchain-node:v1.0.0
```

### Configuration

#### Secrets (API keys, optional mining address)

Edit `ci/kubernetes/manifests/03-secrets.yaml` to set API keys.

- `BITCOIN_API_ADMIN_KEY`: admin key (protected endpoints)
- `BITCOIN_API_WALLET_KEY`: wallet key (wallet endpoints)
- `MINER_ADDRESS`: **optional**; if omitted/empty, the container entrypoint will auto-create one and persist it in the pod’s wallet volume.

After editing:

```bash
cd ci/kubernetes/manifests
kubectl apply -f 03-secrets.yaml

# Secret values are injected as env vars at pod startup, so restart workloads
kubectl rollout restart statefulset/miner -n blockchain
kubectl rollout restart statefulset/webserver -n blockchain
```

#### ConfigMap (non-secret configuration)

If you need to change non-secret configuration (ports, connect targets, feature toggles), edit `ci/kubernetes/manifests/02-configmap.yaml`.

#### Rate Limiting (Redis + `axum_rate_limiter`)

The webserver includes Redis-backed rate limiting (via `axum_rate_limiter`).

- Rate limiting settings live in `ci/kubernetes/manifests/14-configmap-rate-limit.yaml` (mounted into the webserver pod as `/app/Settings.toml`)
- Redis is deployed in-cluster via `ci/kubernetes/manifests/15-redis.yaml`

After changing rate limit settings:

```bash
cd ci/kubernetes/manifests
kubectl apply -f 14-configmap-rate-limit.yaml
kubectl rollout restart statefulset/webserver -n blockchain
```

**Adjust Resource Limits (Optional):**
Edit manifests to adjust CPU/memory requests and limits.

### Deploy

#### Option 1: Using the script (recommended)

```bash
cd ci/kubernetes/manifests
./deploy.sh
```

Notes:

- The script applies manifests in dependency order.
- If you are upgrading from an older setup where `webserver` was a Deployment, the script deletes `deployment/webserver` so it doesn’t keep spawning pods that accidentally share storage.

#### Option 2: Using Kustomize

```bash
cd ci/kubernetes/manifests
kubectl apply -k .
```

#### Option 3: Manual (explicit order is safer)

Kubernetes does not guarantee ordering when you `kubectl apply -f .`. Prefer the script or Kustomize unless you know what you’re doing.

## Verification

### Quick Snapshot (recommended first)

```bash
kubectl get namespaces
kubectl get pods -n blockchain -o wide
kubectl get svc -n blockchain
kubectl get pvc -n blockchain
```

### Check Pods
```bash
# List / watch pods
kubectl get pods -n blockchain
kubectl get pods -n blockchain -w

# Wait for pods to be ready
kubectl wait --for=condition=ready pod -l app=miner -n \
  blockchain --timeout=300s
kubectl wait --for=condition=ready pod -l app=webserver -n \
  blockchain --timeout=300s
```

### Check Services
```bash
kubectl get svc -n blockchain
```

### Check PersistentVolumeClaims
```bash
kubectl get pvc -n blockchain
```

### Check HPA
```bash
kubectl get hpa -n blockchain
```

### Check Pod Logs
```bash
# Miner logs
kubectl logs -n blockchain -l app=miner --tail=50

# Webserver logs
kubectl logs -n blockchain -l app=webserver --tail=50

# Specific pod
kubectl logs -n blockchain <pod-name> -f
```

### Describe Resources
```bash
# Describe pod
kubectl describe pod -n blockchain <pod-name>

# Describe StatefulSets
kubectl describe statefulset miner -n blockchain
kubectl describe statefulset webserver -n blockchain

# Describe service
kubectl describe svc webserver-service -n blockchain
```

## Accessing Services

### Method 1: LoadBalancer (cloud providers)
```bash
# Get external IP
kubectl get svc webserver-service -n blockchain

# Wait for EXTERNAL-IP, then access
curl http://<EXTERNAL-IP>:8080/api/health/ready
```

### Method 2: NodePort (local/Minikube)
```bash
# Get node IP
minikube ip

# Access via node IP and nodePort
curl http://$(minikube ip):<nodePort>/api/health/ready
```

### Method 3: Port Forward (recommended for development)
```bash
# Forward webserver port
kubectl port-forward -n blockchain svc/webserver-service 8080:8080

# Access locally
curl http://localhost:8080/api/health/ready
```

### Method 4: Minikube Tunnel (for LoadBalancer services on Minikube)
```bash
# Run in separate terminal
minikube tunnel

# Get external IP
kubectl get svc webserver-service -n blockchain

# Access
curl http://127.0.0.1:8080/api/health/ready
```

## Monitoring

### Resource Usage (`kubectl top`)

`kubectl top ...` requires the Kubernetes Metrics API (usually provided by **metrics-server**).

For Minikube, enable it via addon:

```bash
minikube addons enable metrics-server
kubectl get pods -n kube-system | grep metrics-server
kubectl get apiservices | grep metrics
```

Then:

```bash
kubectl top pods -n blockchain
kubectl top nodes
```

### Check HPA Status
```bash
# Describe HPA
kubectl describe hpa webserver-hpa -n blockchain

# Watch HPA
kubectl get hpa -n blockchain -w
```

### View Events
```bash
# Namespace events
kubectl get events -n blockchain --sort-by='.lastTimestamp'

# Pod events
kubectl describe pod <pod-name> -n blockchain | grep Events -A 10
```

## Scaling

### Manual Scaling
```bash
# Scale webserver StatefulSet
kubectl scale statefulset webserver -n blockchain --replicas=5

# Scale miner StatefulSet
kubectl scale statefulset miner -n blockchain --replicas=3

# Verify
kubectl get pods -n blockchain
```

### Automatic Scaling (HPA)
HPA automatically scales based on CPU and memory usage:

```bash
# Check HPA status
kubectl get hpa -n blockchain

# Watch HPA in action
kubectl get hpa -n blockchain -w
```

### Adjust HPA Thresholds
Edit `ci/kubernetes/manifests/10-hpa-webserver.yaml` or `ci/kubernetes/manifests/11-hpa-miner.yaml`:

```yaml
metrics:
- type: Resource
  resource:
    name: cpu
    target:
      type: Utilization
      averageUtilization: 70  # Change threshold here
```

Apply changes:
```bash
# From the repo root:
cd ci/kubernetes/manifests
kubectl apply -f 10-hpa-webserver.yaml
```

## Updates

### Rolling Update (image)
```bash
# Update image
kubectl set image statefulset/webserver \
  blockchain-node=blockchain-node:v1.1.0 -n blockchain

# Check rollout status
kubectl rollout status statefulset/webserver -n blockchain

# View rollout history
kubectl rollout history statefulset/webserver -n blockchain

# Rollback if needed
kubectl rollout undo statefulset/webserver -n blockchain

# Rollback to specific revision
kubectl rollout undo statefulset/webserver -n blockchain \
  --to-revision=2
```

### Update Configuration
```bash
# Update ConfigMap
kubectl edit configmap blockchain-config -n blockchain

# Restart pods to pick up changes
kubectl rollout restart statefulset/webserver -n blockchain
kubectl rollout restart statefulset/miner -n blockchain
```

### Update Secrets
```bash
# Update Secret
kubectl edit secret blockchain-secrets -n blockchain

# Restart pods to pick up changes
kubectl rollout restart statefulset/webserver -n blockchain
kubectl rollout restart statefulset/miner -n blockchain
```

## Common Operations

### View Logs
```bash
# All miner logs
kubectl logs -n blockchain -l app=miner -f

# All webserver logs
kubectl logs -n blockchain -l app=webserver -f

# Specific pod
kubectl logs -n blockchain <pod-name> -f
```

### Execute Commands in Pods
```bash
# Execute command
kubectl exec -n blockchain <pod-name> -- <command>

# Interactive shell
kubectl exec -n blockchain <pod-name> -it -- /bin/sh
```

### Copy Files
```bash
# Copy from pod
kubectl cp blockchain/<pod-name>:/app/data/file.txt ./file.txt

# Copy to pod
kubectl cp ./file.txt blockchain/<pod-name>:/app/data/file.txt
```

## Troubleshooting

### Pods Not Starting
```bash
# Check pod status
kubectl get pods -n blockchain

# Describe pod for details
kubectl describe pod <pod-name> -n blockchain

# Check logs
kubectl logs <pod-name> -n blockchain
```

**Common issues:**
- **ImagePullBackOff**: Image not found or registry access denied
- **CrashLoopBackOff**: Container crashing
- **Pending**: Resource constraints or PVC not bound

### Services Not Accessible
```bash
# Check service endpoints
kubectl get endpoints -n blockchain

# Test connectivity
kubectl exec -n blockchain <pod-name> -- \
  curl http://webserver-service:8080/api/health/ready
```

### HPA Not Scaling
```bash
# Check HPA status
kubectl describe hpa webserver-hpa -n blockchain

# Check resource usage
kubectl top pods -n blockchain

# Verify metrics server
kubectl get apiservices | grep metrics
```

### PersistentVolume Issues
```bash
# Check PVC status
kubectl get pvc -n blockchain

# Describe PVC
kubectl describe pvc miner-data-pvc -n blockchain

# Check PVs
kubectl get pv
```

## Cleanup

### Delete All Resources
```bash
# From the repo root:
cd ci/kubernetes/manifests

# Undeploy using the script (recommended)
./undeploy.sh

# Or delete namespace (deletes everything)
kubectl delete namespace blockchain

# Stop the local cluster (keeps cluster on disk)
minikube stop

# Or delete it completely
minikube delete
```

### Delete Specific Resources
```bash
# Delete StatefulSets
kubectl delete statefulset miner -n blockchain
kubectl delete statefulset webserver -n blockchain

# Delete service
kubectl delete svc webserver-service -n blockchain

# Delete PVC (data will be lost!)
kubectl delete pvc -n blockchain miner-data-miner-0
```

For deeper troubleshooting patterns, see [Section 7: Production & Advanced Topics](07-Production.md).

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Kubernetes Manifests](04-Manifests.md) | [↑ Table of Contents](#) | [Next Section: Autoscaling →](06-Autoscaling.md) |
|:---:|:---:|:---:|
| *Section 4* | *Current Section* | *Section 6* |

</div>
