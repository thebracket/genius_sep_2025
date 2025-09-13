# Microservice Design

There are whole *books* written on microservices design, so we'll keep it relatively short.

In my experience, microservices should be structured around business capabilities. Each microservice should represent a specific business function or domain, such as user management, order processing, or inventory management. This approach allows for better alignment with business goals and makes it easier to understand the purpose of each service.

You don't always want to start with microservices. Starting with a monolith made up of crates/modules that each provide a specific business function is often a good idea. And with some careful design, switching from one to the other is not too difficult.

When you make a crate - as a microservice, take advantage of Rust's ability to have both a `main.rs` and a `lib.rs` file.

The `main.rs` file is the entry point for the microservice. The `lib.rs` defines a client for using it.

And if you're migrating from a monolith - you start with the `lib.rs` as the service API, and then add `main.rs` to make it a microservice.

If you choose to adopt "hexagonal design" or other 100% buzzword compliant architectures, you can still apply this approach: make sure that there's always a limited, clear API for each service AND service layer. So whether you are writing a service that talks to the `BookRespository` or something that wraps its own database (or something inbetween) - the API should be clear and limited.