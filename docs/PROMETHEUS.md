To expose Prometheus metrics for your Axum server, including system metrics, you'll need to integrate Prometheus instrumentation into your application. Below is a step-by-step guide to help you set this up.

---

## **1. Add Necessary Dependencies**

First, ensure you have the following dependencies in your `Cargo.toml` file:

```toml
[dependencies]
axum = "0.6"                     # Web framework
prometheus = "0.14"              # Prometheus client library
sysinfo = "0.29"                 # System information library
tokio = { version = "1", features = ["full"] }  # Asynchronous runtime
```

*Note:* Always check [crates.io](https://crates.io/) for the latest versions.

---

## **2. Initialize Prometheus Metrics**

Create a Prometheus `Registry` to manage your metrics:

```rust
use prometheus::{Registry, Counter, Gauge};
use std::sync::Arc;

let registry = Registry::new_custom(Some("my_app".into()), None).unwrap();
let registry = Arc::new(registry);

let http_requests_total = Counter::new("http_requests_total", "Total number of HTTP requests").unwrap();
registry.register(Box::new(http_requests_total.clone())).unwrap();

let cpu_usage_gauge = Gauge::new("cpu_usage_percent", "CPU usage percentage").unwrap();
registry.register(Box::new(cpu_usage_gauge.clone())).unwrap();

let memory_usage_gauge = Gauge::new("memory_usage_percent", "Memory usage percentage").unwrap();
registry.register(Box::new(memory_usage_gauge.clone())).unwrap();
```

---

## **3. Create Metrics Handler for Axum**

Define an Axum handler that exposes the metrics:

```rust
use axum::{Router, routing::get, response::IntoResponse};
use prometheus::{Encoder, TextEncoder};
use std::sync::Arc;

async fn metrics_handler(registry: Arc<Registry>) -> impl IntoResponse {
    let encoder = TextEncoder::new();

    let metric_families = registry.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

// When building your router
let app = Router::new()
    .route("/metrics", get({
        let registry = Arc::clone(&registry);
        move || metrics_handler(registry)
    }));
```

*Explanation:* The handler collects all registered metrics from the registry, encodes them in the Prometheus text format, and returns them as a response to HTTP GET requests on `/metrics`.

---

## **4. Collect System Metrics**

Use the `sysinfo` crate to collect system metrics and update the Prometheus gauges:

```rust
use sysinfo::{System, SystemExt, RefreshKind};

fn collect_system_metrics(
    system: &mut System,
    cpu_gauge: &Gauge,
    memory_gauge: &Gauge,
) {
    system.refresh_cpu();
    system.refresh_memory();

    let cpu_usage = system.global_cpu_info().cpu_usage();
    cpu_gauge.set(cpu_usage as f64);

    let total_memory = system.total_memory() as f64;
    let used_memory = system.used_memory() as f64;
    let memory_usage = (used_memory / total_memory) * 100.0;
    memory_gauge.set(memory_usage);
}
```

---

## **5. Schedule Periodic Metric Updates**

Spawn a background task to periodically update the system metrics:

```rust
use tokio::time::{interval, Duration};

let cpu_gauge_clone = cpu_usage_gauge.clone();
let memory_gauge_clone = memory_usage_gauge.clone();
tokio::spawn(async move {
    let mut system = System::new_with_specifics(RefreshKind::new().with_cpu().with_memory());
    let mut interval = interval(Duration::from_secs(5));

    loop {
        interval.tick().await;
        collect_system_metrics(&mut system, &cpu_gauge_clone, &memory_gauge_clone);
    }
});
```

---

## **6. Increment Request Counter**

Update your request handling logic to increment the `http_requests_total` counter:

```rust
use axum::extract::MatchedPath;
use axum::middleware::Next;
use axum::http::Request;

// Middleware to count requests
async fn track_requests<B>(
    request: Request<B>,
    next: Next<B>,
    counter: Counter,
) -> impl IntoResponse {
    // Increment the counter
    counter.inc();

    // Proceed to the next middleware or handler
    next.run(request).await
}

// Apply middleware to your routes
let app = app.layer(axum::middleware::from_fn({
    let counter = http_requests_total.clone();
    move |req, next| track_requests(req, next, counter)
}));
```

---

## **7. Run Your Axum Server**

Integrate everything into your main function and run the server:

```rust
#[tokio::main]
async fn main() {
    // Your registry and metrics code here...

    // Build your application with routes and middleware
    let app = Router::new()
        // ... your routes ...
        .route("/metrics", get({
            let registry = Arc::clone(&registry);
            move || metrics_handler(registry)
        }))
        .layer(axum::middleware::from_fn({
            let counter = http_requests_total.clone();
            move |req, next| track_requests(req, next, counter)
        }));

    // Start the server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

---

## **8. Configure Prometheus to Scrape Metrics**

Finally, configure your Prometheus instance to scrape the `/metrics` endpoint of your Axum server by updating the `prometheus.yml` configuration file:

```yaml
scrape_configs:
  - job_name: 'axum_server'
    scrape_interval: 5s
    static_configs:
      - targets: ['localhost:3000']  # Replace with your server's address and port
```

---

## **Summary**

By following these steps, your Axum server will:

- Expose an endpoint at `/metrics` that Prometheus can scrape.
- Collect and expose custom metrics like HTTP request counts.
- Collect and expose system metrics such as CPU and memory usage.

---

## **Additional Tips**

- **Error Handling:** Ensure you handle errors appropriately in your production code. The `.unwrap()` calls are for simplicity.
- **Metric Labels:** You can add labels to your metrics for more granular data.
- **Security:** If your metrics endpoint shouldn't be publicly accessible, consider implementing authentication or network-level restrictions.
- **Optimization:** For high-performance needs, batch updates or use more efficient data structures.

---

**Let me know if you need further assistance or have questions about specific parts of the implementation!**