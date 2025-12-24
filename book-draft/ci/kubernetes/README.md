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
   - [What You’ll Build](#what-youll-build)
   - [Why Kubernetes (vs Docker Compose)](#why-kubernetes-vs-docker-compose)
   - [Prerequisites & Tools](#prerequisites--tools)
   - [Choose Your Cluster (Minikube vs kind)](#choose-your-cluster-minikube-vs-kind)
   - [Quick Start (Minikube)](#quick-start-minikube)
   - [Accessing the Webserver (Port-Forward)](#accessing-the-webserver-port-forward)
   - [Rate Limiting (Webserver)](#rate-limiting-webserver)
   - [Common Operations](#common-operations)
   - [Cleanup](#cleanup)
   - [Troubleshooting](#troubleshooting)
2. [Section 2: Architecture & Core Concepts](02-Architecture.md)
3. [Section 3: Migration Guide](03-Migration.md)
4. [Section 4: Kubernetes Manifests](04-Manifests.md)
5. [Section 5: Deployment & Operations](05-Deployment.md)
6. [Section 6: Autoscaling](06-Autoscaling.md)
7. [Section 7: Production & Advanced Topics](07-Production.md)

---

## Section 1: Introduction & Quick Start

### What You’ll Build

In this section we’ll take our network from “it works on my laptop” to “it runs under a real orchestrator.”
Concretely, you will deploy three building blocks:

- **Miners** as a **StatefulSet** (stable identity and stable storage per miner)
- **Webservers** as a **StatefulSet** (each webserver has its own blockchain DB and wallets)
- **Redis** as a small in-cluster service used by the webserver for **rate limiting** (shared state for `axum_rate_limiter`)

By the end, you’ll be able to `kubectl port-forward` the webserver service and use the API from your machine.

### Why Kubernetes (vs Docker Compose)

Docker Compose is an excellent **local development** tool, but Kubernetes is built for **long-running** systems:

| Feature | Docker Compose | Kubernetes |
|---------|---------------|------------|
| **Self-healing** | Limited | ✅ Restarts and reschedules failed pods |
| **Rolling updates** | Manual | ✅ Built-in rollout/rollback mechanisms |
| **Service discovery** | Docker DNS | ✅ Cluster DNS + Services |
| **Autoscaling** | Manual | ✅ HPA (Horizontal Pod Autoscaler) |
| **Stateful workloads** | Possible | ✅ First-class StatefulSets + PVC templates |

In this repo, Kubernetes matters most because **miners and webservers are stateful** (disk-backed databases and wallets), and Kubernetes gives us the right primitives to manage that cleanly.

### Prerequisites & Tools

You need three things:

- **`kubectl`**: talk to the cluster (apply YAML, inspect state, logs).
- **A local cluster**: Minikube (recommended for this chapter) or kind.
- **Docker**: build the application image.

Optional but useful:

- **Kustomize**: patch manifests per environment (`kubectl apply -k`).
- **kubectx/kubens**: quick context/namespace switching.

#### Install (macOS via Homebrew)

```bash
brew install kubectl
brew install minikube
brew install docker  # Docker Desktop is the usual choice on macOS
```

Optional:

```bash
brew install kubectx
```

#### Install (Linux, generic)

Install kubectl (official):

```bash
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
chmod +x kubectl
sudo mv kubectl /usr/local/bin/
kubectl version --client
```

Install Minikube (official):

```bash
curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64
sudo install minikube-linux-amd64 /usr/local/bin/minikube
minikube version
```

### Choose Your Cluster (Minikube vs kind)

Both run a real Kubernetes API server locally.

- **Minikube**: best “learning cluster” and the smoothest workflow for this repo (we build the Docker image *into* the cluster).
- **kind**: best “fast disposable cluster”, great for CI-like workflows (you build normally, then load the image into the cluster).

kind image workflow example:

```bash
docker build -t blockchain-node:latest .
kind load docker-image blockchain-node:latest
```

### Quick Start (Minikube)

This is the shortest path to a working cluster.

#### 1) Start the cluster

```bash
cd ci/kubernetes
minikube start --cpus=4 --memory=3072mb --addons=metrics-server
```

Minikube **addons** are optional cluster components you can enable at cluster creation time (or later). They’re a convenient way to turn on common Kubernetes services without manually installing Helm charts or raw manifests.

- **Why we enable `metrics-server`**: it provides the Kubernetes **Metrics API** (`metrics.k8s.io`), which powers commands like `kubectl top pods` and supports CPU/memory-based autoscaling via HPA in many setups.
- **How to list available addons**:

```bash
minikube addons list
```

Some commonly useful addons (depending on your goals):

- **`metrics-server`**: enables `kubectl top ...` and provides CPU/memory metrics to the cluster.
- **`ingress`**: enables an Ingress controller (handy when you want HTTP routing without port-forward).
- **`dashboard`**: enables the Kubernetes Dashboard UI (visual cluster inspection).
- **`registry`**: runs a local container registry inside Minikube (useful if you want to push/pull images instead of `minikube docker-env`).
- **`csi-hostpath-driver`**: storage driver used by some Minikube configurations for PVCs (often enabled by default depending on version/driver).

To enable and access the Dashboard:

```bash
minikube addons enable dashboard

# Launches the dashboard and opens your browser (or prints a URL, depending on environment)
minikube dashboard

# If you prefer a URL you can open manually:
minikube dashboard --url
```

<details>
<summary><b>Optional deep dive: what did <code>minikube start</code> change?</b></summary>

- It creates (or reuses) a local cluster and allocates CPU/memory for it.
- It updates your kubeconfig (`~/.kube/config`) so `kubectl` can talk to Minikube (context usually becomes `minikube`).
- It stores Minikube’s internal cluster state under `~/.minikube/`.

Useful commands:

```bash
minikube status                 # Shows whether Minikube and its Kubernetes components are running
kubectl config current-context  # Shows which cluster/context kubectl is currently targeting (should be `minikube`)
kubectl get nodes               # Lists cluster nodes and their readiness; confirms the cluster is schedulable
minikube ip                     # Prints the Minikube node IP (useful for NodePort access and debugging networking)
```

</details>

#### 2) Build the image into Minikube

```bash
# Use Minikube's Docker
eval $(minikube docker-env)

# Build image
# IMPORTANT: run the build from the repository root (build context),
# because the Dockerfile uses COPY paths like `ci/docker-compose/configs/...`.
cd ../../
docker build -t blockchain-node:latest -f Dockerfile .
cd ci/kubernetes/manifests
```

<details>
<summary><b>Optional deep dive: what does <code>eval $(minikube docker-env)</code> do?</b></summary>

It points your `docker` CLI at the Docker daemon *inside* Minikube, so `docker build ...` stores the image in the cluster node’s local image cache.

Without this, your `docker` CLI will still talk to your **local** Docker daemon (e.g. Docker Desktop on your laptop). In that case, the image you build exists only on your machine, and the Minikube node won’t be able to use it unless you push it to a registry (or otherwise load it into the cluster).

Once you’ve built the image, you usually want to “switch Docker back” to normal. Otherwise, later `docker ...` commands will still be targeting Minikube’s internal Docker daemon, which can be confusing (your normal local images/containers won’t appear, and you might wonder where they “went”).

This is why we run:

```bash
eval $(minikube docker-env -u)
```

`minikube docker-env -u` prints shell commands that **unset** the environment variables (`DOCKER_HOST`, TLS paths, etc.) that were redirecting your Docker CLI to Minikube. `eval $(...)` applies those unsets to **this terminal session only**.

It’s safe to run because it doesn’t delete images or change Minikube itself—it simply restores your shell back to using your usual local Docker daemon, which is what the rest of this chapter assumes.

</details>

#### 3) (Optional) Update secrets

Edit `03-secrets.yaml` to set your API keys, or use defaults for testing.

If you don’t provide a mining address (`MINER_ADDRESS`), the container will **auto-create** one on first startup and persist it in the wallet volume. This keeps the “quick start” smooth while still producing a real address you can later back up.

If you edit `03-secrets.yaml`, apply the Secret and restart the pods so the new values are injected as environment variables:

```bash
# From the repo root:
cd ci/kubernetes/manifests

# Apply the updated Secret
kubectl apply -f 03-secrets.yaml

# Restart workloads to pick up updated Secret env vars
kubectl rollout restart statefulset/webserver -n blockchain
kubectl rollout restart statefulset/miner -n blockchain
```

Optional verification:

```bash
kubectl get secret blockchain-secrets -n blockchain
```

#### 4) Deploy

```bash
# From the repo root:
cd ci/kubernetes/manifests
./deploy.sh
```

`./deploy.sh` is a convenience wrapper around a sequence of `kubectl` commands. It:

- Validates `kubectl` is installed and you’re connected to a cluster (`kubectl cluster-info`)
- Applies the manifests in dependency order (namespace → config → storage → Redis → workloads → services → autoscalers → PDB)
- Waits for miner/webserver pods to become Ready, then prints a status summary

**Commands it runs (high level):**

- `kubectl apply -f 01-namespace.yaml`
- `kubectl apply -f 02-configmap.yaml`
- `kubectl apply -f 14-configmap-rate-limit.yaml`
- `kubectl apply -f 03-secrets.yaml`
- `kubectl apply -f 04-pvc-miner.yaml`
- `kubectl apply -f 05-pvc-webserver.yaml`
- `kubectl apply -f 15-redis.yaml`
- `kubectl apply -f 06-statefulset-miner.yaml`
- `kubectl apply -f 07-deployment-webserver.yaml` (webserver runs as a StatefulSet)
- `kubectl apply -f 08-service-miner-headless.yaml`
- `kubectl apply -f 08-service-miner.yaml`
- `kubectl apply -f 09-service-webserver.yaml`
- `kubectl apply -f 10-hpa-webserver.yaml`
- `kubectl apply -f 11-hpa-miner.yaml`
- `kubectl apply -f 12-pod-disruption-budget.yaml`
- `kubectl wait --for=condition=ready pod -l app=miner -n blockchain --timeout=300s`
- `kubectl wait --for=condition=ready pod -l app=webserver -n blockchain --timeout=300s`

#### 5) Verify

```bash
kubectl get pods -n blockchain

kubectl wait --for=condition=ready pod -l app=miner -n blockchain --timeout=300s
kubectl wait --for=condition=ready pod -l app=webserver -n blockchain --timeout=300s
```

#### Useful Kubernetes Commands (Quick Reference)

Tip: these are grouped as copy/paste blocks. In most Markdown renderers you can click the code block’s copy button to copy the whole group.

**Status (what’s running?)**

```bash
# List all namespaces (cluster-wide)
kubectl get namespaces

# Pod status + node/IP (best first snapshot)
kubectl get pods -n blockchain -o wide

# Services (stable names/ports inside the cluster)
kubectl get svc -n blockchain

# Which pod IPs each Service routes to
kubectl get endpoints -n blockchain

# PersistentVolumeClaims (what storage is bound)
kubectl get pvc -n blockchain
```

**Debug a single pod**

```bash
# Events, probe failures, image pull errors, scheduling issues
kubectl describe pod <pod> -n blockchain

# Recent logs
kubectl logs -n blockchain <pod> -c blockchain-node --tail 200

# Logs from the last crashed instance (CrashLoopBackOff)
kubectl logs -n blockchain <pod> -c blockchain-node --previous --tail 200

# Shell inside the container
kubectl exec -n blockchain -it <pod> -- sh
```

**Rollouts / restarts**

```bash
# Watch rollout progress
kubectl rollout status statefulset/webserver -n blockchain

# Restart to pick up ConfigMap/Secret env var changes
kubectl rollout restart statefulset/webserver -n blockchain
```

**Events (often the fastest root cause)**

```bash
kubectl get events -n blockchain --sort-by=.lastTimestamp | tail -50
```

### Accessing the Webserver (Port-Forward)

```bash
kubectl port-forward -n blockchain svc/webserver-service 8080:8080
```

Then open http://localhost:8080 in your browser.

### Rate Limiting (Webserver)

The Kubernetes manifests deploy rate limiting “ready to go” (see `ci/kubernetes/manifests/`):

- **Redis**: `ci/kubernetes/manifests/15-redis.yaml` (service name `redis`, port 6379)
- **Settings**: `ci/kubernetes/manifests/14-configmap-rate-limit.yaml` (mounts `Settings.toml` into the webserver)
- **Env var**: `RL_SETTINGS_PATH=/app/Settings.toml`

To change limits/strategies, edit `ci/kubernetes/manifests/14-configmap-rate-limit.yaml`, apply it, and restart webserver pods:

```bash
# From the repo root:
cd ci/kubernetes/manifests
kubectl apply -f 14-configmap-rate-limit.yaml
kubectl rollout restart statefulset/webserver -n blockchain
```

### Common Operations

These are the commands you’ll use constantly once the cluster is up.

#### Scale Manually

```bash
# Scale webservers to 5 (webserver is a StatefulSet)
kubectl scale statefulset webserver -n blockchain --replicas=5

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
kubectl rollout restart statefulset/webserver -n blockchain
kubectl rollout restart statefulset/miner -n blockchain
```

#### Update Image

```bash
# Set new image
kubectl set image statefulset/webserver blockchain-node=blockchain-node:v1.1.0 -n blockchain

# Check rollout
kubectl rollout status statefulset/webserver -n blockchain

# Rollback if needed
kubectl rollout undo statefulset/webserver -n blockchain
```

### Cleanup

```bash
# From the repo root:
cd ci/kubernetes/manifests
./undeploy.sh

# Or delete everything
kubectl delete namespace blockchain

# Stop the local cluster (keeps the cluster on disk)
minikube stop

# Or remove it completely
minikube delete
```

### Troubleshooting

This section is written as a practical playbook: start with the symptom you see, run the “diagnose” commands, then apply the fix.

#### First: Get a High-Signal Snapshot

These commands answer “what is broken” in under 10 seconds:

```bash
kubectl get pods -n blockchain -o wide
kubectl get svc -n blockchain
kubectl get pvc -n blockchain
```

#### Symptom: `CrashLoopBackOff`

**What it means:** the container starts, exits, Kubernetes restarts it, and repeats.

**Diagnose:**

```bash
# "previous" shows logs from the last crashed container instance (most useful for crash loops)
kubectl logs -n blockchain <pod-name> -c blockchain-node --previous --tail 200

# Events and probe failures
kubectl describe pod <pod-name> -n blockchain
```

**Common causes in this project:**

- **Invalid `MINER_ADDRESS` (placeholder)**
  - Symptom: miner logs show address validation failures (e.g. “invalid character”).
  - Fix: remove/omit the placeholder value and let the entrypoint auto-generate, or set a real address.
  - Verify the secret value:

```bash
kubectl get secret blockchain-secrets -n blockchain -o jsonpath='{.data.MINER_ADDRESS}' | base64 --decode; echo
```

#### Symptom: `Running` but `0/1` (Not Ready)

**What it means:** the container is running, but readiness probes fail, so Kubernetes will not route traffic to it.

**Diagnose:**

```bash
kubectl describe pod <pod-name> -n blockchain
```

Look for probe errors like:
- `Readiness probe failed: connect: connection refused` (process not listening)
- HTTP probe returns non-200 (server not ready yet)

**Common cause we hit:** ports mismatched between container behavior and Kubernetes probes.

- In Kubernetes, pods should generally listen on **stable in-pod ports** (e.g. miner P2P `2001`, webserver HTTP `8080`).
- If a pod listens on a different port, probes will fail even if the process is healthy.

#### Symptom: Webserver crashes with Sled DB lock errors

If you see errors like:

> `could not acquire lock on "/app/data/.../db": Resource temporarily unavailable`

**What it means:** multiple pods are trying to open the same on-disk Sled database concurrently (shared filesystem path).

**Fix (recommended):** ensure each webserver has **its own storage**, just like miners. In this repo we run webservers as a **StatefulSet** with per-pod PVCs. You can confirm per-pod PVCs exist:

```bash
kubectl get pvc -n blockchain | grep webserver
```

#### Symptom: Old webserver pods keep crashlooping after an upgrade

If you migrated webserver from a Deployment to a StatefulSet, you may briefly have both running.

**Diagnose:**

```bash
kubectl get deployment -n blockchain
kubectl get statefulset -n blockchain
```

**Fix:** delete the old Deployment so only the StatefulSet remains:

```bash
kubectl delete deployment webserver -n blockchain
```

#### Services Not Accessible (from your laptop)

**Why it happens:** cluster networking is not directly reachable from your host by default. For Minikube/local dev, the simplest path is port-forwarding.

**Fix (local):**

```bash
kubectl port-forward -n blockchain svc/webserver-service 8080:8080
curl -f http://localhost:8080/api/health/ready
```

**Diagnose inside the cluster:**

```bash
kubectl get endpoints -n blockchain
kubectl exec -n blockchain <pod-name> -- curl -f http://webserver-service:8080/api/health/ready
```

#### HPA Not Working

```bash
# Check HPA status
kubectl describe hpa webserver-hpa -n blockchain

# Check resource usage
kubectl top pods -n blockchain
```

If `kubectl top ...` fails, you likely don’t have metrics available in your cluster (e.g. `metrics-server`). In Minikube you can enable it:

```bash
minikube addons enable metrics-server
```

Then verify the Metrics API is actually up (it can take a minute):

```bash
# Confirm the metrics-server pod is running
kubectl get pods -n kube-system | grep metrics-server

# Confirm the Metrics API endpoint is registered and Available=True
kubectl get apiservices | grep metrics
```

Once those look healthy, retry:

```bash
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

## Section 4: Kubernetes Manifests

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

| [↑ Table of Contents](#table-of-contents) | [Next Section: Architecture & Core Concepts →](02-Architecture.md) |
|:---:|:---:|:---:|
| *Current Section* | *Section 2* |

</div>

---

## Additional Resources

- [Kubernetes Official Documentation](https://kubernetes.io/docs/)
- [Kubernetes HPA Documentation](https://kubernetes.io/docs/tasks/run-application/horizontal-pod-autoscale/)
- [StatefulSets Documentation](https://kubernetes.io/docs/concepts/workloads/controllers/statefulset/)
