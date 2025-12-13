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
   - [Section 3: Migration Guide](03-Migration.md)
   - [Section 4: Kubernetes Manifests](04-Manifests.md)
   - [Section 5: Deployment & Operations](05-Deployment.md)
   - [Section 6: Autoscaling](06-Autoscaling.md)
   - **Section 7: Production & Advanced Topics** ← *You are here*

</details>

</div>

<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

# Chapter 8, Section 7: Production & Advanced Topics

**Part II: Deployment & Operations** | **Chapter 9: Kubernetes Deployment**

<div align="center">

**📚 [← Chapter 8: Docker Compose](../docker-compose/01-Introduction.md)** | **Chapter 9: Kubernetes** | **[End of Book →](#)** 📚

</div>

---

This section covers production considerations, advanced topics, and troubleshooting.

## Production Considerations

### 1. Use Specific Image Tags

Replace `latest` with specific version tags:

```yaml
image: your-registry/blockchain-node:v1.0.0
imagePullPolicy: Always
```

**Benefits:**
- Reproducible deployments
- Easy rollback to known versions
- Better security (no unexpected updates)

### 2. Enable Resource Quotas

```yaml
apiVersion: v1
kind: ResourceQuota
metadata:
  name: blockchain-quota
  namespace: blockchain
spec:
  hard:
    requests.cpu: "10"
    requests.memory: 20Gi
    limits.cpu: "20"
    limits.memory: 40Gi
```

### 3. Configure Network Policies

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: blockchain-netpol
  namespace: blockchain
spec:
  podSelector:
    matchLabels:
      component: blockchain-node
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: blockchain
    ports:
    - protocol: TCP
      port: 2001
```

### 4. Set Up Monitoring

Use Prometheus and Grafana:

```yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: blockchain-monitor
  namespace: blockchain
spec:
  selector:
    matchLabels:
      app: webserver
  endpoints:
  - port: web
    path: /metrics
```

### 5. Configure Logging

Use centralized logging (ELK, Loki):

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: fluentd-config
data:
  fluent.conf: |
    <source>
      @type tail
      path /var/log/containers/*blockchain*.log
      tag kubernetes.*
    </source>
```

### 6. Set Up Backup Strategy

Use Velero or similar for cluster backups:

```yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: blockchain-backup
  namespace: blockchain
spec:
  schedule: "0 2 * * *"  # Daily at 2 AM
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: backup
            image: your-registry/backup-tool:latest
            command:
            - /bin/sh
            - -c
            - |
              kubectl exec miner-0 -n blockchain -- tar czf /tmp/backup.tar.gz /app/data
              kubectl cp blockchain/miner-0:/tmp/backup.tar.gz /backups/miner-$(date +%Y%m%d).tar.gz
          restartPolicy: OnFailure
```

### 7. Configure PodDisruptionBudget

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

### 8. Use External Secret Management

For production, use:
- **Sealed Secrets**: Encrypt secrets for Git storage
- **AWS Secrets Manager**: Store secrets in AWS
- **HashiCorp Vault**: External secret management
- **External Secrets Operator**: Sync secrets from external systems

## Production Checklist

Before deploying to production:

- [ ] Use specific image tags (not `latest`)
- [ ] Set `imagePullPolicy: Always`
- [ ] Configure proper resource limits
- [ ] Set up storage classes
- [ ] Configure network policies
- [ ] Set up monitoring (Prometheus/Grafana)
- [ ] Configure logging (ELK/Loki)
- [ ] Set up backup strategy (Velero)
- [ ] Configure RBAC for access control
- [ ] Use external secret management
- [ ] Set up alerting
- [ ] Configure resource quotas
- [ ] Test disaster recovery
- [ ] Document runbooks

## Advanced Topics

### StatefulSet Implementation: Chain Topology for Miners

**Current Implementation**: We use **StatefulSet with Headless Service** for miners to enable chain topology.

**How It Works:**

1. **StatefulSet** creates pods with stable names: `miner-0`, `miner-1`, `miner-2`
2. **Headless Service** provides DNS per pod: `miner-0.miner-headless.blockchain.svc.cluster.local`
3. **Entrypoint Script** detects StatefulSet pod name and auto-configures connections:
   - `miner-0`: Connects to `local` (seed node)
   - `miner-1`: Connects to `miner-0.miner-headless.blockchain.svc.cluster.local:2001`
   - `miner-2`: Connects to `miner-1.miner-headless.blockchain.svc.cluster.local:2001`

**Benefits:**
- ✅ **Chain Topology**: Miners connect sequentially
- ✅ **Stable Names**: Pod names persist across restarts
- ✅ **Ordered Startup**: Pods start sequentially (0, then 1, then 2)
- ✅ **Isolated Storage**: Each pod gets its own PVC automatically
- ✅ **Direct Connections**: Pod-to-pod connections via headless service DNS

**Comparison: Deployment vs StatefulSet**

| Aspect | Deployment | StatefulSet |
|--------|-----------|-------------|
| **Pod Names** | Random: `miner-7d8f9c4b5-x2k9p` | Stable: `miner-0`, `miner-1`, `miner-2` |
| **Startup** | Parallel (all at once) | Sequential (0, then 1, then 2) |
| **Connection** | All connect to service (load balanced) | Chain: miner-1 → miner-0, miner-2 → miner-1 |
| **Storage** | Shared PVCs (manual) | Automatic per-pod PVCs |
| **DNS** | Service DNS only | Direct pod DNS via headless service |

### Using Downward API

Downward API allows pods to access information about themselves:

```yaml
env:
- name: POD_NAME
  valueFrom:
    fieldRef:
      fieldPath: metadata.name
- name: POD_NAMESPACE
  valueFrom:
    fieldRef:
      fieldPath: metadata.namespace
- name: POD_IP
  valueFrom:
    fieldRef:
      fieldPath: status.podIP
```

**What Happens:**
- Kubernetes sets `metadata.name = "miner-0"`
- Downward API injects `POD_NAME = "miner-0"` as environment variable
- Container can use `POD_NAME` for configuration

### Service Discovery & Networking

**How Pods Connect:**

In Kubernetes, pods connect using **Service DNS names**:
```
<service-name>.<namespace>.svc.cluster.local
```

**Example: Webserver Connecting to Miner**

**Docker Compose:**
```bash
NODE_CONNECT_NODES=miner_1:2001
```

**Kubernetes:**
```bash
NODE_CONNECT_NODES=miner-service.blockchain.svc.cluster.local:2001
```

**Connection Pattern:**
```
Webserver Pod 1 → miner-service.blockchain.svc.cluster.local:2001
Webserver Pod 2 → miner-service.blockchain.svc.cluster.local:2001
Webserver Pod 3 → miner-service.blockchain.svc.cluster.local:2001
```

All webservers sync their blockchain copies from miners via the load-balanced service.

### Persistent Storage

**Storage Classes:**

```yaml
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: fast-ssd
provisioner: kubernetes.io/aws-ebs
parameters:
  type: gp3
  fsType: ext4
```

**Volume Claims with Storage Classes:**

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: miner-data-pvc
spec:
  storageClassName: fast-ssd
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 50Gi
```

### Init Containers: Detailed Flow

**Why Init Containers Run First:**

```
Pod Created
    ↓
Init Container Phase:
  - Init Container 1 starts
  - Must exit successfully (code 0)
  - Init Container 2 starts (if multiple)
  - Must exit successfully
    ↓
Main Container Phase:
  - Main containers can start
```

**Our Use Case:**

```yaml
initContainers:
- name: wait-for-miner
  command: ['sh', '-c', 'until nc -z miner-service.blockchain.svc.cluster.local 2001; do sleep 2; done;']
```

**Execution Flow:**

```
Init Container Starts
    ↓
Loop:
  1. Try: nc -z miner-service.blockchain.svc.cluster.local:2001
  2. If succeeds:
     - Exit code 0
     - Init container completes
     - Main container can start
  3. If fails:
     - Sleep 2 seconds
     - Retry
    ↓
Maximum wait: ~2 minutes (60 attempts × 2 seconds)
```

### Resource Requests and Limits

**Requests:**

```yaml
resources:
  requests:
    cpu: "500m"
    memory: "512Mi"
```

**What Requests Do:**
- **Scheduling**: Scheduler uses requests to find nodes with capacity
- **Guarantee**: Kubernetes guarantees at least this much
- **Quota**: Counts against namespace resource quota

**Limits:**

```yaml
resources:
  limits:
    cpu: "2000m"
    memory: "2Gi"
```

**What Limits Do:**
- **Maximum**: Container cannot exceed this
- **Throttling**: CPU throttled if exceeds limit
- **Killing**: OOMKilled if memory exceeds limit

**Why Set Both:**
- **Requests**: Ensure pod can be scheduled
- **Limits**: Prevent resource exhaustion
- **Efficiency**: Better resource utilization

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

**Common Issues:**
- **ImagePullBackOff**: Image not found or registry access denied
  - Solution: Check image name and registry credentials
- **CrashLoopBackOff**: Container crashing
  - Solution: Check logs: `kubectl logs <pod-name> -n blockchain`
- **Pending**: Resource constraints or PVC not bound
  - Solution: Check resources and PVC status

**Check StatefulSet:**

```bash
kubectl describe statefulset miner -n blockchain
```

**What to Look For:**

```
Events:
  - ScalingReplicaSet: Scaled up replica set miner-xxx to 2
    ↓
Check ReplicaSet:
  kubectl describe replicaset miner-xxx -n blockchain
    ↓
Events:
  - SuccessfulCreate: Created pod miner-xxx-abc
    ↓
Check Pod:
  kubectl describe pod miner-xxx-abc -n blockchain
    ↓
Events:
  - Scheduled: Successfully assigned to node-1
  - Pulling: Pulling image blockchain-node:latest
  - Pulled: Successfully pulled image
  - Created: Created container blockchain-node
  - Started: Started container blockchain-node
```

### Services Not Accessible

```bash
# Check service endpoints
kubectl get endpoints -n blockchain

# Check service
kubectl describe svc webserver-service -n blockchain

# Test connectivity from pod
kubectl exec -n blockchain <pod-name> -- curl http://webserver-service:8080/api/health/ready
```

**Check Endpoints:**

```bash
kubectl get endpoints miner-service -n blockchain
```

**Expected:**
```
NAME            ENDPOINTS
miner-service   10.244.1.5:2001,10.244.2.3:2001
```

**If Empty:**
- Pods not ready (check readiness probe)
- Selector mismatch (check pod labels)
- Port mismatch (check service targetPort)

### HPA Not Scaling

```bash
# Check HPA status
kubectl describe hpa webserver-hpa -n blockchain

# Check resource usage
kubectl top pods -n blockchain

# Check metrics server
kubectl get apiservice v1beta1.metrics.k8s.io
```

**What to Look For:**

```
Metrics:
  resource cpu on pods: 85% / 70%  # Current / Target
    ↓
Desired Replicas: 3
Current Replicas: 2
    ↓
HPA will scale Deployment to 3 replicas
```

**If Not Scaling:**
- Metrics Server not installed
- Pods not reporting metrics
- HPA target not set correctly

### PersistentVolume Issues

```bash
# Check PVC status
kubectl get pvc -n blockchain

# Describe PVC
kubectl describe pvc miner-data-pvc -n blockchain

# Check PVs
kubectl get pv
```

**Common Issues:**
- **Pending**: Storage class not found or insufficient storage
  - Solution: Create storage class or check available storage

### Network Issues

```bash
# Test DNS resolution
kubectl exec -n blockchain <pod-name> -- nslookup miner-service.blockchain.svc.cluster.local

# Test connectivity
kubectl exec -n blockchain <pod-name> -- nc -zv miner-service.blockchain.svc.cluster.local 2001
```

## Summary

This section covered:
- Production considerations and best practices
- Advanced topics (StatefulSets, service discovery, storage)
- Troubleshooting common issues

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Autoscaling](06-Autoscaling.md) | [↑ Table of Contents](#) | [Last Section: Production & Advanced Topics →](#production-considerations) |
|:---:|:---:|:---:|
| *Section 6* | *Current Section* | *End of Chapter 8* |

</div>

---

**Key Takeaways:**
- Use specific image tags and resource limits in production
- Set up monitoring, logging, and backups
- Understand StatefulSet vs Deployment differences
- Use Downward API for pod self-awareness
- Configure proper resource requests and limits

---

<div align="center">

**📚 [← Section 6: Autoscaling](06-Autoscaling.md)** | **Section 7: Production & Advanced Topics** | **[End of Book →](#)** 📚

</div>

---

*This chapter has explored production-grade Kubernetes deployment for the blockchain network. We've examined architecture and core concepts, migration from Docker Compose, Kubernetes manifests, deployment and operations, autoscaling capabilities, and production considerations with advanced topics. Kubernetes provides industry-standard container orchestration with automatic scaling, high availability, rolling updates, and production-grade operational capabilities. The deployment system transforms our development blockchain network into a scalable, resilient, production-ready system. In the next chapter, we'll explore [Chapter 10: Rust Language Guide](../../rust/README.md) to understand the comprehensive Rust language features used throughout our blockchain implementation.*

For more detailed information, see the complete guide sections on each topic.
