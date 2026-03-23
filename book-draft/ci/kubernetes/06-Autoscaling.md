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

## Chapter 23, Section 6: Autoscaling

**Part II: Deployment & Operations** | **Chapter 32: Kubernetes Deployment**

<div align="center">

**[← Chapter 31: Docker Compose](../docker-compose/01-Introduction.md)** | **Chapter 32: Kubernetes** | **[End of Book →](#)**

</div>

---

Autoscaling is Kubernetes’ way of adjusting capacity to match demand. In practice, “autoscaling” is three different problems:

- **Horizontal Pod Autoscaler (HPA)**: changes **replica count** (pods) for a workload.
- **Vertical Pod Autoscaler (VPA)**: changes **resource requests/limits** (CPU/memory) for pods.
- **Cluster Autoscaler**: changes **node count** (VMs/instances) so the cluster has enough room to schedule pods.

This project uses **HPA** (CPU + memory) to scale **miners** and **webservers**.

> **Methods involved:**
> - Webserver HPA: `10-hpa-webserver.yaml` ([Listing 23A.17](01A-Kubernetes-Code-Listings.md#listing-23a17-cikubernetesmanifests10-hpa-webserveryaml))
> - Miner HPA: `11-hpa-miner.yaml` ([Listing 23A.18](01A-Kubernetes-Code-Listings.md#listing-23a18-cikubernetesmanifests11-hpa-mineryaml))

## How to Read This Section

- If you want to **understand the mechanism**, read **How HPA Decides Replica Counts**.
- If you want to **operate a running cluster**, read **Verify HPA is Working** and **Troubleshooting**.
- If you want to **tune behavior**, read **Tuning & StatefulSet Considerations**.

## Table of Contents

- [Prerequisites](#prerequisites)
- [How HPA Decides Replica Counts (Technical)](#how-hpa-decides-replica-counts-technical)
  - [The Core Formula](#the-core-formula)
  - [Multiple Metrics: CPU + Memory](#multiple-metrics-cpu--memory)
  - [Behavior: Stabilization Windows and Policies](#behavior-stabilization-windows-and-policies)
- [How Autoscaling is Configured in This Repo](#how-autoscaling-is-configured-in-this-repo)
  - [Webserver HPA](#webserver-hpa)
  - [Miner HPA](#miner-hpa)
- [Verify HPA is Working](#verify-hpa-is-working)
- [Tuning & StatefulSet Considerations](#tuning--statefulset-considerations)
- [Troubleshooting](#troubleshooting)
- [Summary](#summary)

## Prerequisites

HPA needs two things to work reliably:

1. **A metrics pipeline**: for CPU/memory autoscaling, that’s the **Metrics API** (`metrics.k8s.io`), typically provided by `metrics-server`.
2. **Resource requests on containers**: `averageUtilization` is computed relative to requests.

### Metrics Server (Minikube)

If you are using Minikube, enable metrics-server:

```bash
minikube addons enable metrics-server
```

Then verify the Metrics API is available:

```bash
kubectl get apiservices | grep metrics
kubectl top pods -n blockchain
```

If `kubectl top ...` errors with “Metrics API not available”, your HPA will also be unable to scale.

### Resource Requests

When you use `averageUtilization`, Kubernetes computes utilization as a percentage of **requested** resources. If requests are missing, HPA will behave poorly (or not at all).

At a minimum, your containers should look like:

```yaml
resources:
  requests:
    cpu: "500m"
    memory: "512Mi"
```

## How HPA Decides Replica Counts (Technical)

HPA is a controller loop:

1. Read observed metrics (from the Metrics API or custom adapters).
2. Compute a desired replica count.
3. Write the new replica count to the target workload.

### The Core Formula

For a single “utilization” metric, HPA approximates the desired replica count as:

\[
\text{desiredReplicas} = \left\lceil \text{currentReplicas} \times \frac{\text{currentUtilization}}{\text{targetUtilization}} \right\rceil
\]

Example:

- `currentReplicas = 2`
- `currentUtilization = 85%`
- `targetUtilization = 70%`

\[
\text{desiredReplicas} = \lceil 2 \times 85/70 \rceil = \lceil 2.42 \rceil = 3
\]

### Multiple Metrics: CPU + Memory

If you specify multiple metrics (like CPU and memory), HPA computes a desired replica count **per metric** and then chooses the **maximum**. Practically:

- CPU says “3 replicas”
- memory says “5 replicas”
- HPA chooses **5**

This is why memory thresholds can dominate if you have a memory leak or high baseline memory use.

### Behavior: Stabilization Windows and Policies

HPA `behavior` controls *how fast* HPA is allowed to change replica counts.

- **stabilizationWindowSeconds (scaleDown)**: prevents oscillation by “remembering” recent desired values and scaling down conservatively.
- **policies**: caps how many pods can be added/removed per time period.
- **selectPolicy**: chooses how to apply multiple policies:
  - `Max`: pick the most aggressive (largest change)
  - `Min`: pick the most conservative

## How Autoscaling is Configured in This Repo

The autoscaling manifests live in `ci/kubernetes/manifests/`.

### Webserver HPA

**File**: `ci/kubernetes/manifests/10-hpa-webserver.yaml`

Important: the webserver is a **StatefulSet** in Kubernetes (for per-pod persistent storage), so the HPA targets a **StatefulSet**, not a Deployment.

```yaml
  scaleTargetRef:
    apiVersion: apps/v1
  kind: StatefulSet
    name: webserver
```

The webserver HPA uses **CPU and memory**:

- **CPU**: target average utilization `70%`
- **Memory**: target average utilization `80%`
- **Replicas**: `minReplicas: 1`, `maxReplicas: 10`

The scale-up/scale-down behavior is also defined (stabilization + policies) to avoid thrashing during spikes.

### Miner HPA

**File**: `ci/kubernetes/manifests/11-hpa-miner.yaml`

Miners are also a **StatefulSet**, and scale conservatively:

```yaml
  scaleTargetRef:
    apiVersion: apps/v1
    kind: StatefulSet
    name: miner
```

Key settings:

- **CPU**: target average utilization `80%`
- **Replicas**: `minReplicas: 1`, `maxReplicas: 5`
- **Scale-up policy**: add **1 miner at a time** (conservative)
- **Scale-down stabilization**: `600s` (10 minutes)

Why conservative? Miners are stateful, join the peer network, and can produce churn if scaled too aggressively.

## Verify HPA is Working

Check that HPAs exist:

```bash
kubectl get hpa -n blockchain
```

Inspect the decision-making and recent events:

```bash
kubectl describe hpa webserver-hpa -n blockchain
kubectl describe hpa miner-hpa -n blockchain
```

Verify metrics are flowing:

```bash
kubectl top pods -n blockchain
kubectl top nodes
```

If HPA is working, `kubectl describe hpa ...` will show current metrics, target metrics, and the computed desired replicas.

## Tuning & StatefulSet Considerations

Scaling a StatefulSet is not the same as scaling a stateless Deployment:

- **Stable identity**: pods are created as `webserver-0`, `webserver-1`, etc.
- **Per-pod storage**: each ordinal typically has its own PVC(s) via `volumeClaimTemplates`.
- **Scale-down order**: StatefulSets scale down from the **highest ordinal** first.
- **PVC lifecycle**: by default, PVCs may **remain** after scale-down. This is often what you want for stateful data.

Practical tuning guidelines:

- **Tune maxReplicas to your cluster capacity**: HPA can request replicas that your cluster cannot schedule.
- **Prefer conservative scale-down** for stateful workloads: reduce flapping and avoid churn.
- **Watch memory-based scaling carefully**: if memory usage is mostly “baseline + cache”, memory thresholds can keep replicas pinned high.

If you also need to scale the **cluster** (nodes), that is a different controller (Cluster Autoscaler) and is typically configured at the cloud-provider or cluster layer.

## Troubleshooting

### Symptom: `kubectl top ...` says “Metrics API not available”

Your cluster does not have metrics-server (or it’s broken).

```bash
kubectl get apiservices | grep metrics
kubectl get pods -n kube-system | grep metrics-server
```

For Minikube:

```bash
minikube addons enable metrics-server
```

### Symptom: HPA exists but never scales

Check:

- **HPA events**: `kubectl describe hpa webserver-hpa -n blockchain`
- **Workload requests**: ensure containers have `resources.requests` set
- **Target reference**: ensure `scaleTargetRef` matches a real object (`StatefulSet/webserver`, `StatefulSet/miner`)

### Symptom: Pods won’t schedule when HPA scales up

HPA can ask for more pods than the cluster can fit. Diagnose:

```bash
kubectl get events -n blockchain \
  --sort-by=.metadata.creationTimestamp | tail -n 50
kubectl describe pod <pending-pod-name> -n blockchain
```

Common causes:

- insufficient CPU/memory on nodes
- storage provisioning limits (PVCs)
- quota limits (namespaces)

## Summary

In this project, **HPA** is used to scale miners and webservers based on **CPU and memory**. The key operational dependencies are:

- a working **Metrics API** (metrics-server in local clusters)
- correct **resource requests** on containers
- realistic **maxReplicas** and behavior settings for stateful workloads

For the full manifests and their rationale, see [Section 4: Kubernetes Manifests](04-Manifests.md).

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Deployment & Operations](05-Deployment.md) | [↑ Table of Contents](#table-of-contents) | [Next Section: Production & Advanced Topics →](07-Production.md) |
|:---:|:---:|:---:|
| *Section 5* | *Current Section* | *Section 7* |

</div>
