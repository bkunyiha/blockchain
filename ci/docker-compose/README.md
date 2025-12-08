# Docker Compose Deployment

This directory contains all Docker Compose-related files for deploying the blockchain network.

## Directory Structure

```
docker-compose/
├── configs/                # Configuration files
│   ├── docker-compose.yml                  # Main compose file
│   ├── docker-compose.miner.yml            # Miner-only compose file
│   ├── docker-compose.webserver.yml        # Webserver-only compose file
│   ├── Dockerfile                          # Docker image definition
│   ├── .dockerignore                       # Docker ignore file
│   ├── docker-entrypoint.sh                # Container entrypoint script
│   ├── wait-for-node.sh                    # Node wait script
│   ├── docker-compose.scale.sh             # Scaling helper script
│   ├── scale-up.sh                         # Incremental scale up
│   ├── scale-down.sh                       # Incremental scale down
│   └── generate-compose-ports.sh           # Port mapping generator
└── README.md               # This quick start guide
```

## Quick Start

### Default Setup (1 miner + 1 webserver)

```bash
cd configs
docker-compose up -d
```

### Scale to Multiple Instances

```bash
cd configs
./docker-compose.scale.sh 3 2  # 3 miners, 2 webservers
```

## Documentation

**Complete Book Documentation**: See [`../../book-draft/ci/docker-compose/`](../../book-draft/ci/docker-compose/) for comprehensive chapter-by-chapter guide.

**All Chapters:**
- **[Chapter 1: Introduction & Quick Start](../../book-draft/ci/docker-compose/01-Introduction.md)** - Complete Docker Compose guide with quick start, examples, and troubleshooting
- **[Chapter 2: Architecture & Container System](../../book-draft/ci/docker-compose/02-Architecture.md)** - Container naming, instance detection, volumes, and data directories
- **[Chapter 3: Execution Flow & Startup Process](../../book-draft/ci/docker-compose/03-Execution-Flow.md)** - Complete code execution order from Docker Compose to blockchain binary
- **[Chapter 4: Network Configuration](../../book-draft/ci/docker-compose/04-Network-Configuration.md)** - Node connections, miner connection chain, and network topology
- **[Chapter 5: Port Mapping & External Access](../../book-draft/ci/docker-compose/05-Port-Mapping.md)** - Port mapping details, scaling helper script, and external access strategies
- **[Chapter 6: Scaling & Deployment](../../book-draft/ci/docker-compose/06-Scaling.md)** - Scaling methods comparison, incremental scaling, and data persistence
- **[Chapter 7: Sequential Startup](../../book-draft/ci/docker-compose/07-Sequential-Startup.md)** - Sequential startup mechanism, health checks, and wait script behavior
- **[Chapter 8: Deployment Scenarios](../../book-draft/ci/docker-compose/08-Deployment-Scenarios.md)** - Common deployment scenarios, examples, and best practices

## Key Features

- **Multi-instance scaling**: Run multiple miners and webservers
- **Automatic port mapping**: All instances accessible externally
- **Sequential startup**: Nodes wait for previous nodes
- **Isolated data**: Each instance has its own data directory
- **Health checks**: Built-in health monitoring

