# Benchmark methodology

Envelope workloads measure production encode, decode, CRC32C, and validation
error paths without filesystem synchronization. File-store workloads use the
public streaming or complete-operation APIs and include the real filesystem
commit and synchronization path.

Coordination measures submission-to-completion latency for missing-key
inspection with same-key and different-key contention. Lifecycle measures
threads, peak RSS, allocations, startup, work, and deterministic close for
different store counts and concurrency bounds. Maintenance separates idle
barriers from barriers queued behind work. Backpressure uses gated Tokio
endpoints and benchmark-only events to distinguish pressure establishment from
post-release completion.

Use release builds on the target filesystem, keep raw JSON, record the actual
environment, and compare repeated paired runs. Warm-cache measurements do not
support cold-cache or physical write-amplification claims.
