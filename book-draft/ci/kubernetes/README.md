<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../../README.md)
2. [Chapter 2: Transaction ID Format](../../bitcoin-blockchain/primitives/02-Transaction-ID-Format.md)
3. [Chapter 3: Web API Architecture](../../bitcoin-blockchain/web/README.md)
4. [Chapter 4: Desktop Admin Interface](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md)
5. [Chapter 5: Wallet User Interface](../../bitcoin-wallet-ui/04-Wallet-UI.md)
6. [Chapter 6: Embedded Database & Persistence](../../bitcoin-wallet-ui/05-Embedded-Database.md)
7. [Chapter 7: Web Admin Interface](../../bitcoin-web-ui/06-Web-Admin-UI.md)

### Part II: Deployment & Operations

8. [Chapter 8: Docker Compose Deployment](../docker-compose/01-Introduction.md)
9. **Chapter 9: Kubernetes Deployment** ← *You are here*
   - **Section 1: Introduction & Quick Start** ← *You are here*
   - [Section 2: Architecture & Core Concepts](02-Architecture.md)
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

# Chapter 9: Kubernetes Deployment

**Part II: Deployment & Operations**

<div align="center">

**📚 [← Chapter 8: Docker Compose](../docker-compose/01-Introduction.md)** | **Chapter 9: Kubernetes** | **[End of Book →](#)** 📚

</div>

---

In this final chapter, we'll learn how to deploy and manage our blockchain network on Kubernetes. This is where we transform our development system into a production-ready, scalable blockchain network that can handle real-world workloads.

## Table of Contents

1. [Section 1: Introduction & Quick Start](#section-1-introduction--quick-start)
2. [Section 2: Architecture & Core Concepts](02-Architecture.md)
3. [Section 3: Migration Guide](03-Migration.md)
4. [Section 4: Kubernetes Manifests](04-Manifests.md)
5. [Section 5: Deployment & Operations](05-Deployment.md)
6. [Section 6: Autoscaling](06-Autoscaling.md)
7. [Section 7: Production & Advanced Topics](07-Production.md)

---

## Section 1: Introduction & Quick Start

### Overview

In this chapter, we'll explore how to deploy the blockchain network on Kubernetes. Our setup will support automatic scaling, high availability, rolling updates, and production-grade orchestration. By the end of this chapter, you'll understand how to take a blockchain network from development to production using industry-standard container orchestration.

### Why Kubernetes for Production

When we move from development to production, we need more than what Docker Compose can provide. Let's explore why Kubernetes is the right choice for production deployments:

| Feature | Docker Compose | Kubernetes |
|---------|---------------|-------------|
| **Autoscaling** | Manual or custom scripts | ✅ Native HPA (Horizontal Pod Autoscaler) |
| **Service Discovery** | Docker DNS | ✅ Native DNS-based service discovery |
| **Load Balancing** | Manual setup | ✅ Built-in load balancing |
| **Rolling Updates** | Manual | ✅ Zero-downtime rolling updates |
| **Self-Healing** | Limited | ✅ Automatic restart and replacement |
| **Resource Management** | Basic | ✅ CPU/Memory limits and requests |
| **Multi-Node** | Single host | ✅ Multi-node cluster support |
| **Production Ready** | Development/Testing | ✅ Enterprise-grade orchestration |

### Quick Start

#### Prerequisites Check

```bash
# Check kubectl
kubectl version --client

# Check cluster connection
kubectl cluster-info

# Check nodes
kubectl get nodes
```

#### Quick Deploy (5 Minutes)

**Step 1: Build Image (Minikube)**

```bash
# Start Minikube
minikube start --cpus=4 --memory=8192

# Use Minikube's Docker
eval $(minikube docker-env)

# Build image
cd ../docker-compose/configs
docker build -t blockchain-node:latest .
cd ../../kubernetes/manifests
```

**Step 2: Update Secrets (Optional)**

Edit `03-secrets.yaml` to set your API keys, or use defaults for testing.

**Step 3: Deploy**

```bash
cd manifests
./deploy.sh
```

**Step 4: Verify**

```bash
# Check pods are running
kubectl get pods -n blockchain

# Wait for all pods to be ready
kubectl wait --for=condition=ready pod -l app=miner -n blockchain --timeout=300s
kubectl wait --for=condition=ready pod -l app=webserver -n blockchain --timeout=300s
```

**Step 5: Access**

```bash
# Port forward
kubectl port-forward -n blockchain svc/webserver-service 8080:8080
```

Then open http://localhost:8080 in your browser.

### Common Operations

#### Scale Manually

```bash
# Scale webservers to 5
kubectl scale deployment webserver -n blockchain --replicas=5

# Scale miners to 3 (miner is a StatefulSet)
kubectl scale statefulset miner -n blockchain --replicas=3
```

#### View Logs

```bash
# All miner logs
kubectl logs -n blockchain -l app=miner -f

# All webserver logs
kubectl logs -n blockchain -l app=webserver -f

# Specific pod
kubectl logs -n blockchain <pod-name> -f
```

#### Update Configuration

```bash
# Edit ConfigMap
kubectl edit configmap blockchain-config -n blockchain

# Restart pods to pick up changes
kubectl rollout restart deployment/webserver -n blockchain
kubectl rollout restart statefulset/miner -n blockchain
```

#### Update Image

```bash
# Set new image
kubectl set image deployment/webserver blockchain-node=blockchain-node:v1.1.0 -n blockchain

# Check rollout
kubectl rollout status deployment/webserver -n blockchain

# Rollback if needed
kubectl rollout undo deployment/webserver -n blockchain
```

### Cleanup

```bash
cd manifests
./undeploy.sh

# Or delete everything
kubectl delete namespace blockchain
```

### Troubleshooting

#### Pods Not Starting

```bash
# Check pod status
kubectl get pods -n blockchain

# Describe pod for details
kubectl describe pod <pod-name> -n blockchain

# Check logs
kubectl logs <pod-name> -n blockchain
```

#### Services Not Accessible

```bash
# Check service endpoints
kubectl get endpoints -n blockchain

# Test from pod
kubectl exec -n blockchain <pod-name> -- curl http://webserver-service:8080/api/health/ready
```

#### HPA Not Working

```bash
# Check HPA status
kubectl describe hpa webserver-hpa -n blockchain

# Check resource usage
kubectl top pods -n blockchain
```

---

## Section 2: Architecture & Core Concepts

See [02-Architecture.md](02-Architecture.md) for detailed information about:
- Kubernetes architecture overview
- Core concepts (Namespaces, ConfigMaps, Secrets, Deployments, StatefulSets, Services)
- Differences from Docker Compose
- Service discovery and networking

---

## Section 3: Migration Guide

See [03-Migration.md](03-Migration.md) for detailed information about:
- Step-by-step migration from Docker Compose
- Image preparation and registry setup
- Creating Kubernetes resources
- Migration checklist
- Testing and verification

---

## Chapter 4: Kubernetes Manifests

See [04-Manifests.md](04-Manifests.md) for detailed information about:
- Complete manifest examples with explanations
- Namespace configuration
- ConfigMaps and Secrets
- PersistentVolumeClaims
- StatefulSets and Deployments
- Services and networking
- HPA configuration

---

## Section 5: Deployment & Operations

See [05-Deployment.md](05-Deployment.md) for detailed information about:
- Step-by-step deployment process
- Verification and monitoring
- Accessing services
- Scaling operations
- Updates and rollbacks
- Day-to-day operations

---

## Section 6: Autoscaling

See [06-Autoscaling.md](06-Autoscaling.md) for detailed information about:
- HPA (Horizontal Pod Autoscaler) configuration
- Scaling policies and thresholds
- Metrics and monitoring
- Best practices
- Troubleshooting autoscaling

---

## Section 7: Production & Advanced Topics

See [07-Production.md](07-Production.md) for detailed information about:
- Production considerations
- Security best practices
- Monitoring and alerting
- Backup strategies
- Advanced networking
- Troubleshooting guide

---

---

<div align="center">

**Local Navigation - Table of Contents**

| [← First Section: Introduction & Quick Start](#chapter-1-introduction--quick-start) | [↑ Table of Contents](#table-of-contents) | [Next Section: Architecture & Core Concepts →](02-Architecture.md) |
|:---:|:---:|:---:|
| *Start of Chapter 8* | *Current Section* | *Section 2* |

</div>

---

## Additional Resources

- [Kubernetes Official Documentation](https://kubernetes.io/docs/)
- [Kubernetes HPA Documentation](https://kubernetes.io/docs/tasks/run-application/horizontal-pod-autoscale/)
- [StatefulSets Documentation](https://kubernetes.io/docs/concepts/workloads/controllers/statefulset/)
