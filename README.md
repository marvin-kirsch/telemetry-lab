# Telemetry Lab

A Rust learning project exploring telemetry, networking, and reliable systems through incremental, hands-on development.

The project starts with a deliberate refresh of Rust fundamentals and will evolve toward a sensor telemetry system with recording, replay, and fault simulation. This repository documents both the implementation and the reasoning behind it.

## Current status

Early development: refreshing the basics and establishing the project foundation. The roadmap below describes planned work, not completed features.

## Roadmap

- [ ] Generate simulated sensor readings with sequence numbers and timestamps.
- [ ] Record readings as JSON lines and replay recorded sessions.
- [ ] Send and receive telemetry over UDP.
- [ ] Detect missing, duplicate, and out-of-order readings.
- [ ] Simulate packet loss and delays to evaluate receiver behaviour.
- [ ] Measure throughput, latency, and timing variability.
- [ ] Connect a microcontroller and replace simulated readings with real sensor data.

## Learning goals

- Write idiomatic Rust using ownership, types, and explicit error handling.
- Understand concurrency, networking, and timing through practical experiments.
- Build testable components and investigate failure modes.
- Document design decisions, limitations, and measured results.

## Development

For the Cargo project, run these commands from the directory containing `Cargo.toml`:

```sh
cargo run
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

Hardware is planned for a later stage; initial development uses simulated data.

## Project notes

As the project grows, this README will include usage examples and measured results. Significant design choices and experiments will be documented alongside the code so the development process remains easy to follow.
