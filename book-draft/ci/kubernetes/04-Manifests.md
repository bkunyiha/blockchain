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
   - **Section 4: Kubernetes Manifests** ← *You are here*
   - [Section 5: Deployment & Operations](05-Deployment.md)
   - [Section 6: Autoscaling](06-Autoscaling.md)
   - [Section 7: Production & Advanced Topics](07-Production.md)

</details>

</div>

<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

# Chapter 8, Section 4: Kubernetes Manifests

**Part II: Deployment & Operations** | **Chapter 9: Kubernetes Deployment**

<div align="center">

**📚 [← Chapter 8: Docker Compose](../docker-compose/01-Introduction.md)** | **Chapter 9: Kubernetes** | **[End of Book →](#)** 📚

</div>

---

This section provides complete manifest examples with detailed explanations. Each manifest file is numbered in deployment order.

## Overview of Manifest Files

| File | Resource Type | Purpose | Key Configurations |
|------|--------------|---------|-------------------|
| `01-namespace.yaml` | Namespace | Creates isolated environment | Namespace name, labels |
| `02-configmap.yaml` | ConfigMap | Stores non-sensitive configuration | Node settings, connection strings |
| `03-secrets.yaml` | Secret | Stores sensitive data | API keys, base64 encoding |
| `04-pvc-miner.yaml` | PersistentVolumeClaim | Storage for miner data | Storage size (50Gi), access mode |
| `05-pvc-webserver.yaml` | PersistentVolumeClaim | Storage for webserver data | Storage size (50Gi), access mode |
| `06-statefulset-miner.yaml` | StatefulSet | Defines miner pods | Replicas, volumeClaimTemplates, headless service |
| `07-deployment-webserver.yaml` | Deployment | Defines webserver pods | Replicas, init containers, health checks |
| `08-service-miner-headless.yaml` | Service | Headless service for pod DNS | clusterIP: None, stable DNS per pod |
| `08-service-miner.yaml` | Service | Internal network endpoint | ClusterIP, session affinity |
| `09-service-webserver.yaml` | Service | External network endpoint | LoadBalancer, port mappings |
| `10-hpa-webserver.yaml` | HorizontalPodAutoscaler | Auto-scales webservers | CPU/Memory thresholds, scaling policies |
| `11-hpa-miner.yaml` | HorizontalPodAutoscaler | Auto-scales miners | CPU threshold, conservative scaling |
| `12-pod-disruption-budget.yaml` | PodDisruptionBudget | Ensures minimum availability | Minimum available pods |

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
  MINER_NODE_IS_MINER: "yes"
  MINER_NODE_IS_WEB_SERVER: "no"
  MINER_NODE_CONNECT_NODES: "local"
  WEBSERVER_NODE_IS_MINER: "no"
  WEBSERVER_NODE_IS_WEB_SERVER: "yes"
  WEBSERVER_NODE_CONNECT_NODES: "miner-service.blockchain.svc.cluster.local:2001"
  SEQUENTIAL_STARTUP: "no"
  WALLET_FILE: "wallets/wallets.dat"
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
  MINER_ADDRESS: "your-wallet-address-here"  # REQUIRED: Must be set to a valid wallet address
```

**Key Points:**
- `stringData` is plain text (Kubernetes encodes automatically)
- For production, use sealed-secrets or external secret management

## 4. PersistentVolumeClaim (Miner)

**File**: `04-pvc-miner.yaml`

Provides persistent storage for miner blockchain data.

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

## 5. PersistentVolumeClaim (Webserver)

**File**: `05-pvc-webserver.yaml`

Provides persistent storage for webserver blockchain data.

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

## 7. Deployment (Webserver)

**File**: `07-deployment-webserver.yaml`

Defines webserver pods with rolling updates.

**Key Features:**
- Stateless pods (can be replaced)
- Rolling updates and rollbacks
- Init containers for dependency waiting
- Health checks (liveness and readiness probes)

**Important Configurations:**
```yaml
apiVersion: apps/v1
kind: Deployment
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
            path: /api/health/liveness
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
      volumes:
      - name: webserver-data
        persistentVolumeClaim:
          claimName: webserver-data-pvc
      - name: webserver-wallets
        persistentVolumeClaim:
          claimName: webserver-wallets-pvc
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
2. ConfigMap and Secrets
3. PersistentVolumeClaims
4. StatefulSet/Deployment
5. Services
6. HPA
7. PodDisruptionBudget

Or use Kustomize to deploy all at once:

```bash
kubectl apply -k .
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
