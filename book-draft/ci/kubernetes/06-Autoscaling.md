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
   - **Section 6: Autoscaling** ← *You are here*
   - [Section 7: Production & Advanced Topics](07-Production.md)

</details>

</div>

<div align="right">

**[← Back to Main Book](../../../README.md)**

</div>

---

# Chapter 8, Section 6: Autoscaling

**Part II: Deployment & Operations** | **Chapter 9: Kubernetes Deployment**

<div align="center">

**📚 [← Chapter 8: Docker Compose](../docker-compose/01-Introduction.md)** | **Chapter 9: Kubernetes** | **[End of Book →](#)** 📚

</div>

---

This section explains how HPA (Horizontal Pod Autoscaler) works and how to configure autoscaling for the blockchain network.

## Overview

HPA automatically scales the number of pods based on CPU, memory, or custom metrics. This ensures your application has enough resources during high load and scales down during low load to save resources.

## HPA Configuration

### Webserver HPA

**File**: `10-hpa-webserver.yaml`

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

**Key Configuration:**
- **minReplicas**: Minimum number of pods (1)
- **maxReplicas**: Maximum number of pods (10)
- **CPU threshold**: Scale up if CPU > 70%
- **Memory threshold**: Scale up if Memory > 80%
- **Scale down**: Wait 5 minutes, scale down by 50% per minute
- **Scale up**: Scale up by 100% or 2 pods (whichever is more) every 15 seconds

### Miner HPA

**File**: `11-hpa-miner.yaml`

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

**Key Configuration:**
- **minReplicas**: Minimum number of pods (1)
- **maxReplicas**: Maximum number of pods (5)
- **CPU threshold**: Scale up if CPU > 80% (mining is CPU-intensive)
- **Conservative scaling**: 10-minute stabilization window for scale down

## How HPA Works

### Scaling Decision Process

```
1. HPA queries Metrics Server
   ↓
2. Gets current CPU/Memory usage for all pods
   ↓
3. Calculates average utilization
   ↓
4. Compares to target threshold
   ↓
5. Calculates desired replicas
   ↓
6. Updates Deployment/StatefulSet
   ↓
7. Kubernetes creates/destroys pods
```

### Example: Scale Up

**Current State:**
- 2 webserver pods
- Average CPU: 85% (target: 70%)
- Desired replicas: 3 (calculated by HPA)

**HPA Action:**
- Updates Deployment to 3 replicas
- Kubernetes creates 1 new pod
- New pod starts and joins service

### Example: Scale Down

**Current State:**
- 5 webserver pods
- Average CPU: 30% (target: 70%)
- Desired replicas: 2 (calculated by HPA)

**HPA Action:**
- Waits for stabilization window (5 minutes)
- Updates Deployment to 2 replicas
- Kubernetes terminates 3 pods (oldest first)

## Scaling Policies

### Scale Up Behavior

```yaml
scaleUp:
  stabilizationWindowSeconds: 0  # No delay
  policies:
  - type: Percent
    value: 100                   # Double pods
    periodSeconds: 15            # Every 15 seconds
  - type: Pods
    value: 2                     # Or add 2 pods
    periodSeconds: 15            # Every 15 seconds
  selectPolicy: Max              # Use whichever is more aggressive
```

**Result**: Can scale from 2 to 4 pods in 15 seconds, or from 2 to 6 pods in 30 seconds.

### Scale Down Behavior

```yaml
scaleDown:
  stabilizationWindowSeconds: 300  # Wait 5 minutes
  policies:
  - type: Percent
    value: 50                     # Reduce by 50%
    periodSeconds: 60             # Every minute
```

**Result**: Waits 5 minutes, then reduces pods by 50% each minute until target is reached.

## Metrics

### Resource Metrics

**CPU Utilization:**
- Target: 70% average across all pods
- Scale up if: Average CPU > 70%
- Scale down if: Average CPU < 70% for stabilization period

**Memory Utilization:**
- Target: 80% average across all pods
- Scale up if: Average Memory > 80%
- Scale down if: Average Memory < 80% for stabilization period

### Custom Metrics (Advanced)

You can configure HPA to use custom metrics:

```yaml
metrics:
- type: Pods
  pods:
    metric:
      name: requests_per_second
    target:
      type: AverageValue
      averageValue: "100"
```

## Monitoring HPA

### Check HPA Status

```bash
# Get HPA status
kubectl get hpa -n blockchain

# Describe HPA (detailed information)
kubectl describe hpa webserver-hpa -n blockchain
```

### Watch HPA in Action

```bash
# Watch HPA
kubectl get hpa -n blockchain -w

# Watch pods (see scaling happen)
kubectl get pods -n blockchain -w
```

### Check Metrics

```bash
# Check current resource usage
kubectl top pods -n blockchain

# Check HPA metrics
kubectl describe hpa webserver-hpa -n blockchain | grep -A 10 Metrics
```

## Troubleshooting

### HPA Not Scaling

**Check Metrics Server:**
```bash
kubectl get apiservice v1beta1.metrics.k8s.io
```

**Check Resource Usage:**
```bash
kubectl top pods -n blockchain
```

**Check HPA Status:**
```bash
kubectl describe hpa webserver-hpa -n blockchain
```

**Common Issues:**
- Metrics Server not installed
- Pods not reporting metrics
- HPA target not set correctly
- Resource limits not set on pods

### HPA Scaling Too Aggressively

**Adjust Stabilization Window:**
```yaml
behavior:
  scaleDown:
    stabilizationWindowSeconds: 600  # Increase to 10 minutes
```

**Adjust Scaling Policies:**
```yaml
scaleUp:
  policies:
  - type: Percent
    value: 50  # Reduce from 100% to 50%
    periodSeconds: 30  # Increase from 15 to 30 seconds
```

### HPA Not Scaling Down

**Check Scale Down Policies:**
```bash
kubectl describe hpa webserver-hpa -n blockchain | grep -A 10 "Scale Down"
```

**Common Causes:**
- Stabilization window too long
- Scale down policies too conservative
- Pods still using resources

## Best Practices

### 1. Set Appropriate Thresholds

- **Webservers**: CPU 70%, Memory 80% (handles traffic spikes)
- **Miners**: CPU 80% (mining is CPU-intensive)

### 2. Configure Resource Limits

Pods must have resource limits for HPA to work:

```yaml
resources:
  requests:
    cpu: "500m"
    memory: "512Mi"
  limits:
    cpu: "2000m"
    memory: "2Gi"
```

### 3. Use Stabilization Windows

Prevent rapid scaling up and down:
- Scale up: 0 seconds (respond quickly to load)
- Scale down: 5-10 minutes (prevent premature scale down)

### 4. Set Reasonable Min/Max Replicas

- **Min**: Ensure availability (at least 1-2 pods)
- **Max**: Prevent resource exhaustion (based on cluster capacity)

### 5. Monitor HPA Behavior

Regularly check HPA status and adjust thresholds based on actual usage patterns.

## Key Metrics to Monitor

### For Webservers:
- **CPU Usage**: Scale up if > 70% average
- **Memory Usage**: Scale up if > 80% average
- **Request Rate**: Monitor requests per second per instance
- **Response Time**: Monitor p95 response time

### For Miners:
- **CPU Usage**: Scale up if > 80% average
- **Memory Usage**: Scale up if > 85% average
- **Block Processing Rate**: Monitor blocks processed per second

## Summary

HPA provides:
- **Automatic Scaling**: Based on CPU, memory, or custom metrics

---

<div align="center">

**Local Navigation - Table of Contents**

| [← Previous Section: Deployment & Operations](05-Deployment.md) | [↑ Table of Contents](#) | [Next Section: Production & Advanced Topics →](07-Production.md) |
|:---:|:---:|:---:|
| *Section 5* | *Current Section* | *Section 7* |

</div>

---
- **Cost Optimization**: Scale down during low load
- **High Availability**: Scale up during high load
- **Flexible Policies**: Configure scaling behavior

For detailed HPA configuration examples, see [Chapter 4: Kubernetes Manifests](04-Manifests.md).
