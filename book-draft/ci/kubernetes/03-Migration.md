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
   - [Section 2: Architecture](02-Architecture.md)
   - **Section 3: Migration Guide** ← *You are here*
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

# Chapter 8, Section 3: Migration Guide

**Part II: Deployment & Operations** | **Chapter 9: Kubernetes Deployment**

<div align="center">

**📚 [← Chapter 8: Docker Compose](../docker-compose/01-Introduction.md)** | **Chapter 9: Kubernetes** | **[End of Book →](#)** 📚

</div>

---

This section provides a step-by-step guide for migrating from Docker Compose to Kubernetes.

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
cd ci/docker-compose/configs
docker build -t blockchain-node:latest .

# Tag for registry (replace with your registry)
docker tag blockchain-node:latest your-registry/blockchain-node:v1.0.0

# Push to registry
docker push your-registry/blockchain-node:v1.0.0
```

### For Minikube (Local Development)

```bash
# Use Minikube's Docker daemon
eval $(minikube docker-env)

# Build directly in Minikube
docker build -t blockchain-node:latest .
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

Create `02-configmap.yaml`:

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: blockchain-config
  namespace: blockchain
data:
  MINER_NODE_IS_MINER: "yes"
  MINER_NODE_IS_WEB_SERVER: "no"
  MINER_NODE_CONNECT_NODES: "local"
  WEBSERVER_NODE_IS_MINER: "no"
  WEBSERVER_NODE_IS_WEB_SERVER: "yes"
  WEBSERVER_NODE_CONNECT_NODES: "miner-service.blockchain.svc.cluster.local:2001"
  SEQUENTIAL_STARTUP: "no"
  WALLET_FILE: "wallets/wallets.dat"
```

Apply:
```bash
kubectl apply -f 02-configmap.yaml
```

### Secrets

Create `03-secrets.yaml`:

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
  MINER_ADDRESS: "your-wallet-address-here"  # REQUIRED: Must be set to a valid wallet address
```

Apply:
```bash
kubectl apply -f 03-secrets.yaml
```

## Step 4: Create PersistentVolumeClaims

### Miner PVC

Create `04-pvc-miner.yaml`:

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

### Webserver PVC

Create `05-pvc-webserver.yaml`:

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: webserver-data-pvc
  namespace: blockchain
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 50Gi
```

Apply:
```bash
kubectl apply -f 04-pvc-miner.yaml
kubectl apply -f 05-pvc-webserver.yaml
```

## Step 5: Create Deployments

### StatefulSet for Miners

Create `06-statefulset-miner.yaml` (see [Chapter 4: Kubernetes Manifests](04-Manifests.md) for complete example).

### Deployment for Webservers

Create `07-deployment-webserver.yaml` (see [Chapter 4: Kubernetes Manifests](04-Manifests.md) for complete example).

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

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Architecture & Core Concepts](02-Architecture.md) | [↑ Table of Contents](#) | [Next Section: Kubernetes Manifests →](04-Manifests.md) |
|:---:|:---:|:---:|
| *Section 2* | *Current Section* | *Section 4* |

</div>

---

# Check Secrets
kubectl get secret blockchain-secrets -n blockchain -o yaml
```

For more detailed troubleshooting, see [Chapter 7: Production & Advanced Topics](07-Production.md).
