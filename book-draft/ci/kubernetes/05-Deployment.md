<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../../README.md)
2. [Chapter 2: Transaction System](../../bitcoin-blockchain/02-Transaction-System.md)
3. [Chapter 4: Desktop Admin Interface](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)
4. [Chapter 5: Wallet User Interface](../../bitcoin-wallet-ui/04-Wallet-UI.md)
5. [Chapter 6: Embedded Database & Persistence](../../bitcoin-wallet-ui/05-Embedded-Database.md)
6. [Chapter 7: Web Admin Interface](../../bitcoin-web-ui/06-Web-Admin-UI.md)

### Part II: Deployment & Operations

8. [Chapter 8: Docker Compose Deployment](../docker-compose/01-Introduction.md)
9. [Chapter 9: Kubernetes Deployment](README.md) - Main chapter
   - [Section 1: Introduction & Quick Start](README.md)
   - [Section 2: Architecture](02-Architecture.md)
   - [Section 3: Migration Guide](03-Migration.md)
   - [Section 4: Kubernetes Manifests](04-Manifests.md)
   - **Section 5: Deployment & Operations** ← *You are here*
   - [Section 6: Autoscaling](06-Autoscaling.md)
   - [Section 7: Production & Advanced Topics](07-Production.md)

</details>

</div>

<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

# Chapter 8, Section 5: Deployment & Operations

**Part II: Deployment & Operations** | **Chapter 9: Kubernetes Deployment**

<div align="center">

**📚 [← Chapter 8: Docker Compose](../docker-compose/01-Introduction.md)** | **Chapter 9: Kubernetes** | **[End of Book →](#)** 📚

</div>

---

This section covers step-by-step deployment, verification, monitoring, scaling, and day-to-day operations.

## Deployment Process

### Prerequisites

**Required Tools:**
```bash
# Install kubectl
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/darwin/amd64/kubectl"
chmod +x kubectl
sudo mv kubectl /usr/local/bin/

# Verify installation
kubectl version --client
```

**Cluster Setup:**

**Option 1: Minikube (Local Development)**
```bash
minikube start --cpus=4 --memory=8192
minikube addons enable ingress
eval $(minikube docker-env)
```

**Option 2: Cloud Provider**
- AWS EKS, Google GKE, Azure AKS, etc.

### Image Preparation

**For Minikube:**
```bash
eval $(minikube docker-env)
cd ../docker-compose/configs
docker build -t blockchain-node:latest .
```

**For Cloud/Registry:**
```bash
docker build -t blockchain-node:latest .
docker tag blockchain-node:latest your-registry/blockchain-node:v1.0.0
docker push your-registry/blockchain-node:v1.0.0
```

### Configuration

**Update Secrets:**
Edit `manifests/03-secrets.yaml`:
```yaml
stringData:
  BITCOIN_API_ADMIN_KEY: "your-secure-admin-key"
  BITCOIN_API_WALLET_KEY: "your-secure-wallet-key"
  MINER_ADDRESS: "your-wallet-address-here"  # REQUIRED: Must be set to a valid wallet address
```

**Update ConfigMap (Optional):**
Edit `manifests/02-configmap.yaml` if needed.

**Adjust Resource Limits (Optional):**
Edit manifests to adjust CPU/memory requests and limits.

### Deploy

**Option 1: Using Script (Recommended)**
```bash
cd manifests
./deploy.sh
```

**Option 2: Using Kustomize**
```bash
cd manifests
kubectl apply -k .
```

**Option 3: Manual**
```bash
cd manifests
kubectl apply -f .
```

## Verification

### Check Namespace
```bash
kubectl get ns blockchain
```

### Check Pods
```bash
# List all pods
kubectl get pods -n blockchain

# Watch pods
kubectl get pods -n blockchain -w

# Wait for pods to be ready
kubectl wait --for=condition=ready pod -l app=miner -n blockchain --timeout=300s
kubectl wait --for=condition=ready pod -l app=webserver -n blockchain --timeout=300s
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

# Describe StatefulSet/Deployment
kubectl describe statefulset miner -n blockchain
kubectl describe deployment webserver -n blockchain

# Describe service
kubectl describe svc webserver-service -n blockchain
```

## Accessing Services

### Method 1: LoadBalancer (Cloud)
```bash
# Get external IP
kubectl get svc webserver-service -n blockchain

# Wait for EXTERNAL-IP, then access
curl http://<EXTERNAL-IP>:8080/api/health/ready
```

### Method 2: NodePort (Local/Minikube)
```bash
# Get node IP
minikube ip

# Access via node IP and nodePort
curl http://$(minikube ip):<nodePort>/api/health/ready
```

### Method 3: Port Forward (Development)
```bash
# Forward webserver port
kubectl port-forward -n blockchain svc/webserver-service 8080:8080

# Access locally
curl http://localhost:8080/api/health/ready
```

### Method 4: Minikube Tunnel
```bash
# Run in separate terminal
minikube tunnel

# Get external IP
kubectl get svc webserver-service -n blockchain

# Access
curl http://127.0.0.1:8080/api/health/ready
```

## Monitoring

### Check Resource Usage
```bash
# Install metrics server (if not installed)
kubectl apply -f https://github.com/kubernetes-sigs/metrics-server/releases/latest/download/components.yaml

# Check pod resource usage
kubectl top pods -n blockchain

# Check node resource usage
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
# Scale webserver deployment
kubectl scale deployment webserver -n blockchain --replicas=5

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
Edit `manifests/10-hpa-webserver.yaml` or `11-hpa-miner.yaml`:

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
kubectl apply -f manifests/10-hpa-webserver.yaml
```

## Updates

### Rolling Update
```bash
# Update image
kubectl set image deployment/webserver blockchain-node=blockchain-node:v1.1.0 -n blockchain

# Check rollout status
kubectl rollout status deployment/webserver -n blockchain

# View rollout history
kubectl rollout history deployment/webserver -n blockchain

# Rollback if needed
kubectl rollout undo deployment/webserver -n blockchain

# Rollback to specific revision
kubectl rollout undo deployment/webserver -n blockchain --to-revision=2
```

### Update Configuration
```bash
# Update ConfigMap
kubectl edit configmap blockchain-config -n blockchain

# Restart pods to pick up changes
kubectl rollout restart deployment/webserver -n blockchain
kubectl rollout restart statefulset/miner -n blockchain
```

### Update Secrets
```bash
# Update Secret
kubectl edit secret blockchain-secrets -n blockchain

# Restart pods to pick up changes
kubectl rollout restart deployment/webserver -n blockchain
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
kubectl exec -n blockchain <pod-name> -- curl http://webserver-service:8080/api/health/ready
```

### HPA Not Scaling
```bash
# Check HPA status
kubectl describe hpa webserver-hpa -n blockchain

# Check resource usage
kubectl top pods -n blockchain

# Verify metrics server
kubectl get apiservice v1beta1.metrics.k8s.io
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
# Delete all resources in namespace
kubectl delete -f manifests/

# Or delete namespace (deletes everything)
kubectl delete namespace blockchain
```

### Delete Specific Resources
```bash
# Delete StatefulSet/Deployment
kubectl delete statefulset miner -n blockchain

# Delete service

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Kubernetes Manifests](04-Manifests.md) | [↑ Table of Contents](#) | [Next Section: Autoscaling →](06-Autoscaling.md) |
|:---:|:---:|:---:|
| *Section 4* | *Current Section* | *Section 6* |

</div>

---
kubectl delete svc webserver-service -n blockchain

# Delete PVC (data will be lost!)
kubectl delete pvc miner-data-pvc -n blockchain
```

For more detailed troubleshooting, see [Chapter 7: Production & Advanced Topics](07-Production.md).
