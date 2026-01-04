<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../../01-Introduction.md) - Book introduction, project structure, technical stack
2. [Chapter 1.2: Introduction to Bitcoin & Blockchain](../../bitcoin-blockchain/README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 1.3: Bitcoin Whitepaper](../../bitcoin-blockchain/00-Bitcoin-Whitepaper-Summary.md) - Bitcoin Whitepaper
4. [Chapter 1.4: Bitcoin Whitepaper In Rust](../../bitcoin-blockchain/whitepaper-rust/README.md) - Bitcoin Whitepaper In Rust
5. [Chapter 2.0: Rust Blockchain Project](../../bitcoin-blockchain/Rust-Project-Index.md) - Blockchain Project
6. [Chapter 2.1: Primitives](../../bitcoin-blockchain/primitives/README.md) - Core data structures
7. [Chapter 2.2: Utilities](../../bitcoin-blockchain/util/README.md) - Utility functions and helpers
8. [Chapter 2.3: Cryptography](../../bitcoin-blockchain/crypto/README.md) - Cryptographic primitives and libraries
9. [Chapter 2.4: Blockchain(POW & Block Acceptance)](../../bitcoin-blockchain/chain/01-Technical-Foundations.md) - Proof Of Work
10. [Chapter 2.5: Storage Layer](../../bitcoin-blockchain/store/README.md) - Persistent storage implementation
11. [Chapter 2.6: Blockchain(POW & Block Acceptance)](../../bitcoin-blockchain/chain/02-Block-Acceptance-Whitepaper-Step-5.md) - Proof Of Work
12. [Chapter 2.7: Network Layer](../../bitcoin-blockchain/net/README.md) - Peer-to-peer networking and protocol
13. [Chapter 2.8: Node Orchestration](../../bitcoin-blockchain/node/README.md) - Node context and coordination
14. [Chapter 2.9: Wallet System](../../bitcoin-blockchain/wallet/README.md) - Wallet implementation and key management
15. [Chapter 3: Web API Architecture](../../bitcoin-blockchain/web/README.md) - REST API implementation
16. [Chapter 4: Desktop Admin Interface](../../bitcoin-desktop-ui/03-Desktop-Admin-UI.md) - Iced framework architecture
17. [Chapter 5: Wallet User Interface](../../bitcoin-wallet-ui/04-Wallet-UI.md) - Wallet UI implementation
18. [Chapter 6: Embedded Database & Persistence](../../bitcoin-wallet-ui/05-Embedded-Database.md) - SQLCipher integration
19. [Chapter 7: Web Admin Interface](../../bitcoin-web-ui/06-Web-Admin-UI.md) - React/TypeScript web UI

### Part II: Deployment & Operations

20. [Chapter 8: Docker Compose Deployment](../docker-compose/01-Introduction.md) - Docker Compose guide
21. **Chapter 9: Kubernetes Deployment** ← *You are here*
22. [Chapter 10: Rust Language Guide](../../rust/README.md) - Rust programming language reference

</details>

</div>

---
<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

# Chapter 9, Section 7: Production & Advanced Topics

**Part II: Deployment & Operations** | **Chapter 9: Kubernetes Deployment**

<div align="center">

**📚 [← Chapter 8: Docker Compose](../docker-compose/01-Introduction.md)** | **Chapter 9: Kubernetes** | **[End of Book →](../../../README.md)** 📚

</div>

---

This section is the “production hardening” part of the Kubernetes chapter: it explains what changes when you move from a local learning cluster to a real environment, how to run safe deployments via CI/CD, and how cloud providers differ in the details that matter (identity, load balancers, storage, and registries).

## How to Read This Section

- If you need a **production checklist**, start at [Production Checklist](#production-checklist).
- If you’re setting up pipelines, read [CI/CD (GitHub / GitLab)](#cicd-github--gitlab--build-publish-deploy).
- If you’re choosing a provider (or migrating), read [Deploying to the Cloud](#deploying-to-the-cloud-aws-vs-azure-vs-gcp).
- If you’re debugging complex behaviors, jump to [Troubleshooting](#troubleshooting).

## Table of Contents

- [The Production Mindset](#the-production-mindset)
- [Production Foundations (Security, Reliability, Operations)](#production-foundations-security-reliability-operations)
  - [Release Engineering (immutable images, rollouts)](#release-engineering-immutable-images-rollouts)
  - [Supply Chain Security (SBOM, scanning, signing)](#supply-chain-security-sbom-scanning-signing)
  - [Identity & Access (RBAC, workload identity)](#identity--access-rbac-workload-identity)
  - [Network Controls (ingress/egress, NetworkPolicy)](#network-controls-ingressegress-networkpolicy)
  - [Runtime Hardening (pod security context)](#runtime-hardening-pod-security-context)
  - [Stateful Data (PVCs, snapshots, retention)](#stateful-data-pvcs-snapshots-retention)
  - [Redis in Production (rate limiting backend)](#redis-in-production-rate-limiting-backend)
  - [Observability (metrics, logs, traces)](#observability-metrics-logs-traces)
  - [Backups & Disaster Recovery](#backups--disaster-recovery)
  - [Availability During Maintenance (PDBs)](#availability-during-maintenance-pdbs)
- [Production Checklist](#production-checklist)
- [CI/CD (GitHub / GitLab) — Build, Publish, Deploy](#cicd-github--gitlab--build-publish-deploy)
- [Deploying to the Cloud (AWS vs Azure vs GCP)](#deploying-to-the-cloud-aws-vs-azure-vs-gcp)
- [Advanced Topics](#advanced-topics)
- [Troubleshooting](#troubleshooting)

## The Production Mindset

Local clusters are forgiving; production is not. In production, you should assume:

- **Nodes fail** (voluntary disruptions, upgrades, spot preemption, hardware failure).
- **Traffic patterns are adversarial** (bursts, retries, scraping, scanners).
- **Humans make mistakes** (bad config, wrong image tags, accidental deletes).
- **Your dependencies become production systems** (Redis, storage provisioners, DNS, ingress).

Production hardening is therefore less about adding “more YAML”, and more about installing *guardrails*:

- **immutability** (build once, deploy many)
- **least privilege** (RBAC, secrets, network)
- **controlled change** (rollouts, validation, fast rollback)
- **operability** (metrics/logs/traces, runbooks, backups)

## Production Foundations (Security, Reliability, Operations)

### Release Engineering (immutable images, rollouts)

The first production rule is: **stop using `latest`**.

- Use **content-addressable tags** (commit SHA) and optionally a semver tag.
- Prefer `imagePullPolicy: IfNotPresent` when the tag is immutable; use `Always` mainly for mutable tags (which you should avoid).

```yaml
image: your-registry/blockchain-node:sha-<git-sha>
imagePullPolicy: IfNotPresent
```

Stateful workloads require extra rollout discipline:

- For StatefulSets, Kubernetes uses **RollingUpdate** by default, updating pods in ordinal order.
- Consider **partitioned rollouts** for staged deployments: update the image, but only roll a subset of ordinals first.

Why it matters: stateful nodes are part of a network and also hold durable state; a bad rollout can create systemic instability.

**Config example: StatefulSet rollout strategy (partitioned rollouts)**

Use a partitioned rollout when you want to stage changes (for example: upgrade `webserver-0` first, validate, then roll the rest).

```yaml
spec:
  updateStrategy:
    type: RollingUpdate
    rollingUpdate:
      # Pods with ordinal < partition are updated; pods >= partition are left alone.
      # Example: partition: 1 updates webserver-0 but keeps webserver-1, webserver-2, ...
      partition: 1
```

**Config example: Kustomize pinning by image tag**

Kustomize **overlays** are environment-specific layers (for example: `overlays/dev`, `overlays/staging`, `overlays/prod`) that *reuse a shared base* set of manifests and then apply small, reviewable changes (image tags, replica counts, resource limits, ingress hostnames, feature flags). They’re important in production because they help you keep **one canonical “base”** while still allowing **controlled, auditable differences** between environments—without copy/pasting YAML and drifting over time.

If you use Kustomize overlays, you can pin an image tag without editing every manifest:

```yaml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
resources:
  - ../../ci/kubernetes/manifests
images:
  - name: blockchain-node
    newName: your-registry/blockchain-node
    newTag: sha-<git-sha>
```

### Supply Chain Security (SBOM, scanning, signing)

In production you should treat the container image as an artifact with provenance:

- **SBOM** (Software Bill of Materials): helps answer “what’s inside this image?”
- **Vulnerability scanning**: catch known CVEs in OS packages and Rust deps.
- **Signing + verification**: ensure the cluster only runs images built by your pipeline.

A practical baseline:

- Run `cargo audit` in CI (Rust dependency advisories)
- Run container scanning (Trivy/Grype) on the final image
- Generate an SBOM (Syft) and publish it with the image
- Optionally enforce signature verification (Cosign) via an admission controller (policy)

### Identity & Access (RBAC, workload identity)

Two core principles:

- **Humans**: access via SSO and time-bound permissions.
- **Automation**: deploy via a dedicated service account with minimal RBAC.

In managed Kubernetes, prefer **workload identity** over static cloud keys:

- AWS EKS: IRSA (IAM Roles for Service Accounts)
- GKE: Workload Identity
- AKS: Workload Identity / Managed Identity

Why it matters: short-lived credentials reduce blast radius and remove the “secret-in-CI” problem.

**Config example: minimal namespace-scoped RBAC for CI deploys**

This example grants a CI service account the ability to apply common objects and update StatefulSets in the `blockchain` namespace. (In real production, tighten this further to exactly what your deployment method needs.)

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: ci-deployer
  namespace: blockchain
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: ci-deployer-role
  namespace: blockchain
rules:
  # Apply and update common namespace-scoped resources
  - apiGroups: [""]
    resources: ["configmaps", "services", "endpoints", "pods", "events", "persistentvolumeclaims"]
    verbs: ["get", "list", "watch", "create", "update", "patch", "delete"]
  - apiGroups: ["apps"]
    resources: ["statefulsets"]
    verbs: ["get", "list", "watch", "create", "update", "patch"]
  - apiGroups: ["autoscaling"]
    resources: ["horizontalpodautoscalers"]
    verbs: ["get", "list", "watch", "create", "update", "patch"]
  - apiGroups: ["policy"]
    resources: ["poddisruptionbudgets"]
    verbs: ["get", "list", "watch", "create", "update", "patch"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: ci-deployer-binding
  namespace: blockchain
subjects:
  - kind: ServiceAccount
    name: ci-deployer
    namespace: blockchain
roleRef:
  kind: Role
  name: ci-deployer-role
  apiGroup: rbac.authorization.k8s.io
```

### Network Controls (ingress/egress, NetworkPolicy)

Production clusters typically move from “flat networking” to explicit allow-lists.

At a minimum, consider:

- **Ingress controls**: only expose the webserver HTTP interface to the paths/ports you intend.
- **Egress controls**: restrict pods so they can only talk to required services (miners, Redis, DNS).

NetworkPolicy is cluster-dependent (needs a CNI that enforces it). A common pattern:

1. Apply a “default deny” policy.
2. Add explicit allow policies for the required flows.

**Config example: default deny for a namespace**

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: default-deny
  namespace: blockchain
spec:
  podSelector: {}
  policyTypes:
    - Ingress
    - Egress
```

**Config example: allow webserver → miner-service + redis (egress)**

This illustrates a typical “tight egress” stance for the `webserver` pods.

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: allow-webserver-egress
  namespace: blockchain
spec:
  podSelector:
    matchLabels:
      app: webserver
  policyTypes:
  - Egress
  egress:
    # Allow calls to miners (P2P sync)
    - to:
        - podSelector:
        matchLabels:
              app: miner
    ports:
    - protocol: TCP
      port: 2001
    # Allow calls to Redis (rate limiting backend)
    - to:
        - podSelector:
            matchLabels:
              app: redis
      ports:
        - protocol: TCP
          port: 6379
    # Allow DNS (otherwise most clusters break name resolution)
    - to:
        - namespaceSelector:
            matchLabels:
              kubernetes.io/metadata.name: kube-system
          podSelector:
            matchLabels:
              k8s-app: kube-dns
      ports:
        - protocol: UDP
          port: 53
        - protocol: TCP
          port: 53
```

### Runtime Hardening (pod security context)

Hardening a container is about reducing what an attacker can do *after* they get code execution.

Typical baseline:

- run as non-root
- drop Linux capabilities
- use a read-only root filesystem where possible
- set seccomp profile

Even if you don’t enable all of these immediately, document the plan and the constraints (some apps need writable paths, or require a specific UID/GID for volumes).

**Config example: baseline pod/container securityContext**

```yaml
spec:
  template:
    spec:
      securityContext:
        runAsNonRoot: true
        seccompProfile:
          type: RuntimeDefault
      containers:
        - name: blockchain-node
          securityContext:
            allowPrivilegeEscalation: false
            readOnlyRootFilesystem: true
            capabilities:
              drop: ["ALL"]
```

If you set `readOnlyRootFilesystem: true`, ensure your app only writes to mounted volumes (e.g. `/app/data`, `/app/wallets`) and uses writable temp paths as needed.

### Stateful Data (PVCs, snapshots, retention)

For stateful applications, storage is part of correctness, not just “capacity”.

Key production decisions:

- **StorageClass**: what backs PVCs (EBS/PD/Azure Disk), performance tier, and reclaim policy.
- **Snapshot strategy**: use CSI `VolumeSnapshot` (or provider-native snapshots) rather than “copying files out of pods”.
- **Retention policy**: define what happens to PVCs when scaling down or deleting StatefulSets.

Also define *what recovery means*:

- is restoring a single pod acceptable?
- do you need point-in-time recovery?
- what’s your RPO/RTO?

**Config example: StorageClass (cloud CSI)**

Your exact StorageClass depends on your provider and CSI driver, but the important production ideas are: performance tier, encryption, and reclaim policy.

```yaml
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: fast-ssd
provisioner: ebs.csi.aws.com
parameters:
  type: gp3
  encrypted: "true"
reclaimPolicy: Retain
volumeBindingMode: WaitForFirstConsumer
```

**Config example: CSI snapshot + restore workflow**

Snapshots are typically expressed via `VolumeSnapshotClass` and `VolumeSnapshot` objects (requires snapshot controller + CSI support):

```yaml
apiVersion: snapshot.storage.k8s.io/v1
kind: VolumeSnapshotClass
metadata:
  name: csi-snapshots
driver: ebs.csi.aws.com
deletionPolicy: Retain
---
apiVersion: snapshot.storage.k8s.io/v1
kind: VolumeSnapshot
metadata:
  name: webserver-0-snapshot
  namespace: blockchain
spec:
  volumeSnapshotClassName: csi-snapshots
  source:
    persistentVolumeClaimName: webserver-data-webserver-0
```

Then restore by creating a PVC from the snapshot (and wiring that PVC into a replacement pod/workload):

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: webserver-data-restore
  namespace: blockchain
    spec:
  storageClassName: fast-ssd
  dataSource:
    name: webserver-0-snapshot
    kind: VolumeSnapshot
    apiGroup: snapshot.storage.k8s.io
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 10Gi
```

### Redis in Production (rate limiting backend)

In this project, webserver rate limiting uses Redis as a distributed state store. In production you should decide:

- **Durability**: do you need Redis persistence (AOF/RDB), or is rate-limit state acceptable to lose?
- **Availability**: do you need HA (Sentinel/managed Redis), or is a single replica acceptable?
- **Isolation**: dedicate Redis to rate limiting, or share it with other workloads (not recommended without guardrails)?

If you expect real traffic or multiple webserver replicas, a managed Redis (or a Redis HA setup) is usually the simplest operational choice.

**Config example: minimal Redis hardening knobs**

Even with a simple Redis Deployment, production usually adds explicit resources and persistence (or explicitly decides persistence is unnecessary).

```yaml
          containers:
  - name: redis
    image: redis:7-alpine
    args: ["--appendonly", "yes"]
    resources:
      requests:
        cpu: "100m"
        memory: "256Mi"
      limits:
        cpu: "500m"
        memory: "512Mi"
    ports:
      - containerPort: 6379
```

### Observability (metrics, logs, traces)

Production observability is “can we explain what happened?”:

- **Metrics**: capacity, saturation, errors, latency (RED/USE)
- **Logs**: structured, queryable, with retention
- **Traces**: request flow across components (optional but powerful)

If you add metrics to the webserver, pair it with:

- scrape (Prometheus)
- dashboards (Grafana)
- alerting (Alertmanager)

**Config example: Prometheus scrape via annotations**

If you don’t use the Prometheus Operator, a common pattern is annotating pods/services for scraping:

```yaml
metadata:
  annotations:
    prometheus.io/scrape: "true"
    prometheus.io/port: "8080"
    prometheus.io/path: "/metrics"
```

**Config example: Prometheus Operator ServiceMonitor**

If you do use the Prometheus Operator, you typically create a `ServiceMonitor` that selects the Service for scraping:

```yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: webserver
  namespace: blockchain
spec:
  selector:
    matchLabels:
      app: webserver
  endpoints:
  - port: web
    path: /metrics
      interval: 15s
```

### Backups & Disaster Recovery

Backups must be designed around *what you need to recover*:

- **Cluster resources**: Git + infrastructure-as-code (the cluster should be rebuildable)
- **Persistent data**: CSI snapshots + tested restore procedures

Avoid “backup by `kubectl cp` from pods” as a primary strategy; it is fragile, slow, and hard to operate at scale.

### Availability During Maintenance (PDBs)

PodDisruptionBudgets protect you during *voluntary* disruptions (node drains, upgrades):

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

PDBs do not prevent crashes; they prevent the platform from evicting too many pods at once.

### Secret Management

In production, avoid committing plaintext secrets. Prefer one of:

- **External Secrets Operator** + cloud secret manager (AWS Secrets Manager / GCP Secret Manager / Azure Key Vault)
- **Vault** (central secret control + dynamic secrets)
- **sealed-secrets** (encrypted secrets stored in Git, decrypted in-cluster)

**Config example: External Secrets Operator**

This is the “Git contains references, not values” model. It requires installing ESO and configuring access to your provider’s secret manager.

```yaml
apiVersion: external-secrets.io/v1beta1
kind: ExternalSecret
metadata:
  name: blockchain-api-keys
  namespace: blockchain
spec:
  refreshInterval: 1h
  secretStoreRef:
    kind: ClusterSecretStore
    name: cloud-secrets
  target:
    name: blockchain-secrets
    creationPolicy: Owner
  data:
    - secretKey: BITCOIN_API_ADMIN_KEY
      remoteRef:
        key: blockchain/bitcoin_api_admin_key
    - secretKey: BITCOIN_API_WALLET_KEY
      remoteRef:
        key: blockchain/bitcoin_api_wallet_key
```

## Production Checklist

Before deploying to production:

- [ ] Use specific image tags (not `latest`)
- [ ] Use immutable tags (commit SHA / semver) and enable a rollback strategy
- [ ] Configure CPU/memory requests and limits (HPA depends on requests)
- [ ] Configure namespace-level governance (LimitRanges / ResourceQuotas)
- [ ] Decide service exposure model (Ingress + TLS vs LoadBalancer)
- [ ] Configure NetworkPolicies (if your CNI enforces them) and egress controls
- [ ] Configure RBAC for humans and CI (least privilege)
- [ ] Use external secret management or sealed-secrets (avoid plaintext secrets in Git)
- [ ] Establish supply chain controls (SBOM + vulnerability scanning; optionally signing)
- [ ] Decide Redis durability/HA characteristics for rate limiting (if enabled)
- [ ] Choose StorageClasses and snapshot strategy for PVCs
- [ ] Implement backups + test restores (define RPO/RTO)
- [ ] Establish observability (metrics/logs, dashboards, alerting, on-call signals)
- [ ] Document runbooks (deploy, rollback, incident response, restore)

## CI/CD (GitHub / GitLab) — Build, Publish, Deploy

In production, “kubectl apply from a laptop” does not scale. CI/CD should produce **immutable artifacts** (container images) and apply **reviewable manifests** to the cluster in a controlled way.

At a high level, a production pipeline usually has four stages:

1. **Build**: compile + build the container image
2. **Publish**: push the image to a registry (ECR / ACR / GAR)
3. **Deploy**: apply manifests (Kustomize/Helm) with pinned image tags
4. **Verify**: check rollout status, health endpoints, and smoke tests

### GitHub Actions (typical approach)

Key ideas:

- Use **short-lived credentials** (OIDC → cloud IAM) instead of long-lived static keys.
- Push to a registry with a **content-addressable tag** (commit SHA) and optionally a semver tag.
- Deploy via `kubectl apply -k` (Kustomize) or Helm with pinned tags.

#### How GitHub Actions typically maps to “build → publish → deploy”

GitHub Actions is an event-driven workflow system. In production it’s usually organized around:

- **Triggers**: what events are allowed to deploy (push to `main`, tags like `v*`, or a manual “workflow_dispatch”).
- **Environments**: GitHub “environments” (e.g. `staging`, `production`) can enforce approvals and hold environment-scoped secrets/variables.
- **Concurrency controls**: prevent two deployments to the same env from racing (e.g. `concurrency: production`).
- **OIDC auth**: the workflow gets a short-lived identity token (`id-token: write`) and exchanges it for cloud credentials.
- **Buildx**: multi-platform builds and layer caching (faster CI and fewer “works on my machine” surprises).
- **Manifest rendering**: Kustomize/Helm turns “desired state” inputs into a specific set of Kubernetes objects.
- **Apply + verify**: apply changes, then wait for rollouts and run a small smoke test.

#### Authentication (OIDC) in practice

In production, avoid storing long-lived cloud keys in GitHub secrets. Instead:

- configure a cloud IAM role / federated identity provider that trusts GitHub’s OIDC issuer
- scope it tightly (only the repo/environment, only the needed actions)
- grant only what you need (push image + update Kubernetes)

This is the core enabling mechanism that makes GitHub Actions viable for real production deployments.

Example pipeline outline (conceptual):

```bash
# Build
docker build -t your-registry/blockchain-node:${GITHUB_SHA} -f Dockerfile .

# Push (registry-specific auth omitted here)
docker push your-registry/blockchain-node:${GITHUB_SHA}

# Deploy (example approaches)
#
# Option A (GitOps-style): update your Kustomize overlay (image tag) in Git,
# and let Argo CD / Flux reconcile the cluster.
#
# Option B (imperative): set image directly, then verify rollout.
kubectl set image statefulset/webserver blockchain-node=your-registry/blockchain-node:${GITHUB_SHA} -n blockchain
kubectl set image statefulset/miner blockchain-node=your-registry/blockchain-node:${GITHUB_SHA} -n blockchain

# Verify
kubectl rollout status statefulset/webserver -n blockchain
kubectl rollout status statefulset/miner -n blockchain
```

#### Example: end-to-end GitHub Actions workflow (build + push + deploy)

Below is an illustrative `.github/workflows/deploy.yml` that:

- builds an image tagged with the commit SHA
- pushes it to a registry
- deploys by updating the cluster (imperative `kubectl set image`)
- verifies by waiting for rollouts and hitting the health endpoint

You will need to adapt registry auth (GHCR/ECR/ACR/GAR) and cluster credentials (kubeconfig or cloud-native auth).

```yaml
name: Deploy (Kubernetes)

on:
  push:
    branches: ["main"]
  workflow_dispatch: {}

permissions:
  contents: read
  packages: write
  id-token: write # required for OIDC flows (cloud auth)

concurrency:
  group: blockchain-production
  cancel-in-progress: false

env:
  NAMESPACE: blockchain
  IMAGE_NAME: blockchain-node
  IMAGE_REGISTRY: ghcr.io/${{ github.repository_owner }}

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    environment: production

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3

      - name: Log in to GHCR
        uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Build and push image
        uses: docker/build-push-action@v6
        with:
          context: .
          file: Dockerfile
          push: true
          tags: |
            ${{ env.IMAGE_REGISTRY }}/${{ env.IMAGE_NAME }}:sha-${{ github.sha }}
          cache-from: type=gha
          cache-to: type=gha,mode=max

      # Option A: GitOps-style deploy (recommended)
      # - Commit an updated Kustomize overlay (image tag) to a config repo
      # - Let Argo CD / Flux reconcile the cluster
      #
      # Option B: Imperative deploy (shown here)
      - name: Set kubeconfig
        run: |
          mkdir -p ~/.kube
          echo "${KUBECONFIG_B64}" | base64 --decode > ~/.kube/config
        env:
          KUBECONFIG_B64: ${{ secrets.KUBECONFIG_B64 }}

      - name: Deploy (update images)
        run: |
          kubectl set image statefulset/webserver webserver=${IMAGE} -n ${NAMESPACE}
          kubectl set image statefulset/miner miner=${IMAGE} -n ${NAMESPACE}
        env:
          IMAGE: ${{ env.IMAGE_REGISTRY }}/${{ env.IMAGE_NAME }}:sha-${{ github.sha }}

      - name: Verify rollouts
        run: |
          kubectl rollout status statefulset/webserver -n ${NAMESPACE} --timeout=10m
          kubectl rollout status statefulset/miner -n ${NAMESPACE} --timeout=10m

      - name: Smoke test (health endpoint)
        run: |
          kubectl port-forward -n ${NAMESPACE} svc/webserver-service 8080:8080 &
          sleep 3
          curl -f http://localhost:8080/api/health/ready
```

**Notes on the example:**

- If you use a cloud-managed cluster, replace “Set kubeconfig” with cloud-native auth (OIDC → kube access) rather than storing kubeconfig.
- If you use Kustomize overlays, consider deploying with `kubectl apply -k overlays/production` instead of `kubectl set image ...`.

### GitLab CI (typical approach)

GitLab CI is a pipeline system built into GitLab. In production it’s usually structured around:

- **Stages** (`test` → `build` → `deploy`) with explicit artifact flow
- **Protected branches/tags** for production deployments (only maintainers can trigger)
- **Environment-scoped variables** for cluster auth and per-env config (`staging` vs `production`)
- **Runners** with the right capabilities (Docker-in-Docker or Kubernetes executor)
- **Registry auth** to GitLab Container Registry (built-in) or external registries (ECR/ACR/GAR)
- **Kubernetes auth** via kubeconfig, or preferably cloud-native OIDC/workload identity in managed clusters

A common pattern is:

- Merge Request pipelines build/test.
- Main branch pipeline builds and pushes images.
- A protected “deploy” job applies manifests to staging/prod.

#### How GitLab CI typically maps to “build → publish → deploy”

- **Test stage**: run fast checks (formatting, unit tests, `cargo test`, etc.).
- **Build stage**: build the container image and push it to a registry tagged with `$CI_COMMIT_SHA` (immutable).
- **Deploy stage**: update the cluster to point at that exact tag.

Deployment safety in GitLab commonly uses:

- `environment: name: production` with **manual approval** (or protected environments)
- `resource_group: production` to prevent concurrent prod deploys
- `kubectl rollout status ...` to block until the rollout completes (or fails)

Example pipeline outline (conceptual):

```bash
# Build/push
docker build -t $CI_REGISTRY_IMAGE/blockchain-node:$CI_COMMIT_SHA -f Dockerfile .
docker push $CI_REGISTRY_IMAGE/blockchain-node:$CI_COMMIT_SHA

# Deploy
kubectl set image statefulset/webserver blockchain-node=$CI_REGISTRY_IMAGE/blockchain-node:$CI_COMMIT_SHA -n blockchain
kubectl set image statefulset/miner blockchain-node=$CI_REGISTRY_IMAGE/blockchain-node:$CI_COMMIT_SHA -n blockchain
kubectl rollout status statefulset/webserver -n blockchain
```

#### Example: `.gitlab-ci.yml` (build + push + deploy)

This example uses the GitLab Container Registry and a kubeconfig stored as a protected, masked CI variable (`KUBECONFIG_B64`). Adapt as needed for your registry and cluster auth model.

```yaml
stages:
  - test
  - build
  - deploy

variables:
  NAMESPACE: "blockchain"
  IMAGE_NAME: "blockchain-node"
  IMAGE: "$CI_REGISTRY_IMAGE/$IMAGE_NAME:sha-$CI_COMMIT_SHA"

test:
  stage: test
  image: rust:1.78
  script:
    - cargo test --workspace

build-image:
  stage: build
  image: docker:27
  services:
    - name: docker:27-dind
      command: ["--tls=false"]
  variables:
    DOCKER_HOST: tcp://docker:2375
    DOCKER_TLS_CERTDIR: ""
  script:
    - docker login -u "$CI_REGISTRY_USER" -p "$CI_REGISTRY_PASSWORD" "$CI_REGISTRY"
    - docker build -t "$IMAGE" -f Dockerfile .
    - docker push "$IMAGE"
  only:
    - main

deploy-production:
  stage: deploy
  image:
    name: bitnami/kubectl:latest
    entrypoint: [""]
  environment:
    name: production
  resource_group: production
  when: manual
  allow_failure: false
  script:
    - mkdir -p ~/.kube
    - echo "$KUBECONFIG_B64" | base64 -d > ~/.kube/config
    - kubectl set image statefulset/webserver webserver="$IMAGE" -n "$NAMESPACE"
    - kubectl set image statefulset/miner miner="$IMAGE" -n "$NAMESPACE"
    - kubectl rollout status statefulset/webserver -n "$NAMESPACE" --timeout=10m
    - kubectl rollout status statefulset/miner -n "$NAMESPACE" --timeout=10m
  only:
    - main
```

**Notes on the example:**

- If you use managed Kubernetes, prefer cloud-native auth (OIDC federation) instead of storing kubeconfig in GitLab variables.
- If you use Kustomize overlays, you can deploy with `kubectl apply -k overlays/production` instead of `kubectl set image ...`.

### Deployment safety practices

- **Pin images** (avoid `latest`): use commit SHA or semver tags.
- **Use dry-runs** in CI for validation:

```bash
# Validate manifests/overlays against the API server without persisting changes
kubectl apply -k <your-kustomize-overlay-path> --dry-run=server
```

- **Separate environments** (staging/prod) using:
  - different namespaces, or
  - Kustomize overlays, or
  - Helm values per environment.
- **Secret handling**: avoid committing plaintext secrets.
  - Use External Secrets Operator / cloud secret managers or sealed-secrets.
- **Access control**: CI service account should have the minimum RBAC required to deploy.

## Deploying to the Cloud (AWS vs Azure vs GCP)

Kubernetes is portable, but the surrounding “platform glue” differs by provider: identity, load balancers, storage, and registry integration.

### AWS (EKS)

**Identity & access**

- Cluster auth is typically done via **IAM**, and workloads can use **IRSA** (IAM Roles for Service Accounts).
- Prefer OIDC federation (GitHub/GitLab → IAM) for CI deployments instead of long-lived AWS keys.

**Networking / ingress**

- Service type `LoadBalancer` commonly provisions an AWS load balancer.
- Many teams use the **AWS Load Balancer Controller** for ALB/NLB behavior (Ingress → ALB).

**Storage**

- Persistent volumes typically use **EBS** (block) via the EBS CSI driver.
- Check the StorageClass and access mode compatibility (`ReadWriteOnce` is common for node-local block volumes).

**Registry**

- Use **ECR**. CI must authenticate to ECR and push tagged images.

### Azure (AKS)

**Identity & access**

- AKS integrates with Azure AD; workloads can use **Workload Identity** / Managed Identity.
- Prefer federated identity for CI (OIDC) where possible.

**Networking / ingress**

- `LoadBalancer` provisions an Azure LB.
- Ingress is commonly NGINX ingress or Azure Application Gateway Ingress Controller (AGIC), depending on architecture.

**Storage**

- Persistent volumes typically use **Azure Disk** (block) or **Azure Files** (shared filesystem).
- For StatefulSets with per-pod databases, **Azure Disk** + `ReadWriteOnce` is a typical fit.

**Registry**

- Use **ACR** (Azure Container Registry). Ensure AKS is permitted to pull images (AKS ↔ ACR integration).

### Google Cloud (GKE)

**Identity & access**

- Prefer **Workload Identity** for pods and OIDC federation for CI deployments.

**Networking / ingress**

- `LoadBalancer` provisions a Google Cloud LB.
- GKE Ingress can integrate with Google Cloud Load Balancing; advanced setups often use NEGs.

**Storage**

- Persistent volumes typically use **Persistent Disk** (block) or Filestore (shared).
- For StatefulSets with `ReadWriteOnce`, PD is the usual choice.

**Registry**

- Use **Artifact Registry (GAR)** (preferred) or legacy GCR. CI authenticates and pushes tagged images.

## Advanced Topics

This chapter intentionally avoids duplicating the deep architecture and manifest walkthroughs from earlier sections. If you need the full reasoning behind headless services, StatefulSets, init containers, and storage, refer to:

- [Section 2: Architecture & Core Concepts](02-Architecture.md)
- [Section 4: Kubernetes Manifests](04-Manifests.md)
- [Section 5: Deployment & Operations](05-Deployment.md)

The “advanced topics” below are production patterns you may adopt as your deployment matures.

### GitOps (recommended in real production)

GitOps is an **operating model** where Git is the **source of truth** for your cluster’s *desired state*, and the cluster is continuously reconciled to match what’s in Git.

At a technical level, GitOps is a control loop (a reconciler) very similar in spirit to core Kubernetes controllers:

1. **Desired state** lives in Git (Kustomize overlays, Helm values, plain YAML, etc.).
2. A GitOps controller (commonly **Argo CD** or **Flux**) periodically (or via webhook) pulls that desired state.
3. The controller diffs **desired vs live** cluster state (drift detection).
4. If there is drift, it applies the minimal changes needed to converge (drift correction).

Conceptually:

```
Git (desired manifests) ──► GitOps controller ──► Kubernetes API (live state)
         ▲                         │
         └──────────── drift detection + reconciliation ────────────┘
```

This inverts the “imperative” workflow where humans/CI push changes directly with `kubectl apply` from laptops or runners. With GitOps, the cluster **pulls** from Git, and Git becomes the audit log.

- CI builds and publishes images
- Git changes (Kustomize/Helm values) represent desired state
- A controller (Argo CD / Flux) reconciles the cluster to match Git

**Advantages in production (and why they matter):**

- **Auditability**: every change is a commit/PR (who changed what, when, and why).
- **Repeatable rollouts**: the same base manifests can be promoted via overlays (dev → staging → prod) by changing only environment-specific patches (image tags, replicas, resources).
- **Fast, reliable rollback**: revert the commit (or roll back the application revision in Argo CD) and the controller converges the cluster back.
- **Drift detection**: if someone hot-fixes the cluster manually, GitOps shows it as drift and can either alert or automatically undo it (depending on policy).
- **Separation of duties**: CI can be limited to building/publishing images, while GitOps handles cluster writes with a tightly scoped service account.
- **Policy enforcement**: combined with admission controls (Kyverno/Gatekeeper), you can enforce “no latest tags”, required requests/limits, signed images, etc.

In practice, GitOps is usually paired with **Kustomize overlays** or Helm values per environment so that production changes are small, reviewable diffs rather than large YAML rewrites.

### Progressive Delivery (canary / blue-green)

For high-stakes rollouts, you can reduce risk by controlling *how* a new version receives traffic.

- **Canary deployment**: run the new version alongside the old version and route a **small percentage** of traffic to it (or route only a subset of users/requests). If metrics look good, gradually increase traffic (and/or replicas) until the canary becomes the full rollout. If anything looks wrong, roll back quickly by routing traffic back to the stable version.
- **Blue/green deployment**: maintain **two complete environments** (“blue” = current stable, “green” = new). Deploy to green, validate it, then **flip traffic** (usually by switching a Service selector, Ingress backend, or load balancer target). Rollback is often a fast flip back to blue.

These patterns are commonly implemented via an Ingress controller or service mesh because you need **traffic shaping** (percentage-based routing, header/cookie-based routing, etc.).

#### Argo Rollouts for canary deployments

[Argo Rollouts](https://argo-rollouts.readthedocs.io/) is a Kubernetes controller + CRD that replaces the “basic” Deployment rollout strategy with progressive delivery features.

Key ideas:

- You define a `Rollout` (instead of `Deployment`) with a **canary strategy**.
- The controller executes a step plan (set weight → pause → analysis → increase weight).
- Traffic shifting can be integrated with **Ingress** (NGINX/ALB), **service meshes** (Istio/Linkerd), or weighted Services (depending on your setup).
- You can attach **AnalysisRuns** to automatically promote or abort based on metrics (error rate, latency, etc.).

**Config example: minimal canary Rollout (step-based weight + pause)**

```yaml
apiVersion: argoproj.io/v1alpha1
kind: Rollout
metadata:
  name: webserver
  namespace: blockchain
spec:
  replicas: 4
  selector:
    matchLabels:
      app: webserver
  template:
    metadata:
      labels:
        app: webserver
    spec:
      containers:
        - name: webserver
          image: your-registry/blockchain-node:sha-<git-sha>
          ports:
            - containerPort: 8080
  strategy:
    canary:
      steps:
        - setWeight: 10   # 10% of traffic to the canary
        - pause: { duration: 120 }  # observe for 2 minutes
        - setWeight: 50
        - pause: { duration: 300 }
        - setWeight: 100  # full promotion
```

In production you typically pair the canary steps with automated analysis (SLO checks) so the system can stop a bad rollout before it reaches 100%.

### Policy as Code (admission controls)

Production clusters often enforce rules at admission time (before objects are created), such as:

- require images to come from approved registries
- block `latest` tags
- require resource requests/limits
- enforce “run as non-root”

Tools in this space include Kyverno and OPA Gatekeeper.

### Multi-cluster and multi-region considerations

If you run multiple clusters (or regions), decide:

- what data needs to be replicated (if anything)
- how you route traffic (DNS failover, global load balancers)
- how you handle upgrades (staged rollouts cluster-by-cluster)
  
In many systems, “multi-region” is not the first step; focus on a reliable single-region setup with tested backups and restore first.

## Troubleshooting
For general Kubernetes troubleshooting (CrashLoopBackOff, readiness probes, service access, DB lock issues, metrics-server), see the playbook in:

- `book-draft/ci/kubernetes/README.md` → **Troubleshooting**

This section focuses on production-specific failure modes.

### Image pulls fail in production clusters

Symptoms:

- `ImagePullBackOff`
- `ErrImagePull`

Common causes:

- wrong image tag (typo or deleted tag)
- private registry auth missing (`imagePullSecrets`)
- cluster nodes cannot reach the registry (egress restrictions)

Diagnose:

```bash
kubectl describe pod <pod-name> -n blockchain | sed -n '/Events:/,$p'
```

### Rollouts “succeed” but the system is unhealthy

In production, always validate *post-rollout health*, not just `kubectl rollout status`.

Minimum checks:

- health endpoints for the webserver
- service endpoints populated
- error rate and latency stable (metrics/logs)

### NetworkPolicy blocks traffic (self-inflicted outage)

If you enable NetworkPolicy, start with “observe” and apply deny/allow incrementally.

Diagnose:

```bash
kubectl get networkpolicy -n blockchain
kubectl describe networkpolicy <name> -n blockchain
```

### PVCs stuck in `Pending`

In managed Kubernetes, storage provisioning is an external dependency (CSI driver + StorageClass).

Diagnose:

```bash
kubectl get storageclass
kubectl describe pvc <pvc-name> -n blockchain
```

## Summary

This section covered:
- Production considerations and best practices
- Advanced topics (StatefulSets, service discovery, storage)
- Troubleshooting common issues

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Autoscaling](06-Autoscaling.md) | [↑ Table of Contents](#table-of-contents) | [End of Chapter 9 →](../../../README.md) |
|:---:|:---:|:---:|
| *Section 6* | *Current Section* | *End of Chapter 9* |

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

**📚 [← Section 6: Autoscaling](06-Autoscaling.md)** | **Section 7: Production & Advanced Topics** | **[End of Book →](../../../README.md)** 📚

</div>

---

*This chapter has explored production-grade Kubernetes deployment for the blockchain network. We've examined architecture and core concepts, migration from Docker Compose, Kubernetes manifests, deployment and operations, autoscaling capabilities, and production considerations with advanced topics. Kubernetes provides industry-standard container orchestration with automatic scaling, high availability, rolling updates, and production-grade operational capabilities. The deployment system transforms our development blockchain network into a scalable, resilient, production-ready system. In the next chapter, we'll explore [Chapter 10: Rust Language Guide](../../rust/README.md) to understand the comprehensive Rust language features used throughout our blockchain implementation.*

For more detailed information, see the complete guide sections on each topic.
