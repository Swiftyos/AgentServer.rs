#!/bin/bash

# Set the root directory name
ROOT_DIR="project-root"

# Create the root directory
mkdir -p "$ROOT_DIR"

# Navigate to the root directory
cd "$ROOT_DIR"

# Initialize the workspace Cargo.toml
cat > Cargo.toml <<EOL
[workspace]
members = [
    "common",
    "services/webhook_service",
    "services/timer_service",
    "services/rest_service",
    "services/graph_processor_worker",
    "services/websocket_service",
]
EOL

# Create common crate
mkdir -p common/src/{models,utils,messaging,db}
touch common/src/lib.rs
touch common/Cargo.toml

# Write Cargo.toml for common crate
cat > common/Cargo.toml <<EOL
[package]
name = "common"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
thiserror = "1.0"
once_cell = "1.17"
regex = "1.6"
uuid = { version = "1.2", features = ["v4"] }
sqlx = { version = "0.6", features = ["postgres", "runtime-tokio-rustls", "macros"] }
config = "0.13"
async-trait = "0.1"

[features]
default = []
EOL

# Create services directory
mkdir -p services

# Function to create a service
create_service() {
  SERVICE_NAME=$1
  mkdir -p services/$SERVICE_NAME/src
  touch services/$SERVICE_NAME/src/main.rs
  touch services/$SERVICE_NAME/Cargo.toml

  # Write Cargo.toml for the service
  cat > services/$SERVICE_NAME/Cargo.toml <<EOL
[package]
name = "$SERVICE_NAME"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
axum = "0.6"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
anyhow = "1.0"
common = { path = "../../common" }
EOL

  # Add service-specific dependencies
  if [ "$SERVICE_NAME" == "webhook_service" ]; then
    cat >> services/$SERVICE_NAME/Cargo.toml <<EOL
rdkafka = { version = "0.29", features = ["tokio"] }
jsonwebtoken = "8.2"
EOL
  elif [ "$SERVICE_NAME" == "timer_service" ]; then
    cat >> services/$SERVICE_NAME/Cargo.toml <<EOL
chrono = { version = "0.4", features = ["serde"] }
tokio-cron-scheduler = "0.6"
rdkafka = { version = "0.29", features = ["tokio"] }
EOL
  elif [ "$SERVICE_NAME" == "rest_service" ]; then
    cat >> services/$SERVICE_NAME/Cargo.toml <<EOL
sqlx = { version = "0.6", features = ["postgres", "runtime-tokio-rustls", "macros"] }
jsonwebtoken = "8.2"
redis = { version = "0.23", features = ["tokio-comp"] }
tower-http = { version = "0.3", features = ["cors"] }
EOL
  elif [ "$SERVICE_NAME" == "graph_processor_worker" ]; then
    cat >> services/$SERVICE_NAME/Cargo.toml <<EOL
petgraph = "0.6"
rayon = "1.6"
rdkafka = { version = "0.29", features = ["tokio"] }
sqlx = { version = "0.6", features = ["postgres", "runtime-tokio-rustls", "macros"] }
aws-sdk-s3 = { version = "0.26", features = ["rustls"] }
EOL
  elif [ "$SERVICE_NAME" == "websocket_service" ]; then
    cat >> services/$SERVICE_NAME/Cargo.toml <<EOL
tokio-tungstenite = "0.18"
redis = { version = "0.23", features = ["tokio-comp"] }
EOL
  fi
}

# Create each service
create_service "webhook_service"
create_service "timer_service"
create_service "rest_service"
create_service "graph_processor_worker"
create_service "websocket_service"

# Create additional directories and files
mkdir -p scripts configs docker k8s frontend

# Create placeholder scripts
echo "#!/bin/bash" > scripts/build.sh
echo "#!/bin/bash" > scripts/deploy.sh
chmod +x scripts/build.sh scripts/deploy.sh

# Create a global config file
touch configs/config.toml

# Create Dockerfiles for each service
for SERVICE in webhook_service timer_service rest_service graph_processor_worker websocket_service; do
  cat > docker/Dockerfile.$SERVICE <<EOL
# Dockerfile for $SERVICE
FROM rust:1.72 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --package $SERVICE

FROM debian:buster-slim
COPY --from=builder /app/target/release/$SERVICE /usr/local/bin/$SERVICE
CMD ["$SERVICE"]
EOL
done

# Create Kubernetes manifests for each service
for SERVICE in webhook_service timer_service rest_service graph_processor_worker websocket_service; do
  touch k8s/${SERVICE}_deployment.yaml
done

# Initialize the frontend directory
cd frontend
npm init -y
mkdir -p src components public styles
touch src/index.jsx
touch public/index.html
cd ..

# Create README.md
cat > README.md <<EOL
# Project Setup

This project consists of multiple Rust services organized as a workspace.

## Services

- **webhook_service**
- **timer_service**
- **rest_service**
- **graph_processor_worker**
- **websocket_service**

## Common Crate

- Shared code and utilities used across services.

## Frontend

- Located in the \`frontend/\` directory.

## Scripts

- **build.sh**: Script to build all services.
- **deploy.sh**: Script to deploy services to Kubernetes.

## Configurations

- Global configurations are stored in the \`configs/\` directory.

## Docker and Kubernetes

- Dockerfiles are in the \`docker/\` directory.
- Kubernetes manifests are in the \`k8s/\` directory.

EOL

echo "Project setup complete!"
