# Repository Guidelines

This repository is the standalone home of the `atomic-blob-store` Rust crate.
The root package contains the library, examples, integration tests, stable
format fixtures, and format/release documentation. `benchmarks/` contains the
protocol-neutral performance harness, and `consumer-tests/` verifies the public
API from a downstream crate.

Use Rust edition 2024 and preserve the workspace MSRV of Rust 1.89. Run
`cargo fmt --all --check`, `cargo test --workspace --all-features`, and
`cargo clippy --workspace --all-targets --all-features -- -D warnings` before
submitting changes. Format or compatibility changes must update `FORMAT.md`,
fixtures, tests, and `CHANGELOG.md`. User-facing changes must update the
changelog and relevant examples or documentation.

Use conventional, squash-friendly commit messages:
`<tag>(<component>): <title>`.
