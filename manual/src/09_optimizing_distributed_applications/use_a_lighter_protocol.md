# Use a Lighter Protocol

If you are using HTTP/JSON for your service calls, consider switching to something lighter weight. Options include:

- gRPC (with Protobuf or FlatBuffers)
- A high-performance TCP or UDP binary protocol (custom or something like Cap'n Proto)
- Message queues (like NATS, RabbitMQ, or Kafka) with a binary protocol

These protocols reduce the serialization/deserialization cost, and often reduce the network overhead as well. gRPC, for example, uses HTTP/2 which has lower overhead than HTTP/1.1, and Protobuf is much more compact than JSON (but gRPC is still very header-heavy!).

There's *always* a tradeoff: the more complex the protocol, the harder it is to debug and maintain.