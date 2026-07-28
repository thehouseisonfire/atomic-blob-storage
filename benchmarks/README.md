# Benchmarks

This package owns the maintained protocol-neutral performance harness for
`atomic-blob-store`. It measures envelope processing, durable file operations,
same-key and different-key coordination, store lifecycle resources, maintenance
barriers, and streaming backpressure.

Run a workload from the workspace root:

```bash
cargo run --release -p atomic-blob-store-benchmarks \
  --bin atomic-blob-store-bench -- \
  persistence file-store --operation save-replace --payload-size 1048576
```

The other commands are `envelope`, `coordination`, `lifecycle`, `maintenance`,
and `backpressure`. Each prints one schema-version-1 JSON object containing the
scenario, configuration, metrics, raw samples, and environment metadata.

Checked-in `baselines/` and selected `results/` retain the original measurements
and their legacy `persistence-*` scenario identifiers so later runs remain
comparable. These measurements characterize their recorded host and filesystem;
they are not cross-platform performance guarantees.

See [METHODOLOGY.md](METHODOLOGY.md) and [RESULTS.md](RESULTS.md).
