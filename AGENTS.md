# Repository Guidelines

## Project Structure & Module Organization

This Rust 2024 workspace targets Rust 1.85+. The public crate lives in `src/`: `lib.rs` defines the shared API and engine integration, `blocking.rs` and `tokio.rs` expose the two facades, `engine/` handles scheduling and lifecycle, and `filesystem/` contains platform backends. Unit tests are colocated in `src/tests.rs`; behavior and compatibility suites live in `tests/`, with immutable V1 samples under `tests/fixtures/v1/`. Runnable examples are in `examples/`. `benchmarks/` is a separate harness crate, while `consumer-tests/` validates the packaged crate from a downstream consumer's perspective. Format and release contracts are documented in `FORMAT.md` and `RELEASE.md`.

## Build, Test, and Development Commands

- `cargo check --locked --workspace --all-targets` checks every workspace target using the committed lockfile.
- `cargo test --locked --workspace --all-features` runs the primary unit, integration, and documentation-facing test set.
- `cargo test --locked -p atomic-blob-store --no-default-features` verifies the Tokio-free blocking build.
- `cargo fmt --all --check` checks standard Rust formatting.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` enforces lint-clean code.
- `cargo doc --locked -p atomic-blob-store --no-deps` validates crate documentation.
- `scripts/validate-atomic-blob-package.sh` packages the crate and exercises blocking and Tokio consumers.
- `python3 scripts/check-markdown-links.py` checks repository Markdown links.

Run Tokio examples with `cargo run --example config_snapshot --features tokio`.

## Coding Style & Naming Conventions

Use `rustfmt` defaults (four-space indentation) and keep Clippy warning-free. Follow Rust conventions: `snake_case` for modules, functions, and tests; `CamelCase` for types and traits; `SCREAMING_SNAKE_CASE` for constants. Keep the default facade free of Tokio dependencies and gate async code with `#[cfg(feature = "tokio")]`. Document observable durability, cancellation, and lifecycle behavior rather than implementation assumptions.

## Testing Guidelines

Use Rust's built-in test framework and `tempfile` for isolated filesystem cases. Name tests after behavior, such as `dropping_streaming_save_preserves_old_blob`. Add integration coverage for public behavior and both facades where applicable. V1 envelope changes require explicit compatibility review and updated immutable fixtures. Filesystem guarantees must be validated on the relevant native platform; do not infer Windows behavior from Unix tests.

## Commit & Pull Request Guidelines

Use Conventional Commits, as in `feat: ...`, `fix(ci): ...`, or `docs: ...`. Keep commits and pull requests focused. PR descriptions should explain user-visible behavior, testing performed, and platform implications; link relevant issues. Update `CHANGELOG.md` for user-facing changes and include benchmark evidence when making performance claims.
