# Changelog

## [Unreleased]


## [0.1.0] - 2026-07-27

- Add `BlockingAtomicBlobStore` blocking facade.
- Add optional `tokio::AtomicBlobStore` async facade (feature-gated).
- Add bounded-memory streaming saves (`AsyncWrite`) and loads (`AsyncRead`).
- Add ordered `flush` and deterministic `close` lifecycle operations.
- Add native Windows filesystem backend.


## [0.1.0-alpha.1] - 2026-07-23

- Extract the bounded atomic blob snapshot abstraction with configurable
  identity, bounded coordination, stable format documentation, and neutral API.
