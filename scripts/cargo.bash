#!/usr/bin/env bash

cargo verify-project || exit 1
cargo check --all-targets --profile=test || exit 1
cargo deny check || exit 1
cargo audit || exit 1
cargo test -j 4 --no-fail-fast -- --show-output || exit 1
cargo fmt --check || exit 1
cargo clippy -- -D clippy::pedantic -W clippy::nursery -D warnings  -D clippy::all || exit 1
cargo outdated || exit 1
exit 0
