# Project File Structure and Key Components in Rust

Below is the proposed project file structure for the system, written in Rust. The project is organized to support scalability, maintainability, and clear separation of concerns, following Rust's best practices. It includes `libs` and `services` directories, each containing relevant subdirectories, as well as an `infra` folder for Kubernetes setup.

---

## Root Directory Structure

```plaintext
project-root/
├── Cargo.toml              # Workspace definition
├── Cargo.lock
├── infra/                  # Infrastructure setup (Kubernetes, Docker Compose)
│   ├── k8s/                # Kubernetes manifests
│   ├── docker-compose.yml  # Optional Docker Compose file
│   └── README.md           # Instructions for infrastructure
├── libs/                   # Shared libraries
│   ├── common/             # Common utilities and models
│   ├── db/                 # Database interactions
│   ├── messaging/          # Messaging utilities
│   └── auth/               # Authentication mechanisms
└── services/               # Microservices
    ├── webhook_service/
    ├── timer_service/
    ├── rest_service/
    ├── graph_processor_worker/
    └── websocket_service/
```

---

## Detailed Directory Structure and Key Files

### **1. `libs/` Directory**

Contains shared libraries used across multiple services.

#### **a. `libs/common/`**

**Purpose:** Houses common data models, utilities, and helper functions.

**Structure:**

```plaintext
libs/common/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Exports modules
│   ├── models/          # Shared data models
│   │   ├── mod.rs
│   │   └── ...          # Model definitions
│   ├── utils/           # Utility functions
│   │   ├── mod.rs
│   │   └── ...          # Utility implementations
│   └── constants.rs     # Common constants
```

**Key Packages:**

- `serde` and `serde_json` for serialization/deserialization.
- `chrono` for date and time handling.
- `uuid` for unique identifiers.

#### **b. `libs/db/`**

**Purpose:** Provides database connection pooling and query execution.

**Structure:**

```plaintext
libs/db/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Exports database functionalities
│   ├── connection.rs    # Database connection setup
│   ├── queries/         # Database query implementations
│   │   ├── mod.rs
│   │   └── ...          # Specific query modules
│   └── models.rs        # Database models (if different from common models)
```

**Key Packages:**

- `sqlx` for asynchronous, compile-time verified SQL queries.
- `config` for managing database configuration.

#### **c. `libs/messaging/`**

**Purpose:** Abstracts messaging broker interactions.

**Structure:**

```plaintext
libs/messaging/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Exports messaging functionalities
│   ├── kafka.rs         # Kafka client implementation
│   ├── rabbitmq.rs      # RabbitMQ client implementation
│   ├── pubsub.rs        # Pub/Sub abstraction
│   └── messages/        # Message format definitions
│       ├── mod.rs
│       └── ...          # Specific message types
```

**Key Packages:**

- `rdkafka` for Kafka integration.
- `lapin` for RabbitMQ.
- `serde` for message serialization.

#### **d. `libs/auth/`**

**Purpose:** Handles authentication and authorization mechanisms.

**Structure:**

```plaintext
libs/auth/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Exports auth functionalities
│   ├── jwt.rs           # JWT token handling
│   ├── middleware.rs    # Authentication middleware for services
│   └── validators.rs    # Request validation utilities
```

**Key Packages:**

- `jsonwebtoken` for JWT handling.
- `argon2` or `bcrypt` for password hashing (if needed).

---

### **2. `services/` Directory**

Contains individual microservices, each a separate binary crate.

#### **a. `services/webhook_service/`**

**Purpose:** Receives and validates incoming webhook requests.

**Structure:**

```plaintext
services/webhook_service/
├── Cargo.toml
├── Dockerfile
├── src/
│   ├── main.rs          # Entry point
│   ├── handlers/
│   │   ├── mod.rs
│   │   └── webhook.rs   # Webhook endpoint handlers
│   ├── models/
│   │   ├── mod.rs
│   │   └── webhook.rs   # Request/response models
│   ├── middleware/
│   │   ├── mod.rs
│   │   └── auth.rs      # Authentication middleware
│   ├── config.rs        # Service configuration
│   └── routes.rs        # Route definitions
```

**Key Packages:**

- `axum` for HTTP server and routing.
- `tokio` for asynchronous runtime.
- `serde` for JSON parsing.
- `tracing` for structured logging.
- `libs/common`, `libs/messaging`, and `libs/auth` as dependencies.

#### **b. `services/timer_service/`**

**Purpose:** Schedules graph executions based on time triggers.

**Structure:**

```plaintext
services/timer_service/
├── Cargo.toml
├── Dockerfile
├── src/
│   ├── main.rs          # Entry point
│   ├── scheduler.rs     # Scheduling logic
│   ├── tasks/
│   │   ├── mod.rs
│   │   └── enqueue.rs   # Enqueueing tasks into the message queue
│   ├── config.rs        # Service configuration
│   └── utils.rs         # Utility functions
```

**Key Packages:**

- `cron` or `tokio-cron-scheduler` for scheduling tasks.
- `libs/messaging` for enqueuing tasks.
- `libs/common` and `libs/db` as dependencies.

#### **c. `services/rest_service/`**

**Purpose:** Allows users to manually trigger graph executions.

**Structure:**

```plaintext
services/rest_service/
├── Cargo.toml
├── Dockerfile
├── src/
│   ├── main.rs          # Entry point
│   ├── handlers/
│   │   ├── mod.rs
│   │   └── trigger.rs   # Handlers for manual execution
│   ├── models/
│   │   ├── mod.rs
│   │   └── trigger.rs   # Request/response models
│   ├── middleware/
│   │   ├── mod.rs
│   │   └── auth.rs      # Authentication middleware
│   ├── config.rs        # Service configuration
│   └── routes.rs        # Route definitions
```

**Key Packages:**

- Same as `webhook_service`.

#### **d. `services/graph_processor_worker/`**

**Purpose:** Executes graphs using all available processors.

**Structure:**

```plaintext
services/graph_processor_worker/
├── Cargo.toml
├── Dockerfile
├── src/
│   ├── main.rs            # Entry point
│   ├── processor.rs       # Graph processing logic
│   ├── workers/
│   │   ├── mod.rs
│   │   └── executor.rs    # Execution worker implementations
│   ├── models/
│   │   ├── mod.rs
│   │   └── task.rs        # Task models
│   ├── config.rs          # Service configuration
│   └── utils.rs           # Utility functions
```

**Key Packages:**

- `tokio` for asynchronous execution and multithreading.
- `rayon` for data parallelism (if needed).
- `libs/db` for accessing graph definitions.
- `libs/messaging` for consuming tasks and publishing updates.
- `serde` for serialization/deserialization.

#### **e. `services/websocket_service/`**

**Purpose:** Manages WebSocket connections for live output streaming.

**Structure:**

```plaintext
services/websocket_service/
├── Cargo.toml
├── Dockerfile
├── src/
│   ├── main.rs            # Entry point
│   ├── handlers/
│   │   ├── mod.rs
│   │   └── websocket.rs   # WebSocket connection handlers
│   ├── models/
│   │   ├── mod.rs
│   │   └── messages.rs    # Message formats
│   ├── middleware/
│   │   ├── mod.rs
│   │   └── auth.rs        # Authentication middleware
│   ├── config.rs          # Service configuration
│   └── utils.rs           # Utility functions
```

**Key Packages:**

- `axum` with WebSocket support.
- `tokio-tungstenite` or `tokio-websockets` for WebSocket management.
- `libs/messaging` for subscribing to topics.
- `redis` or `kafka` client for pub/sub mechanisms.
- `libs/auth` for authentication.

---

### **3. `infra/` Directory**

Contains infrastructure setup, including Kubernetes manifests and other deployment configurations.

**Structure:**

```plaintext
infra/
├── k8s/
│   ├── deployments/
│   │   ├── webhook_service.yaml
│   │   ├── timer_service.yaml
│   │   └── ...
│   ├── services/
│   │   ├── webhook_service_svc.yaml
│   │   ├── timer_service_svc.yaml
│   │   └── ...
│   ├── configmaps/
│   │   └── common_config.yaml
│   ├── secrets/
│   │   └── common_secrets.yaml
│   └── namespaces.yaml
├── docker-compose.yml    # Optional, for local development
└── README.md             # Instructions and documentation
```

---

## Key Rust Packages for Each Service

### **Common Across Services**

- **`axum`**: Web framework for building HTTP servers and routes.
- **`tokio`**: Asynchronous runtime for handling async I/O and multithreading.
- **`serde`, `serde_json`**: Serialization and deserialization of data structures.
- **`tracing`**: Instrumentation for application-level tracing and logging.
- **`dotenv`** or **`config`**: Configuration management.
- **`anyhow` or `thiserror`**: Error handling.

### **Service-Specific Packages**

- **Database Interaction** (`libs/db`):

  - **`sqlx`**: Async database interaction with support for compile-time checked queries.

- **Messaging** (`libs/messaging`):

  - **`rdkafka`**: Kafka client for Rust.
  - **`lapin`**: AMQP client library for RabbitMQ.
  - **`redis`**: Redis client for pub/sub.

- **Authentication** (`libs/auth`):

  - **`jsonwebtoken`**: Handling JWT tokens.
  - **`argon2` or `bcrypt`**: Password hashing algorithms.

- **WebSocket Management** (`services/websocket_service`):

  - **`tokio-tungstenite`**: Asynchronous WebSocket library.
  - **`axum-extra`**: Additional Axum utilities, possibly including WebSocket support.

- **Scheduling** (`services/timer_service`):

  - **`tokio-cron-scheduler`**: Cron-like job scheduling for Tokio.

- **Graph Processing** (`services/graph_processor_worker`):

  - **`petgraph`**: Graph data structure library.
  - **`rayon`**: Data parallelism library for CPU-bound tasks.

---

## Database Models, Queries, and Messaging Formats

### **Database Models and Queries**

- **Location**: Placed in `libs/db/src/models/` and `libs/db/src/queries/`.
- **Models**: Define structs that map to database tables using `sqlx::FromRow`.
- **Queries**: Use `sqlx::query!` or `sqlx::query_as!` macros for compile-time checked queries.

**Example:**

```rust
// libs/db/src/models/graph.rs
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Graph {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub definition: serde_json::Value,
    pub created_at: chrono::NaiveDateTime,
    // ...
}
```

```rust
// libs/db/src/queries/graph_queries.rs
use sqlx::PgPool;
use crate::models::graph::Graph;

pub async fn get_graph_by_id(pool: &PgPool, graph_id: uuid::Uuid) -> Result<Graph, sqlx::Error> {
    sqlx::query_as!(
        Graph,
        "SELECT * FROM graphs WHERE id = $1",
        graph_id
    )
    .fetch_one(pool)
    .await
}
```

### **Request and Response Models**

- **Location**: In each service under `src/models/`.
- **Purpose**: Define the data structures for HTTP requests and responses.
- **Use `serde`**: Annotate structs with `#[derive(Serialize, Deserialize)]`.

**Example:**

```rust
// services/rest_service/src/models/trigger.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerRequest {
    pub graph_id: uuid::Uuid,
    pub live_monitoring: bool,
    // ...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerResponse {
    pub execution_id: uuid::Uuid,
    pub status: String,
    // ...
}
```

### **Messaging Formats**

- **Location**: Under `libs/messaging/src/messages/`.
- **Purpose**: Define the structure of messages sent through the message broker.
- **Use `serde`**: For serialization into JSON or other formats supported by the broker.

**Example:**

```rust
// libs/messaging/src/messages/task.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskMessage {
    pub execution_id: uuid::Uuid,
    pub graph_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub live_monitoring: bool,
    // ...
}
```

---

## Structuring the Project for Rapid Development

- **Modular Design**: Separating code into `libs` and `services` allows for reusability and cleaner code management.
- **Shared Libraries**: Common functionalities are abstracted in `libs`, reducing duplication.
- **Workspace Setup**: Using a Cargo workspace simplifies dependency management and building multiple crates.
- **Docker Integration**: Each service includes a `Dockerfile` for containerization, facilitating consistent deployment.
- **Infrastructure as Code**: `infra` directory contains all deployment configurations, promoting DevOps best practices.
- **Consistent Coding Standards**: Following Rust's conventions (e.g., module organization, error handling) ensures code quality.
- **Automated Testing**: Implement unit and integration tests within each crate to maintain reliability.

---

## Dockerfile Placement

- **Location**: Each service has its own `Dockerfile` at the root of its directory.
- **Purpose**: Builds a Docker image for the service, ensuring consistent runtime environments.

**Example `Dockerfile` for `webhook_service`:**

```dockerfile
# Start from the official Rust image
FROM rust:1.70 as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Copy the service and libs directories
COPY services/webhook_service ./services/webhook_service
COPY libs ./libs

# Build the binary
RUN cd services/webhook_service && cargo build --release

# Use a minimal base image for the final artifact
FROM debian:buster-slim

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/webhook_service /usr/local/bin/webhook_service

# Expose the service port
EXPOSE 8080

# Set the entrypoint
ENTRYPOINT ["webhook_service"]
```

---

## Kubernetes Setup in `infra/`

- **Deployment Manifests**: YAML files defining Deployments, Services, ConfigMaps, and Secrets.
- **Namespace Segregation**: Organize resources under specific namespaces for better management.
- **ConfigMaps and Secrets**: Externalize configuration and sensitive data from the application code.
- **Horizontal Pod Autoscaler**: Configure autoscaling policies based on CPU/memory usage or custom metrics.
- **Monitoring Tools**: Include manifests for Prometheus and Grafana for monitoring.

**Example Deployment for `webhook_service`:**

```yaml
# infra/k8s/deployments/webhook_service.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: webhook-service
  labels:
    app: webhook-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: webhook-service
  template:
    metadata:
      labels:
        app: webhook-service
    spec:
      containers:
        - name: webhook-service
          image: your-docker-repo/webhook-service:latest
          ports:
            - containerPort: 8080
          envFrom:
            - configMapRef:
                name: common-config
            - secretRef:
                name: common-secrets
          resources:
            requests:
              cpu: "100m"
              memory: "128Mi"
            limits:
              cpu: "500m"
              memory: "256Mi"
```

---

## Ensuring Best Practices

- **Error Handling**: Use `Result` and `?` operator for propagating errors.
- **Logging and Tracing**: Implement structured logging with `tracing` crate.
- **Concurrency**: Leverage `tokio` for async I/O and tasks.
- **Security**: Enforce TLS, validate inputs, and handle authentication properly.
- **Code Reviews**: Regularly review code to maintain standards.
- **Continuous Integration**: Set up CI pipelines for building and testing code automatically.

---

## Conclusion

By structuring the project with clear separation of concerns, shared libraries, and consistent patterns, developers can rapidly build and maintain the system. Utilizing Rust's powerful type system, async capabilities, and modern tooling, the project is set up for high performance and scalability.

- **`libs` Directory**: Contains reusable code, promoting DRY principles.
- **`services` Directory**: Each microservice is isolated, making it easier to develop and deploy independently.
- **Infrastructure as Code**: The `infra` directory allows for seamless deployment and scaling.
- **Adherence to Best Practices**: Following Rust conventions ensures code quality and reliability.

This setup not only accelerates development but also ensures that the system is robust, secure, and maintainable in the long term.