#!/usr/bin/env bash
set -euo pipefail

execute=false
if [[ "${1:-}" == "--execute" ]]; then
    execute=true
    shift
fi
if (($# != 0)); then
    echo "usage: $0 [--execute]" >&2
    exit 2
fi

workspace_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$workspace_dir"
version="$(cargo metadata --no-deps --format-version 1 |
    python3 -c 'import json,sys; print(next(p["version"] for p in json.load(sys.stdin)["packages"] if p["name"] == "atomic-blob-store"))')"

test -z "$(git status --short)" || {
    echo "error: release requires a clean worktree" >&2
    exit 1
}
grep -Fq "## [$version] - " CHANGELOG.md || {
    echo "error: cut CHANGELOG.md for $version before publishing" >&2
    exit 1
}

cargo fmt --all --check
cargo check --locked --workspace --all-targets
cargo test --locked --workspace --all-features
scripts/validate-atomic-blob-package.sh

if [[ "$execute" != true ]]; then
    echo "Validated atomic-blob-store $version. Re-run with --execute to publish."
    exit 0
fi

cargo publish --locked -p atomic-blob-store
git tag -a "atomic-blob-store-$version" -m "atomic-blob-store $version"
