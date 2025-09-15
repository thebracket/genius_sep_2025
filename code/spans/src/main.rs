use tracing::{debug, error, info, info_span, instrument, trace};
use tracing_subscriber::{fmt, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_subscriber::fmt::time::SystemTime;

fn init_tracing() {
    // Use RUST_LOG if set; otherwise default to info globally and trace for this crate
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,spans=trace"));

    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_target(false)
                .with_thread_ids(true)
                .with_line_number(true)
                .with_timer(SystemTime)
                .with_span_events(FmtSpan::CLOSE)
                .compact(),
        )
        .init();
}

#[instrument(level = "info", skip(items), fields(len = %items.len()))]
fn sum(items: &[u64]) -> u64 {
    let mut total = 0u64;
    for (idx, v) in items.iter().enumerate() {
        trace!(idx, value = *v, "accumulating");
        total = total.saturating_add(*v);
    }
    debug!(%total, "sum complete");
    total
}

#[instrument(level = "info", err)]
fn try_parse_usize(input: &str) -> Result<usize, std::num::ParseIntError> {
    input.parse::<usize>()
}

#[instrument(level = "debug", name = "parent_op")] 
fn do_nested_work() {
    let child = info_span!("child_span", step = %"A");
    let _guard = child.enter();
    info!("inside child span");
    for i in 0..2u8 {
        debug!(i, "looping");
    }
}

fn main() {
    init_tracing();
    info!("starting spans example");

    let data = vec![1, 2, 3, 4, 5, 6];
    let total = sum(&data);
    info!(%total, "computed sum");

    match try_parse_usize("not-a-number") {
        Ok(n) => info!(n, "parsed successfully"),
        Err(e) => error!(%e, "failed to parse"),
    }

    do_nested_work();
    info!("done");
}
