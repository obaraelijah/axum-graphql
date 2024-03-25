# Axum-GraphQL

This is a relatively simple template that combines the following crates into a base for future work:

- axum
- async-graphql
- dotenv
- metrics
- opentelemetry
- tracing
- tokio

## What you can use this for

If you plan to implement a GraphQL API in Rust, you can use this template as a starting point.

It comes with metrics and tracing pre-configured. Setting those up is usually a lot of boiler plate and differs from web server to web server. In this case, you don't have to worry about these aspects of your service at all.

## What is included?

- Setup of axum
- A health endpoint (useful for liveness and readyness checks on Kubernetes)
- A simple GraphQL API
- Tracing
  - stdout
  - Jaeger agent
- Metrics
  - Prometheus endpoint included
  - Basic request tracking
    - No. of requests
    - Duration
- A basic Dockerfile for deployment