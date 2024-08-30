# generate zuu config

```bash
zuu init
```

## Config

```toml
# clippy allowed group
allow = []
# clippy warn group
warn = []
# clippy forbid group
forbid = [
    "cargo",
    "complexity",
    "style",
    "nursery",
    "pedantic",
    "suspicious",
    "correctness",
    "perf",
]
# before cargo hooks
before-cargo = ["cargo fmt"]
# cargo hooks
cargo = [
    "verify-project",
    "check --all-targets --profile=test",
    "deny check",
    "audit",
    "test -j 4 --no-fail-fast -- --show-output",
    "fmt --check",
    "outdated",
]
# after cargo hooks
after-cargo = ["git status"]
```

> clippy it's not required in cargo hooks in run automaticly with clippy group

## Projects linked

- [cargo-configure](https://github.com/otechdo/cargo-configure)
