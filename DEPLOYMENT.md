# Antediluvia Deployment Guide

This guide covers how to deploy the Antediluvia server and database using Docker.

## Prerequisites

- [Docker](https://www.docker.com/get-started) installed on your machine or server.
- [Docker Compose](https://docs.docker.com/compose/install/) (usually included with Docker Desktop).

## Local Development / Testing

We use `docker-compose` to spin up a PostgreSQL database and the Game Server together.

1.  **Build and Run**:
    ```bash
    docker-compose up --build
    ```
    This command will:
    - Build the `antediluvia_server` binary inside a Docker container.
    - Start a PostgreSQL 16 database.
    - Start the game server, connecting it to the database.

2.  **Access**:
    - **Game Server UDP**: `0.0.0.0:5000`
    - **Auth Server TCP**: `0.0.0.0:8080`
    - **Database**: `localhost:5432` (User: `admin`, Pass: `password`, DB: `antediluvia`)

3.  **Stopping**:
    ```bash
    docker-compose down
    ```
    To stop and remove volumes (reset DB):
    ```bash
    docker-compose down -v
    ```

## Production Deployment (Google Cloud Platform)

For a production setup on GCP, you can use **Google Kubernetes Engine (GKE)** or a **Compute Engine** VM. Since this is a stateful game server using UDP, a VM or a StatefulSet on GKE is recommended. **Cloud Run** is NOT recommended for the game server because it does not support persistent background processes or UDP easily (though it supports WebSockets/HTTP for the Auth server).

### Option A: Compute Engine (VM)

1.  **Create a VM**:
    - OS: Debian 12 (Bookworm) or Ubuntu 22.04 LTS.
    - Firewall: Allow UDP 5000 and TCP 8080.

2.  **Install Docker & Compose**:
    ```bash
    sudo apt-get update
    sudo apt-get install -y docker.io docker-compose
    ```

3.  **Deploy**:
    - Copy `docker-compose.yml` and `Dockerfile.server` to the VM.
    - Copy the `crates` folder and `Cargo.toml`/`Cargo.lock` to the VM.
    - Run `docker-compose up -d --build`.

### Option B: Kubernetes (GKE)

1.  **Build and Push Image**:
    ```bash
    # Authenticate with Google Container Registry
    gcloud auth configure-docker

    # Build
    docker build -t gcr.io/[PROJECT_ID]/antediluvia-server:latest -f Dockerfile.server .

    # Push
    docker push gcr.io/[PROJECT_ID]/antediluvia-server:latest
    ```

2.  **Deploy Postgres**:
    - Use **Cloud SQL** for PostgreSQL for managed persistence.
    - Update the `DATABASE_URL` environment variable in your deployment config to point to the Cloud SQL instance.

3.  **Deploy Game Server**:
    - Create a Kubernetes `Deployment` or `StatefulSet`.
    - Ensure `hostNetwork: true` or a `NodePort` service is used to expose UDP port 5000.
    - Set environment variables (`DATABASE_URL`, `SERVER_ADDR`, `SERVER_PORT`).

## Configuration

The server is configured via environment variables:

- `SERVER_ADDR`: Bind address (default `0.0.0.0`).
- `SERVER_PORT`: UDP port for game traffic (default `5000`).
- `DATABASE_URL`: Postgres connection string (e.g., `postgres://user:pass@host:5432/db`).
- `RUST_LOG`: Logging level (e.g., `info`, `debug`, `warn`).

## Database Migrations

The server automatically runs `db_pool.init_schema()` on startup. This creates the necessary tables (`players`, `world`) if they do not exist. For production schema changes, consider using `sqlx-cli` migrations properly.
