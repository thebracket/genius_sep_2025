# Optimizing Distributed Applications

Distributed applications add complexity and overhead compared to monolithic applications. Now you have to optimize at several levels:
- The individual services
- The communication between services
- The architecture - and patterns of service calling
- The hosting infrastructure

It's also a lot harder to measure performance in a distributed application. You have to measure the individual services, the communication between services, and the overall system performance.

> Tip: when emitting span traces, assign a stable identifier to transactions - and pass that between services. This allows you to correlate spans across services, and get a picture of individual transaction performance.

Quite often, the speed increase of moving from Python to Rust is significant---and often sufficient. Other times, the problem is your architecture. For example (back when "microservices" were marketed as "Service Oriented Architecture - SoA") I once helped a client with a system that nested service calls 10+ layers deep. Each service call contacted a "service locator" to find the next service. Several services each authenticated via another service. The result was that a simple request took several seconds to complete, and the system was brittle and hard to maintain. 

1. Switching to a "JWT" (to allow services to validate the caller without a separate authentication call) greatly improved performance.
2. Realizing that some of the service calls executed faster than a network call and consolidating the layers improved performance even more.
3. Switching from a heavy "enterprise service bus" to simple REST calls improved performance even more.

This was back in the days of Java---but you see similar antipatterns today. It's very easy to create a distributed application that is slow and brittle.