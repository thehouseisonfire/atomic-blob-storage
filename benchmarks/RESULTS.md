# Recorded results

The checked-in JSON captures Linux/Btrfs/NVMe characterization runs from
2026-07-21 through 2026-07-26. It includes envelope throughput, payload sweeps,
durable complete and streaming I/O, coordination scaling, lifecycle resources,
maintenance barriers, and source/destination backpressure.

The original runs used Rust 1.96.1 while the workspace MSRV was and remains
1.89. Results include environment metadata and raw distributions. Host
frequency and scheduling drift were material, so paired repeats are stronger
evidence than non-interleaved before/after sweeps. No macOS, native Windows,
cold-cache, edge-device, CPU-utilization, or physical-device write-amplification
claim should be inferred from these files.
