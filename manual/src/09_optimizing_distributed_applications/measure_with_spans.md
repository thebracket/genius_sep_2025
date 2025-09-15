# Measure with Spans

We covered earlier logging and tracing with the `tracing` crate. Adding spans to your code (and optionally submitting to something like OpenTelemetry) allows you to measure the performance of your distributed application.

Make sure you attach a "context" to your spans that is passed between services. This allows you to correlate spans across services, and get a picture of individual transaction performance.

This usually then shows you where the bottlenecks are - and you optimize those as local service improvements.

> When the bottleneck is your architecture, you have to change the architecture!