use tracing::info;
use tracing::warn;

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    info!("divide() called with numerator: {}, denominator: {}", numerator, denominator);
    if denominator == 0.0 {
        warn!("denominator is zero, returning None");
        None
    } else {
        let result = numerator / denominator;
        info!("denominator is non-zero, returning Some({})", result);
        Some(result)
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    divide(10.0, 2.0);
    divide(10.0, 0.0);
}