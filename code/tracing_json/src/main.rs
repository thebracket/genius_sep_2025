use tracing::{info, info_span, warn, Level};
use tracing_subscriber::fmt::{format::FmtSpan, time::SystemTime};

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    // Structured fields go into the JSON as key/value pairs
    info!(numerator, denominator, "divide called");
    if denominator == 0.0 {
        warn!(denominator, "denominator is zero, returning None");
        None
    } else {
        let result = numerator / denominator;
        info!(result, "computed division result");
        Some(result)
    }
}

fn main() {
    // Configure a JSON formatter for structured logs
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_timer(SystemTime)
        .json()
        .init();

    // Put some work inside a span to demonstrate span context in JSON
    let span = info_span!("compute_divisions", request_id = 42);
    let _e = span.enter();
    let _ = divide(10.0, 2.0);
    let _ = divide(10.0, 0.0);
}
