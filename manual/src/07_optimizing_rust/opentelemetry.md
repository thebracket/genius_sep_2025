# OpenTelemetry

This guide shows how to connect your `tracing` spans and events to OpenTelemetry (OTel), so you can view traces in a collector/observability tool. It starts with a zero‑infrastructure stdout exporter, then switches to an OTLP exporter that talks to an OTel Collector.

## Quick Start: Export to Stdout

Add dependencies to your `Cargo.toml`:

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }
tracing-opentelemetry = "0.24"
opentelemetry = "0.24"
opentelemetry_sdk = { version = "0.24", features = ["rt-tokio"] }
```

Then wire `tracing` to an OpenTelemetry tracer with a stdout exporter. This requires no external services and is great for validating your setup.

```rust
use tracing::{info, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use opentelemetry::KeyValue;
use opentelemetry_sdk::{trace as sdktrace, Resource};

fn init_tracing() {
    // Identify your service in telemetry data
    let resource = Resource::new(vec![KeyValue::new("service.name", "spans-example")]);

    // Build an OpenTelemetry tracer pipeline that writes spans to stdout (JSON)
    let tracer = opentelemetry_sdk::export::trace::stdout::new_pipeline()
        .with_trace_config(sdktrace::Config::default().with_resource(resource))
        .install_simple();

    // Bridge tracing -> OpenTelemetry
    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // Keep human-readable logs too (optional)
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .with(otel_layer)
        .init();
}

#[instrument]
fn do_work(n: u64) -> u64 {
    (0..n).sum()
}

fn main() {
    init_tracing();
    info!("starting work");
    let out = do_work(10);
    info!(%out, "done");

    // Ensure all spans are exported before process exit
    opentelemetry::global::shutdown_tracer_provider();
}
```

Run it: `cargo run -q`

You’ll see JSON span records printed to stdout along with your normal logs.

## Send Spans to an OpenTelemetry Collector (OTLP)

To forward spans to a collector via OTLP (recommended), add the OTLP exporter:

```toml
[dependencies]
opentelemetry-otlp = { version = "0.17", features = ["grpc-tonic"] }
```

Update the setup to use OTLP over gRPC. Use `install_simple()` for a minimal example; for production prefer `install_batch(...)` with a Tokio runtime.

```rust
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace as sdktrace, Resource};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn init_tracing_otlp() {
    let resource = Resource::new(vec![KeyValue::new("service.name", "spans-example")]);

    // Configure OTLP exporter. By default, sends to http://localhost:4317 (gRPC)
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(sdktrace::Config::default().with_resource(resource))
        .install_simple();

    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .with(otel_layer)
        .init();
}

fn main() {
    init_tracing_otlp();
    // emit some spans/events as usual...
    opentelemetry::global::shutdown_tracer_provider();
}
```

### Run a local collector

Use the OpenTelemetry Collector with a minimal config that logs received spans:

```yaml
# otel-collector.yaml
receivers:
  otlp:
    protocols:
      grpc:
      http:
exporters:
  logging:
    loglevel: debug
service:
  pipelines:
    traces:
      receivers: [otlp]
      exporters: [logging]
```

Run the collector with Docker:

```bash
docker run --rm -it \
  -p 4317:4317 -p 4318:4318 \
  -v $(pwd)/otel-collector.yaml:/etc/otelcol/config.yaml \
  otel/opentelemetry-collector:latest
```

Then run your app. The collector logs will show the spans it receives. You can later swap the `logging` exporter for backends like Jaeger, Tempo, or OTLP to a SaaS.

## Tips

- Set service/resource data: add `service.name`, `service.version`, `deployment.environment` as `Resource` attributes.
- Control verbosity with `RUST_LOG`, e.g. `RUST_LOG=trace,otel=info`.
- For production, use `install_batch(opentelemetry_sdk::runtime::Tokio)` for non-blocking export and call `shutdown_tracer_provider()` on graceful shutdown.
- To propagate context across services, use `opentelemetry::global::get_text_map_propagator()` and an HTTP propagator like W3C TraceContext.
