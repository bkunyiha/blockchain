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
37. **23A: Code Listings** ← *You are here*

### Part III: Language Reference

38. <a href="../../rust/README.md">Chapter 24: Rust Language Guide</a>

</details>

</div>

---
<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

## Chapter 23A: Kubernetes — Complete Code Listings

This companion chapter contains **complete, verbatim listings** of the Kubernetes deployment artifacts used by Chapter 23.

When the narrative chapter discusses readiness probes, init containers, per-pod PVCs, or the deployment scripts, the authoritative implementation is printed here so the reader does not need the repository open.

---

## How to use this chapter

Before each listing, you will find a short guide explaining:

- what the resource/script does in the cluster,
- what fields are operationally critical,
- and what other resources it depends on.

---

## Listing 9.1: `ci/kubernetes/manifests/deploy.sh`

This is the “opinionated `kubectl apply`” script. It applies resources in dependency order and waits for pods to become Ready.

> **Methods involved**
> - Artifact: `deploy.sh` (deployment orchestration)

```bash
#!/bin/bash
# Deployment script for Kubernetes blockchain network
# Usage: ./deploy.sh [environment]
# Example: ./deploy.sh production

set -e

ENVIRONMENT=${1:-default}

echo "Deploying blockchain network to Kubernetes..."
echo "Environment: ${ENVIRONMENT}"
echo ""

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo "Error: kubectl is not installed or not in PATH"
    exit 1
fi

# Check if connected to cluster
if ! kubectl cluster-info &> /dev/null; then
    echo "Error: Not connected to Kubernetes cluster"
    echo "Please configure kubectl to connect to your cluster"
    exit 1
fi

echo "Cluster information:"
kubectl cluster-info | head -1
echo ""

# Deploy in order
echo "Step 1: Creating namespace..."
kubectl apply -f 01-namespace.yaml

echo "Step 2: Creating configuration..."
kubectl apply -f 02-configmap.yaml
kubectl apply -f 14-configmap-rate-limit.yaml
kubectl apply -f 03-secrets.yaml

echo "Step 3: Creating storage..."
kubectl apply -f 04-pvc-miner.yaml

echo "Step 4: Creating rate limiting backend (Redis)..."
kubectl apply -f 15-redis.yaml

echo "Step 5: Creating StatefulSet and Deployment..."
kubectl apply -f 06-statefulset-miner.yaml
kubectl apply -f 09-service-webserver-headless.yaml

# If you're upgrading from the old setup (webserver Deployment -> StatefulSet),
# remove the old Deployment so it doesn't keep spawning pods that share PVCs.
kubectl delete deployment webserver -n blockchain --ignore-not-found=true || true

kubectl apply -f 07-deployment-webserver.yaml

echo "Step 6: Creating services..."
kubectl apply -f 08-service-miner-headless.yaml
kubectl apply -f 08-service-miner.yaml
kubectl apply -f 09-service-webserver.yaml

echo "Step 7: Creating autoscalers..."
kubectl apply -f 10-hpa-webserver.yaml
kubectl apply -f 11-hpa-miner.yaml

echo "Step 8: Creating disruption budgets..."
kubectl apply -f 12-pod-disruption-budget.yaml

echo ""
echo "Deployment complete!"
echo ""
echo "Waiting for pods to be ready..."
kubectl wait --for=condition=ready pod -l app=miner -n blockchain --timeout=300s || true
kubectl wait --for=condition=ready pod -l app=webserver -n blockchain --timeout=300s || true

echo ""
echo "Current status:"
kubectl get pods -n blockchain
echo ""
kubectl get svc -n blockchain
echo ""
kubectl get hpa -n blockchain

echo ""
echo "To view logs:"
echo "  kubectl logs -n blockchain -l app=miner -f"
echo "  kubectl logs -n blockchain -l app=webserver -f"
echo ""
echo "To access webserver:"
echo "  kubectl port-forward -n blockchain svc/webserver-service 8080:8080"
echo "  Then open http://localhost:8080"
```

---

## Listing 9.2: `ci/kubernetes/manifests/undeploy.sh`

Reverse of `deploy.sh`: deletes resources in a safe teardown order.

> **Methods involved**
> - Artifact: `undeploy.sh` (teardown orchestration)

```bash
#!/bin/bash
# Undeployment script for Kubernetes blockchain network
# Usage: ./undeploy.sh

set -e

echo "Undeploying blockchain network from Kubernetes..."
echo ""

# Delete resources in reverse order
echo "Deleting disruption budgets..."
kubectl delete -f 12-pod-disruption-budget.yaml --ignore-not-found=true

echo "Deleting autoscalers..."
kubectl delete -f 11-hpa-miner.yaml --ignore-not-found=true
kubectl delete -f 10-hpa-webserver.yaml --ignore-not-found=true

echo "Deleting services..."
kubectl delete -f 09-service-webserver.yaml --ignore-not-found=true
kubectl delete -f 09-service-webserver-headless.yaml --ignore-not-found=true
kubectl delete -f 08-service-miner.yaml --ignore-not-found=true
kubectl delete -f 15-redis.yaml --ignore-not-found=true

echo "Deleting deployments..."
kubectl delete -f 07-deployment-webserver.yaml --ignore-not-found=true
kubectl delete -f 06-statefulset-miner.yaml --ignore-not-found=true

echo "Deleting storage..."
kubectl delete -f 04-pvc-miner.yaml --ignore-not-found=true

echo "Deleting configuration..."
kubectl delete -f 03-secrets.yaml --ignore-not-found=true
kubectl delete -f 14-configmap-rate-limit.yaml --ignore-not-found=true
kubectl delete -f 02-configmap.yaml --ignore-not-found=true

echo "Deleting namespace..."
kubectl delete -f 01-namespace.yaml --ignore-not-found=true

echo ""
echo "Undeployment complete!"
echo ""
echo "Note: PersistentVolumeClaims were deleted. Data may be lost unless backups were made."
```

---

## Listing 9.3: `ci/kubernetes/manifests/kustomization.yaml`

Kustomize entrypoint for `kubectl apply -k .`. It lists resources in the intended dependency order.

> **Methods involved**
> - Artifact: `kustomization.yaml`

```yaml
# Kustomization file for organizing Kubernetes resources
# Usage: kubectl apply -k .
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization

namespace: blockchain

resources:
  - 01-namespace.yaml
  - 02-configmap.yaml
  - 14-configmap-rate-limit.yaml
  - 03-secrets.yaml
  - 04-pvc-miner.yaml
  - 15-redis.yaml
  - 06-statefulset-miner.yaml
  - 07-deployment-webserver.yaml
  - 08-service-miner-headless.yaml
  - 08-service-miner.yaml
  - 09-service-webserver-headless.yaml
  - 09-service-webserver.yaml
  - 10-hpa-webserver.yaml
  - 11-hpa-miner.yaml
  - 12-pod-disruption-budget.yaml

commonLabels:
  managed-by: kustomize
  part-of: blockchain-network
```

---

## Listing 9.4: `ci/kubernetes/manifests/01-namespace.yaml`

Creates the `blockchain` namespace to isolate resources and simplify operations.

> **Methods involved**
> - Artifact: namespace resource

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: blockchain
  labels:
    name: blockchain
    environment: production
    managed-by: kubernetes
```

---

## Listing 9.5: `ci/kubernetes/manifests/02-configmap.yaml`

Defines non-secret configuration values. Workloads reference these keys as environment variables.

> **Methods involved**
> - Artifact: ConfigMap resource

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: blockchain-config
  namespace: blockchain
  labels:
    app: blockchain
    component: config
data:
  # Default node configuration
  # These are common settings used by all node types
  NODE_IS_WEB_SERVER: "no"
  SEQUENTIAL_STARTUP: "no"  # Kubernetes handles orchestration, so we don't need sequential startup
  CENTERAL_NODE: ""         # Optional central node address
  WALLET_FILE: "wallets/wallets.dat"  # Path to wallet file
  
  # Miner-specific defaults
  # These are used when creating miner pods
  MINER_NODE_IS_MINER: "yes"           # Enable mining
  MINER_NODE_IS_WEB_SERVER: "no"       # Disable web server
  MINER_NODE_CONNECT_NODES: "local"    # Start as seed node (first miner)
  
  # Webserver-specific defaults
  # These are used when creating webserver pods
  WEBSERVER_NODE_IS_MINER: "no"       # Disable mining
  WEBSERVER_NODE_IS_WEB_SERVER: "yes"  # Enable web server
  # Connect to miner service using Kubernetes DNS
  # Format: <service-name>.<namespace>.svc.cluster.local:<port>
  WEBSERVER_NODE_CONNECT_NODES: "miner-service.blockchain.svc.cluster.local:2001"
```

---

## Listing 9.6: `ci/kubernetes/manifests/14-configmap-rate-limit.yaml`

ConfigMap that contains a `Settings.toml` file. It is mounted into the webserver container as `/app/Settings.toml`.

> **Methods involved**
> - Artifact: ConfigMap (file-as-value pattern)

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: rate-limit-settings
  namespace: blockchain
  labels:
    app: blockchain
    component: rate-limit
data:
  # Settings.toml for axum_rate_limiter
  #
  # IMPORTANT:
  # - `redis_addr` is host:port WITHOUT the `redis://` scheme
  # - `ip_whitelist` is optional; keep it empty by default in Kubernetes
  Settings.toml: |
    [rate_limiter]
    redis_addr = "redis:6379"
    ip_whitelist = []

    [[rate_limiter.limiter]]
    strategy = "ip"
    global_bucket = { tokens_count = 20, add_tokens_every = 6 }
```

---

## Listing 9.7: `ci/kubernetes/manifests/03-secrets.yaml`

Secret values (API keys). Mining address can be omitted; the container entrypoint can generate one and persist it.

> **Methods involved**
> - Artifact: Secret resource

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: blockchain-secrets
  namespace: blockchain
  labels:
    app: blockchain
    component: secrets
type: Opaque
stringData:
  # API Keys for blockchain node authentication
  BITCOIN_API_ADMIN_KEY: "admin-secret"   # Admin API key for blockchain operations
  BITCOIN_API_WALLET_KEY: "wallet-secret" # Wallet API key for wallet operations
  # Optional: Mining address (wallet address).
  #
  # If omitted (or left empty), the container entrypoint will auto-create a wallet address
  # and persist it in the pod's wallet volume (see docs).
  # MINER_ADDRESS: ""

---
# Note: In production, use sealed-secrets or external secret management:
# - Sealed Secrets: Encrypt secrets so they can be stored in Git
# - AWS Secrets Manager: Store secrets in AWS and sync to Kubernetes
# - HashiCorp Vault: External secret management system
# - External Secrets Operator: Sync secrets from external systems
```

---

## Listing 9.8: `ci/kubernetes/manifests/04-pvc-miner.yaml`

Legacy PVCs (not used by StatefulSet volume templates). Kept for compatibility with older setups and for clusters without dynamic provisioning.

> **Methods involved**
> - Artifact: PVC resources

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: miner-data-pvc
  namespace: blockchain
  labels:
    app: miner
    component: storage
spec:
  accessModes:
    - ReadWriteOnce  # Can be mounted by a single node
  resources:
    requests:
      storage: 50Gi  # Adjust based on your needs
  # Uncomment and set if you have storage classes configured
  # storageClassName: fast-ssd
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: miner-wallets-pvc
  namespace: blockchain
  labels:
    app: miner
    component: storage
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi  # Wallets are typically small
  # Uncomment and set if you have storage classes configured
  # storageClassName: fast-ssd
```

---

## Listing 9.9: `ci/kubernetes/manifests/05-pvc-webserver.yaml`

Legacy PVCs for webserver data/wallets (see note above).

> **Methods involved**
> - Artifact: PVC resources

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: webserver-data-pvc
  namespace: blockchain
  labels:
    app: webserver
    component: storage
spec:
  accessModes:
    - ReadWriteOnce  # Can be mounted by a single node
  resources:
    requests:
      storage: 50Gi  # Adjust based on your needs
  # Uncomment and set if you have storage classes configured
  # storageClassName: fast-ssd
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: webserver-wallets-pvc
  namespace: blockchain
  labels:
    app: webserver
    component: storage
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi  # Wallets are typically small
  # Uncomment and set if you have storage classes configured
  # storageClassName: fast-ssd
```

---

## Listing 9.10: `ci/kubernetes/manifests/15-redis.yaml`

Redis Deployment + Service for rate limiting state.

> **Methods involved**
> - Artifact: Service + Deployment resources

```yaml
apiVersion: v1
kind: Service
metadata:
  name: redis
  namespace: blockchain
  labels:
    app: redis
    component: rate-limit
spec:
  type: ClusterIP
  selector:
    app: redis
  ports:
    - name: redis
      port: 6379
      targetPort: 6379
      protocol: TCP

---

apiVersion: apps/v1
kind: Deployment
metadata:
  name: redis
  namespace: blockchain
  labels:
    app: redis
    component: rate-limit
spec:
  replicas: 1
  selector:
    matchLabels:
      app: redis
  template:
    metadata:
      labels:
        app: redis
        component: rate-limit
    spec:
      containers:
        - name: redis
          image: redis:7-alpine
          imagePullPolicy: IfNotPresent
          ports:
            - containerPort: 6379
              name: redis
          resources:
            requests:
              cpu: "50m"
              memory: "64Mi"
            limits:
              cpu: "500m"
              memory: "256Mi"
          readinessProbe:
            tcpSocket:
              port: 6379
            initialDelaySeconds: 3
            periodSeconds: 5
          livenessProbe:
            tcpSocket:
              port: 6379
            initialDelaySeconds: 10
            periodSeconds: 10
          volumeMounts:
            - name: redis-data
              mountPath: /data
      volumes:
        - name: redis-data
          emptyDir: {}
```

---

## Listing 9.11: `ci/kubernetes/manifests/08-service-miner-headless.yaml`

Headless Service for stable per-pod DNS names used by StatefulSet.

> **Methods involved**
> - Artifact: Service resource

```yaml
apiVersion: v1
kind: Service
metadata:
  name: miner-headless
  namespace: blockchain
  labels:
    app: miner
    service-type: headless
spec:
  clusterIP: None  # ⚠️ Critical: Headless service - returns pod IPs directly
  selector:
    app: miner  # ⚠️ Critical: Must match pod labels from StatefulSet
  ports:
  - name: p2p
    port: 2001
    targetPort: 2001
    protocol: TCP
  # Note: No sessionAffinity for headless service (direct pod-to-pod connections)
```

---

## Listing 9.12: `ci/kubernetes/manifests/08-service-miner.yaml`

ClusterIP service for miners (stable name for clients).

> **Methods involved**
> - Artifact: Service resource

```yaml
apiVersion: v1
kind: Service
metadata:
  name: miner-service
  namespace: blockchain
  labels:
    app: miner
spec:
  type: ClusterIP  # Internal service, not exposed externally
  selector:
    app: miner
  ports:
  - name: p2p
    port: 2001
    targetPort: 2001
    protocol: TCP
  # Session affinity for consistent connections
  sessionAffinity: ClientIP
  sessionAffinityConfig:
    clientIP:
      timeoutSeconds: 10800  # 3 hours
```

---

## Listing 9.13: `ci/kubernetes/manifests/09-service-webserver-headless.yaml`

Headless Service for webserver StatefulSet.

> **Methods involved**
> - Artifact: Service resource

```yaml
apiVersion: v1
kind: Service
metadata:
  name: webserver-headless
  namespace: blockchain
  labels:
    app: webserver
    service-type: headless
spec:
  clusterIP: None
  selector:
    app: webserver
  ports:
    - name: web
      port: 8080
      targetPort: 8080
      protocol: TCP
    - name: p2p
      port: 2001
      targetPort: 2001
      protocol: TCP
```

---

## Listing 9.14: `ci/kubernetes/manifests/09-service-webserver.yaml`

Webserver Service. In cloud clusters it is typically LoadBalancer; in local clusters you often use port-forwarding.

> **Methods involved**
> - Artifact: Service resource

```yaml
apiVersion: v1
kind: Service
metadata:
  name: webserver-service
  namespace: blockchain
  labels:
    app: webserver
spec:
  # For cloud providers, use LoadBalancer
  # For local development (Minikube), use NodePort or enable tunnel
  type: LoadBalancer
  # Alternative for local: type: NodePort
  selector:
    app: webserver
  ports:
  - name: web
    port: 8080
    targetPort: 8080
    protocol: TCP
    # For NodePort, uncomment and specify:
    # nodePort: 30080
  - name: p2p
    port: 2101
    targetPort: 2001  # Internal port is 2001
    protocol: TCP
    # For NodePort, uncomment and specify:
    # nodePort: 32101
```

---

## Listing 9.15: `ci/kubernetes/manifests/06-statefulset-miner.yaml`

Miner StatefulSet. Uses `volumeClaimTemplates` to allocate per-pod storage.

> **Methods involved**
> - Artifact: StatefulSet resource

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: miner
  namespace: blockchain
  labels:
    app: miner
    component: blockchain-node
spec:
  serviceName: miner-headless  # ⚠️ Critical: Links to headless service for stable DNS
  replicas: 2  # Start with 2 miners
  selector:
    matchLabels:
      app: miner

  # ⚠️ Critical: StatefulSet uses volumeClaimTemplates (at spec level, not pod template)
  # Each pod gets its own PVC automatically: miner-data-miner-0, miner-data-miner-1, etc.
  # This ensures each miner has isolated persistent storage
  volumeClaimTemplates:
  - metadata:
      name: miner-data
    spec:
      accessModes:
        - ReadWriteOnce  # Single pod access
      resources:
        requests:
          storage: 50Gi  # Adjust based on blockchain size
  - metadata:
      name: miner-wallets
    spec:
      accessModes:
        - ReadWriteOnce
      resources:
        requests:
          storage: 1Gi  # Wallets are typically small

  template:
    metadata:
      labels:
        app: miner
        component: blockchain-node
    spec:
      containers:
      - name: blockchain-node
        image: blockchain-node:latest
        # For production, use a specific version tag and registry:
        # image: your-registry/blockchain-node:v1.0.0
        imagePullPolicy: IfNotPresent  # Use Always in production
        
        # Ports
        ports:
        - name: p2p
          containerPort: 2001
          protocol: TCP
        
        # Environment variables from ConfigMap
        env:
        - name: NODE_IS_MINER
          valueFrom:
            configMapKeyRef:
              name: blockchain-config
              key: MINER_NODE_IS_MINER
        - name: NODE_IS_WEB_SERVER
          valueFrom:
            configMapKeyRef:
              name: blockchain-config
              key: MINER_NODE_IS_WEB_SERVER
        # ⚠️ Important: NODE_CONNECT_NODES will be set by entrypoint script based on pod ordinal
        - name: NODE_CONNECT_NODES
          valueFrom:
            configMapKeyRef:
              name: blockchain-config
              key: MINER_NODE_CONNECT_NODES
        - name: SEQUENTIAL_STARTUP
          valueFrom:
            configMapKeyRef:
              name: blockchain-config
              key: SEQUENTIAL_STARTUP
        - name: CENTERAL_NODE
          valueFrom:
            configMapKeyRef:
              name: blockchain-config
              key: CENTERAL_NODE
        - name: WALLET_FILE
          valueFrom:
            configMapKeyRef:
              name: blockchain-config
              key: WALLET_FILE
        
        # Environment variables from Secrets
        - name: NODE_MINING_ADDRESS
          valueFrom:
            secretKeyRef:
              name: blockchain-secrets
              key: MINER_ADDRESS
              optional: true
        
        # ⚠️ Critical: Use Downward API to get pod information for StatefulSet
        # StatefulSet pods have stable names: miner-0, miner-1, miner-2, etc.
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
        
        # Volume mounts
        # ⚠️ Important: StatefulSet uses volumeClaimTemplates instead of volumes
        # Each pod gets its own PVC automatically
        volumeMounts:
        - name: miner-data
          mountPath: /app/data
        - name: miner-wallets
          mountPath: /app/wallets
        
        # Resource limits and requests
        resources:
          requests:
            cpu: "500m"      # 0.5 CPU
            memory: "512Mi"  # 512 MB RAM
          limits:
            cpu: "2000m"     # 2 CPU (mining is CPU-intensive)
            memory: "2Gi"    # 2 GB RAM
        
        # Health checks
        livenessProbe:
          tcpSocket:
            port: 2001
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3
        
        readinessProbe:
          tcpSocket:
            port: 2001
          initialDelaySeconds: 10
          periodSeconds: 5
          timeoutSeconds: 3
          failureThreshold: 3
```

---

## Listing 9.16: `ci/kubernetes/manifests/07-deployment-webserver.yaml`

Webserver StatefulSet. Uses initContainers to wait for miner + redis and mounts `Settings.toml` for rate limiting.

> **Methods involved**
> - Artifact: StatefulSet resource

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: webserver
  namespace: blockchain
  labels:
    app: webserver
    component: blockchain-node
spec:
  serviceName: webserver-headless
  replicas: 2  # Start with 2 webservers
  selector:
    matchLabels:
      app: webserver

  # Each webserver gets its own PVCs (isolated blockchain + wallet storage)
  volumeClaimTemplates:
  - metadata:
      name: webserver-data
    spec:
      accessModes:
        - ReadWriteOnce
      resources:
        requests:
          storage: 50Gi
  - metadata:
      name: webserver-wallets
    spec:
      accessModes:
        - ReadWriteOnce
      resources:
        requests:
          storage: 1Gi

  template:
    metadata:
      labels:
        app: webserver
        component: blockchain-node
    spec:
      # Init containers: Wait for miner service to be available
      initContainers:
      - name: wait-for-miner
        image: busybox:1.35
        command: ['sh', '-c', 'until nc -z miner-service.blockchain.svc.cluster.local 2001; do echo waiting for miner; sleep 2; done;']
      - name: wait-for-redis
        image: busybox:1.35
        command: ['sh', '-c', 'until nc -z redis.blockchain.svc.cluster.local 6379; do echo waiting for redis; sleep 2; done;']
      
      containers:
      - name: blockchain-node
        image: blockchain-node:latest
        # For production, use a specific version tag and registry:
        # image: your-registry/blockchain-node:v1.0.0
        imagePullPolicy: IfNotPresent
        
        # Ports
        ports:
        - name: web
          containerPort: 8080
          protocol: TCP
        - name: p2p
          containerPort: 2001
          protocol: TCP
        
        # Environment variables from ConfigMap
        env:
        - name: NODE_IS_MINER
          valueFrom:
            configMapKeyRef:
              name: blockchain-config
              key: WEBSERVER_NODE_IS_MINER
        - name: NODE_IS_WEB_SERVER
          valueFrom:
            configMapKeyRef:
              name: blockchain-config
              key: WEBSERVER_NODE_IS_WEB_SERVER
        - name: NODE_CONNECT_NODES
          valueFrom:
            configMapKeyRef:
              name: blockchain-config
              key: WEBSERVER_NODE_CONNECT_NODES
        - name: SEQUENTIAL_STARTUP
          valueFrom:
            configMapKeyRef:
              name: blockchain-config
              key: SEQUENTIAL_STARTUP
        - name: CENTERAL_NODE
          valueFrom:
            configMapKeyRef:
              name: blockchain-config
              key: CENTERAL_NODE
        - name: WALLET_FILE
          valueFrom:
            configMapKeyRef:
              name: blockchain-config
              key: WALLET_FILE

        # Rate limiting configuration (axum_rate_limiter)
        - name: RL_SETTINGS_PATH
          value: /app/Settings.toml
        
        # Environment variables from Secrets
        - name: BITCOIN_API_ADMIN_KEY
          valueFrom:
            secretKeyRef:
              name: blockchain-secrets
              key: BITCOIN_API_ADMIN_KEY
        - name: BITCOIN_API_WALLET_KEY
          valueFrom:
            secretKeyRef:
              name: blockchain-secrets
              key: BITCOIN_API_WALLET_KEY
        
        # Environment variables from Secrets
        - name: NODE_MINING_ADDRESS
          valueFrom:
            secretKeyRef:
              name: blockchain-secrets
              key: MINER_ADDRESS
              optional: true
        
        # Use Downward API to get pod information
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
        
        # Volume mounts
        volumeMounts:
        - name: webserver-data
          mountPath: /app/data
        - name: webserver-wallets
          mountPath: /app/wallets
        - name: rate-limit-settings
          mountPath: /app/Settings.toml
          subPath: Settings.toml
          readOnly: true
        
        # Resource limits
        resources:
          requests:
            cpu: "250m"
            memory: "256Mi"
          limits:
            cpu: "1000m"
            memory: "1Gi"
        
        # Health checks
        livenessProbe:
          httpGet:
            path: /api/health/live
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3
        
        readinessProbe:
          httpGet:
            path: /api/health/ready
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 5
          timeoutSeconds: 3
          failureThreshold: 3
      
      # Volumes
      volumes:
      - name: rate-limit-settings
        configMap:
          name: rate-limit-settings
          items:
            - key: Settings.toml
              path: Settings.toml
```

---

## Listing 9.17: `ci/kubernetes/manifests/10-hpa-webserver.yaml`

HPA for webservers (CPU + memory).

> **Methods involved**
> - Artifact: HPA resource

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: webserver-hpa
  namespace: blockchain
  labels:
    app: webserver
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: StatefulSet
    name: webserver
  minReplicas: 1
  maxReplicas: 10
  metrics:
  # CPU-based scaling
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70  # Scale up if CPU > 70%
  # Memory-based scaling
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80  # Scale up if Memory > 80%
  behavior:
    scaleDown:
      stabilizationWindowSeconds: 300  # Wait 5 min before scaling down
      policies:
      - type: Percent
        value: 50  # Scale down by 50% at a time
        periodSeconds: 60
    scaleUp:
      stabilizationWindowSeconds: 0  # Scale up immediately
      policies:
      - type: Pods
        value: 2  # Add 2 pods at a time
        periodSeconds: 60
      - type: Percent
        value: 100  # Or double the pods
        periodSeconds: 60
      selectPolicy: Max  # Use the policy that scales up the most
```

---

## Listing 9.18: `ci/kubernetes/manifests/11-hpa-miner.yaml`

HPA for miners (conservative scaling due to statefulness).

> **Methods involved**
> - Artifact: HPA resource

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: miner-hpa
  namespace: blockchain
  labels:
    app: miner
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: StatefulSet  # ⚠️ Important: Changed from Deployment to StatefulSet
    name: miner
  minReplicas: 1
  maxReplicas: 5
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 80  # Miners can use more CPU
  behavior:
    scaleDown:
      stabilizationWindowSeconds: 600  # Wait 10 min before scaling down miners
    scaleUp:
      stabilizationWindowSeconds: 0
      policies:
      - type: Pods
        value: 1  # Add 1 miner at a time (more conservative)
        periodSeconds: 120
```

---

## Listing 9.19: `ci/kubernetes/manifests/12-pod-disruption-budget.yaml`

Ensures that at least one miner and one webserver remain available during voluntary disruptions.

> **Methods involved**
> - Artifact: PDB resources

```yaml
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: miner-pdb
  namespace: blockchain
  labels:
    app: miner
spec:
  minAvailable: 1  # Always keep at least 1 miner running
  selector:
    matchLabels:
      app: miner
---
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: webserver-pdb
  namespace: blockchain
  labels:
    app: webserver
spec:
  minAvailable: 1  # Always keep at least 1 webserver running
  selector:
    matchLabels:
      app: webserver
```

---

## Listing 9.20: `ci/kubernetes/manifests/13-network-policy.yaml`

Optional network policy template (commented out by default).

> **Methods involved**
> - Artifact: NetworkPolicy template

```yaml
# Network Policy for securing blockchain network
# Uncomment and customize based on your security requirements

# apiVersion: networking.k8s.io/v1
# kind: NetworkPolicy
# metadata:
#   name: blockchain-netpol
#   namespace: blockchain
# spec:
#   podSelector:
#     matchLabels:
#       component: blockchain-node
#   policyTypes:
#   - Ingress
#   - Egress
#   ingress:
#   # Allow traffic from within the namespace
#   - from:
#     - namespaceSelector:
#         matchLabels:
#           name: blockchain
#     ports:
#     - protocol: TCP
#       port: 2001  # P2P port
#     - protocol: TCP
#       port: 8080  # Web port
#   # Allow traffic from ingress controller (if using)
#   - from:
#     - namespaceSelector:
#         matchLabels:
#           name: ingress-nginx
#     ports:
#     - protocol: TCP
#       port: 8080
#   egress:
#   # Allow DNS resolution
#   - to:
#     - namespaceSelector:
#         matchLabels:
#           name: kube-system
#     ports:
#     - protocol: UDP
#       port: 53
#   # Allow traffic to miner service
#   - to:
#     - podSelector:
#         matchLabels:
#           app: miner
#     ports:
#     - protocol: TCP
#       port: 2001
#   # Allow external internet access (for P2P networking)
#   - {}  # Allow all egress (required for blockchain P2P)
```

---

<div align="center">

**Reading order**

**[← Previous: Kubernetes Deployment](README.md)** | **[Next: Rust Language Guide →](../../rust/README.md)**

</div>

---

