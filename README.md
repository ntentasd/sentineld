# sentineld

**A lightweight, async-native system metrics daemon written in Rust.**

`sentineld` monitors per-core CPU usage and exposes the data in Prometheus-compatible format over HTTP. Built for performance and extensibility.

---

## Features

- [x] Per-core CPU usage monitoring with `sysinfo`
- [x] Prometheus `/metrics` endpoint via `axum`
- [x] Fully async and non-blocking powered by `tokio`
- [x] Configurable scrape interval via `PERIOD` env var
- [x] Minimal runtime deps, native performance

## Quick Start

```bash
# Clone and run
git clone https://github.com/ntentasd/sentineld
cd sentineld
cargo run --release
```
---

```bash
# Set scrape interval in seconds (default: 5)
PERIOD=3 cargo run --release
```
---

```url
# Access metrics
http://localhost:8080
```
