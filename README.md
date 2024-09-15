# Project Title: Scalable Graph Execution System

[![Rust CI](https://github.com/Swiftyos/AgentServer.rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Swiftyos/AgentServer.rs/actions/workflows/rust.yml)

## Table of Contents

- [Introduction](#introduction)
- [System Overview](#system-overview)
- [Architecture and Design Decisions](#architecture-and-design-decisions)
  - [1. Trigger Services](#1-trigger-services)
  - [2. Task Queue (Message Broker)](#2-task-queue-message-broker)
  - [3. Graph Processor Workers](#3-graph-processor-workers)
  - [4. Live Output Streaming Mechanism](#4-live-output-streaming-mechanism)
  - [5. WebSocket Service](#5-websocket-service)
  - [6. Data Storage](#6-data-storage)
  - [7. Monitoring and Autoscaling](#7-monitoring-and-autoscaling)
  - [8. Security and Authentication Mechanisms](#8-security-and-authentication-mechanisms)
- [Project Structure](#project-structure)
  - [Workspace Configuration](#workspace-configuration)
  - [Common Crate](#common-crate)
  - [Services](#services)
    - [Webhook Service](#webhook-service)
    - [Timer Service](#timer-service)
    - [REST Service](#rest-service)
    - [Graph Processor Worker](#graph-processor-worker)
    - [WebSocket Service](#websocket-service)
  - [Frontend](#frontend)
  - [Scripts and Configurations](#scripts-and-configurations)
  - [Docker and Kubernetes](#docker-and-kubernetes)
- [Key Packages and Dependencies](#key-packages-and-dependencies)
- [Tooling and Development Practices](#tooling-and-development-practices)
- [Setup and Build Instructions](#setup-and-build-instructions)
- [Architectural Decisions and Justifications](#architectural-decisions-and-justifications)
- [Next Steps](#next-steps)
- [Contributing](#contributing)
- [License](#license)

---

## Introduction

This project is a scalable system designed to handle the execution of **1 billion graphs** distributed among **1 million users**. The system supports graph executions triggered by webhooks, timers, or manual inputs and provides live monitoring via WebSocket connections when users manually initiate executions.

The project is developed in **Rust** using the **Tokio ecosystem** and is deployed using **Kubernetes** for orchestration and scalability. The system is composed of multiple microservices, each responsible for a specific functionality, ensuring clear separation of concerns and maintainability.

---

## System Overview

The system consists of the following components:

1. **Trigger Services**
   - Webhook Service
   - Timer Service
   - REST Service
2. **Task Queue (Message Broker)**
3. **Graph Processor Workers**
4. **Live Output Streaming Mechanism**
5. **WebSocket Service**
6. **Data Storage**
7. **Monitoring and Autoscaling**
8. **Security and Authentication Mechanisms**

---

## Architecture and Design Decisions

### 1. Trigger Services

#### a. Webhook Service

- **Functionality**: Receives incoming webhook requests, validates them, and enqueues tasks for graph execution.
- **Design Decisions**:
  - **Separation from REST Service**: The webhook service is separated from the REST service due to different functionalities, security requirements, and load characteristics. This separation allows for independent scaling, better security isolation, and maintainability.
  - **Stateless Microservice**: Enables horizontal scaling to handle high volumes of webhook triggers.
  - **Decoupling via Task Queue**: Ensures that incoming webhooks do not directly interact with the graph processor, preventing overload.

#### b. Timer Service

- **Functionality**: Schedules graph executions based on predefined time triggers.
- **Design Decisions**:
  - **Distributed Scheduler**: Utilizes a cron-like scheduler for reliability and scalability.
  - **Enqueuing Tasks**: Schedules tasks into the task queue at the appropriate times.

#### c. REST Service

- **Functionality**: Allows users to manually initiate graph executions from the frontend.
- **Design Decisions**:
  - **Separate from Webhook Service**: Due to different responsibilities and security considerations.
  - **Serving Frontend Assets**: Serves the frontend application and provides API endpoints.
  - **Live Monitoring Flag**: Enqueues tasks with a flag indicating if live monitoring is required.

### 2. Task Queue (Message Broker)

- **Purpose**: Decouples trigger services from the graph processor workers, enabling scalability and load balancing.
- **Design Decisions**:
  - **Use of Apache Kafka or RabbitMQ**:
    - **High Throughput**: Handles a massive number of tasks efficiently.
    - **Topic-Based Messaging**: Organizes tasks and supports scalable consumption.

### 3. Graph Processor Workers

- **Functionality**: Executes graphs using all available processors on the machine.
- **Design Decisions**:
  - **Stateless Design**: Workers do not maintain internal state, enabling horizontal scaling.
  - **Resource Optimization**:
    - **Multithreading/Multiprocessing**: Utilizes all CPU cores for efficient execution.
    - **Algorithm Optimization**: Ensures graph processing is as efficient as possible.
  - **Autoscaling Policies**: Scales based on CPU usage, memory usage, and task queue length.

### 4. Live Output Streaming Mechanism

- **Purpose**: Provides real-time updates to users who manually initiate graph executions.
- **Design Decisions**:
  - **Topic-Based Pub/Sub System**:
    - **Per-Execution Topics/Channels**: Each graph execution requiring live updates has a unique topic or channel.
  - **Efficient Message Routing**: Ensures only relevant messages are delivered to the appropriate WebSocket consumers.
  - **Implementation with Redis Pub/Sub**:
    - **Low Latency Messaging**: Supports real-time communication.
    - **Scalability**: Handles a large number of topics/channels efficiently.

### 5. WebSocket Service

- **Functionality**: Manages WebSocket connections and forwards live updates to users.
- **Design Decisions**:
  - **Stateless Service Instances**:
    - **Shared Session Storage**: Uses Redis to store session data, avoiding the need for sticky sessions.
    - **Horizontal Scalability**: Allows any instance to handle any user connection.
  - **Dynamic Subscription Management**:
    - **Subscribes to Relevant Topics**: Only subscribes to topics/channels relevant to connected users.
  - **Load Balancing Without Sticky Sessions**: Enables efficient distribution of connections based on current load.

### 6. Data Storage

- **Functionality**: Stores graph definitions, execution results, and logs.
- **Design Decisions**:
  - **Scalable Databases**:
    - **Graph Definitions**: Stored in databases like PostgreSQL.
    - **Execution Results**: Stored in databases or object storage systems (e.g., AWS S3).
  - **Caching with Redis**: Improves retrieval times for frequently accessed data.
  - **Data Partitioning and Sharding**: Distributes data across multiple databases for scalability.

### 7. Monitoring and Autoscaling

- **Functionality**: Monitors system performance and automatically adjusts resources.
- **Design Decisions**:
  - **Metrics Collection with Prometheus**:
    - **System Metrics**: CPU usage, memory usage, response times.
    - **Custom Metrics**: Task queue length, number of active WebSocket connections.
  - **Visualization with Grafana**: Provides dashboards for real-time monitoring.
  - **Autoscaling Policies**:
    - **Horizontal Pod Autoscaler (HPA)**: Scales microservices based on metrics.
    - **Cluster Autoscaler**: Adjusts the number of nodes in the Kubernetes cluster.
  - **Alerting Mechanisms**: Uses tools like Alertmanager to notify operators of critical issues.

### 8. Security and Authentication Mechanisms

- **Functionality**: Secures communication and ensures only authorized users can trigger and monitor graphs.
- **Design Decisions**:
  - **TLS Encryption**: Secures all inter-service communication.
  - **Authentication Protocols**:
    - **JWT Tokens**: Authenticates API requests.
    - **Webhook Validation**: Verifies signatures or tokens on incoming webhooks.
  - **Authorization Controls**: Ensures users can only access their own graphs and data.

---

## Project Structure

The project is organized as a **Rust workspace** to manage multiple services (crates) efficiently, ensuring scalability, maintainability, and clear separation of concerns. Each service corresponds to a microservice in the system architecture and can be developed, tested, and deployed independently.

### Workspace Configuration

```toml
[workspace]
members = [
    "common",
    "services/webhook_service",
    "services/timer_service",
    "services/rest_service",
    "services/graph_processor_worker",
    "services/websocket_service",
]
```

### Common Crate

- **Location**: `common/`
- **Purpose**: Contains shared code used across multiple services, such as data models, utilities, configuration management, and messaging helpers.
- **Structure**:

  ```plaintext
  common/
  ├── Cargo.toml
  └── src/
      ├── lib.rs
      ├── models/
      │   ├── mod.rs
      │   ├── user.rs
      │   ├── graph.rs
      │   └── execution.rs
      ├── utils/
      │   ├── mod.rs
      │   ├── config.rs
      │   ├── error.rs
      │   └── logging.rs
      ├── messaging/
      │   ├── mod.rs
      │   ├── kafka.rs
      │   └── redis.rs
      └── db/
          ├── mod.rs
          ├── migrations/
          │   ├── V1__init.sql
          │   ├── V2__add_indexes.sql
          │   └── ...
          ├── schema.rs
          └── models.rs
  ```

### Services

Each service is a separate binary crate within the workspace.

#### Webhook Service

- **Location**: `services/webhook_service/`
- **Purpose**: Handles incoming webhook requests, validates them, and enqueues tasks.
- **Structure**:

  ```plaintext
  webhook_service/
  ├── Cargo.toml
  └── src/
      ├── main.rs
      ├── handlers.rs
      ├── routes.rs
      ├── models.rs
      └── utils.rs
  ```

#### Timer Service

- **Location**: `services/timer_service/`
- **Purpose**: Schedules graph executions based on predefined time triggers.
- **Structure**:

  ```plaintext
  timer_service/
  ├── Cargo.toml
  └── src/
      ├── main.rs
      ├── scheduler.rs
      ├── models.rs
      └── utils.rs
  ```

#### REST Service

- **Location**: `services/rest_service/`
- **Purpose**: Provides a RESTful API for users and serves the frontend application.
- **Structure**:

  ```plaintext
  rest_service/
  ├── Cargo.toml
  └── src/
      ├── main.rs
      ├── handlers.rs
      ├── routes.rs
      ├── models.rs
      ├── utils.rs
      └── tests/
          ├── integration_tests.rs
          └── ...
  ```

#### Graph Processor Worker

- **Location**: `services/graph_processor_worker/`
- **Purpose**: Executes graphs and publishes live updates if required.
- **Structure**:

  ```plaintext
  graph_processor_worker/
  ├── Cargo.toml
  └── src/
      ├── main.rs
      ├── processor.rs
      ├── models.rs
      ├── utils.rs
      └── tests/
          ├── unit_tests.rs
          └── ...
  ```

#### WebSocket Service

- **Location**: `services/websocket_service/`
- **Purpose**: Manages WebSocket connections and forwards live updates.
- **Structure**:

  ```plaintext
  websocket_service/
  ├── Cargo.toml
  └── src/
      ├── main.rs
      ├── websocket_handlers.rs
      ├── models.rs
      └── utils.rs
  ```

### Frontend

- **Location**: `frontend/`
- **Purpose**: Contains the frontend application built with **React** and **HTMX**.
- **Structure**:

  ```plaintext
  frontend/
  ├── package.json
  ├── webpack.config.js
  ├── public/
  │   ├── index.html
  │   └── ...
  ├── src/
  │   ├── index.jsx
  │   ├── components/
  │   ├── styles/
  │   └── ...
  └── ...
  ```

### Scripts and Configurations

- **Scripts**: Located in the `scripts/` directory.
  - **`build.sh`**: Script to build all services.
  - **`deploy.sh`**: Script to deploy services to Kubernetes.
- **Configurations**: Global configurations are stored in the `configs/` directory.
  - **`config.toml`**: Configuration file for the application.

### Docker and Kubernetes

- **Dockerfiles**: Located in the `docker/` directory.
  - **`Dockerfile.service_name`**: Dockerfile for each service.
- **Kubernetes Manifests**: Located in the `k8s/` directory.
  - **`service_name_deployment.yaml`**: Deployment manifest for each service.

---

## Key Packages and Dependencies

The project utilizes a range of Rust crates, selected based on community recommendations to ensure robustness, performance, and maintainability.

### Common Dependencies

- **Serialization and Deserialization**: `serde`, `serde_json`
- **Asynchronous Runtime**: `tokio`
- **Logging and Diagnostics**: `tracing`
- **Configuration Management**: `config`
- **Error Handling**:
  - **Libraries**: `thiserror` (for the common crate)
  - **Applications**: `anyhow` (for application crates)
- **Utilities**:
  - **Lazy Initialization**: `once_cell`
  - **Regular Expressions**: `regex`
  - **UUID Generation**: `uuid`
- **Database Interaction**: `sqlx` (supports PostgreSQL)
- **Testing**:
  - **Asynchronous Testing**: `tokio-test`
  - **Snapshot Testing**: `insta`

### Service-Specific Dependencies

#### Webhook Service

- **Web Framework**: `axum`
- **Kafka Client**: `rdkafka`
- **JWT Authentication**: `jsonwebtoken`

#### Timer Service

- **Scheduling**: `tokio-cron-scheduler`
- **Date and Time Handling**: `chrono`

#### REST Service

- **Web Framework**: `axum`
- **Static File Serving**: `tower-http::services::ServeDir`
- **CORS Handling**: `tower-http::cors::CorsLayer`
- **Session Management**: `redis`

#### Graph Processor Worker

- **Graph Algorithms**: `petgraph`
- **Parallelism**: `rayon`
- **AWS S3 Interaction**: `aws-sdk-s3`

#### WebSocket Service

- **WebSocket Support**: `tokio-tungstenite`
- **Session Management and Pub/Sub**: `redis`

---

## Tooling and Development Practices

- **Linting**: `clippy` is integrated into the development workflow for linting and catching common mistakes.
- **Formatting**: `rustfmt` is used to enforce code style consistency across the project.
- **Dependency Management**: `cargo-edit` is used for efficient management of dependencies.
- **Security Auditing**: `cargo-audit` is run regularly to check for vulnerabilities in dependencies.
- **Testing Framework**: `cargo-nextest` is used for faster and more efficient test execution.
- **Benchmarking**: `criterion` is used for performance benchmarking to monitor and optimize performance.
- **Continuous Integration**: Incorporates the above tools into the CI/CD pipeline to ensure code quality and security.

---

## Setup and Build Instructions

### Prerequisites

- **Rust and Cargo**: Install from [rustup.rs](https://rustup.rs)
- **Node.js and npm**: Install from [nodejs.org](https://nodejs.org)

### Project Setup

1. **Clone the Repository**

   ```bash
   git clone https://github.com/yourusername/yourrepository.git
   cd yourrepository
   ```

2. **Install Rust Dependencies**

   Navigate to the project root and build the workspace.

   ```bash
   cargo build
   ```

3. **Set Up the Frontend**

   Navigate to the `frontend/` directory and install frontend dependencies.

   ```bash
   cd frontend
   npm install
   ```

   - Install React, HTMX, and other required packages.

4. **Configure Environment Variables**

   - Copy `configs/config.toml.example` to `configs/config.toml` and modify it according to your environment.

5. **Run Services**

   - Each service can be run individually. For example:

     ```bash
     cargo run --bin rest_service
     ```

---

## Architectural Decisions and Justifications

1. **Separation of Services**

   - **Decision**: Separate the webhook service from the REST service.
   - **Justification**:
     - **Scalability**: Allows independent scaling based on specific load patterns.
     - **Security**: Isolates external webhook handling from user-facing APIs.
     - **Maintainability**: Clear separation of concerns.

2. **Use of Message Queues**

   - **Decision**: Use message queues (e.g., Kafka) to decouple trigger services from graph processor workers.
   - **Justification**:
     - **Scalability**: Enables independent scaling of producers and consumers.
     - **Load Balancing**: Distributes tasks evenly among workers.
     - **Resilience**: Buffers tasks during spikes, preventing overload.

3. **Stateless Microservices**

   - **Decision**: Design services to be stateless where possible.
   - **Justification**:
     - **Scalability**: Easier to scale horizontally.
     - **Fault Tolerance**: Failure of one instance doesn't affect others.

4. **Efficient Live Update Mechanisms**

   - **Decision**: Implement topic-based Pub/Sub for live updates.
   - **Justification**:
     - **Relevance**: Users receive only pertinent updates.
     - **Performance**: Reduces unnecessary processing.

5. **Database Interaction**

   - **Decision**: Use `sqlx` for database interactions with compile-time query checking.
   - **Justification**:
     - **Safety**: Prevents SQL injection and runtime errors.
     - **Asynchronous Support**: Integrates well with Tokio.

6. **Frontend Integration**

   - **Decision**: Use React and HTMX for the frontend, served by the REST service.
   - **Justification**:
     - **Modern UI/UX**: Leverages popular frontend technologies.
     - **Simplified Deployment**: Serving frontend assets from the REST service simplifies architecture.

7. **Security Practices**

   - **Decision**: Implement JWT authentication and secure communication.
   - **Justification**:
     - **Data Protection**: Ensures only authorized access.
     - **Compliance**: Meets industry standards.

8. **Tooling and Best Practices**

   - **Decision**: Use community-recommended tooling for development and CI/CD.
   - **Justification**:
     - **Code Quality**: Tools like `clippy` and `rustfmt` ensure high code quality.
     - **Security**: `cargo-audit` helps identify vulnerabilities.

---

## Next Steps

- **Development**:
  - Implement core functionalities for each service.
  - Develop shared models and utilities in the common crate.
  - Build out the frontend application.

- **Testing**:
  - Write unit and integration tests using `cargo-nextest` and `insta`.
  - Set up test databases for testing database interactions.

- **Deployment**:
  - Configure Dockerfiles and Kubernetes manifests with environment-specific details.
  - Implement the `build.sh` and `deploy.sh` scripts.

- **Monitoring and Optimization**:
  - Integrate monitoring tools like Prometheus and Grafana.
  - Optimize performance based on collected metrics.

- **Documentation**:
  - Document APIs and services.
  - Provide usage examples and API references.
