<div align="center">

![zuu](zuu.png)

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/otechdo/zuu/zuu.yml?branch=main&style=flat&logo=github&logoColor=grey&label=Workflow&labelColor=white&color=white)](https://github.com/otechdo/zuu/actions/workflows/zuu.yml)
[![GitHub Release](https://img.shields.io/github/v/release/otechdo/zuu?include_prereleases&sort=semver&display_name=tag&style=flat&logo=rust&logoColor=grey&label=Release&labelColor=white&color=white)](https://github.com/otechdo/zuu/releases)
[![GitHub License](https://img.shields.io/github/license/otechdo/zuu?style=flat&logo=rust&logoColor=grey&label=License&labelColor=white&color=white)](https://github.com/otechdo/zuu/blob/main/LICENSE)

</div>

# Zuu


**Zuu** is a command-line tool designed to streamline and enhance code verification for Rust projects. It leverages a `zuu.toml` configuration file at the project's root, allowing developers to define a series of checks and actions to be executed before, during, and after `cargo` commands.

**Think of it as a supercharged Makefile for your Rust code quality checks!**


## Key Features

* **Customizable Hooks:** Define `before-cargo`, `cargo`, and `after-cargo` hooks to run shell commands at different stages.
* **Flexible Configuration:** The `zuu.toml` file provides a centralized and adaptable way to manage all your code checks.
* **Built-in `clippy` Support:**  Includes built-in support for `clippy` with customizable lints and severity levels.
* **Extensible with Shell Scripts:** Execute any shell command within your hooks for ultimate flexibility.
* **Automated Workflows:** Integrate seamlessly with CI/CD pipelines and Git hooks.
* **Badge Generation:** Generate badges to display the status of your code verification process.

## Benefits

* **Improved Code Quality:** Enforce coding standards and catch potential issues early.
* **Increased Efficiency:** Automate your code checks and save valuable development time.
* **Enhanced Collaboration:**  Provide a consistent framework for code quality across your team.
* **Greater Flexibility:**  Customize your workflow to fit your specific needs.

## Installation

### For CLI

```bash
cargo install zuu --no-default-features --features cli
```

### For UI (using Broot)

```bash
cargo install zuu broot
```

## Usage

```bash
zuu
```

## Broot Verbs Configuration

```hjson
{
  invocation: zuu
  shortcut: z
  execution: "zuu"
  leave_broot: false
}
```

## Zuu.toml

```toml
before = ["cargo fmt"]
zuu = [
    "cargo verify-project",
    "cargo check --all-targets --profile=test",
    "cargo deny check",
    "cargo audit",
    "cargo test -j 4 --no-fail-fast -- --show-output",
    "cargo fmt --check",
    "cargo clippy -- -D clippy::pedantic -W clippy::nursery -D warnings -D clippy::all",
    "cargo outdated",
]
after = []

[badge]
success = ["curl [https://img.shields.io/badge/zuu-passing-darkgreen](https://img.shields.io/badge/zuu-passing-darkgreen) -o zuu.svg"]
failure = ["curl [https://img.shields.io/badge/zuu-failure-red](https://img.shields.io/badge/zuu-failure-red) -o zuu.svg"]
```

### GitHub Actions Workflow Example 

```yaml
name: zuu
on:
  push:
    branches: [ "master" , "develop" , "main" ]
  pull_request:
    branches: [ "master" , "develop" , "main"  ]
env:
  CARGO_TERM_COLOR: always
  TERM: xterm-256color
jobs:
  zuu:
    strategy:
      matrix:
        os: [ ubuntu-latest, ubuntu-22.04, ubuntu-20.04, macos-latest, macos-13, macos-12 ]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      - name: deps
        run:  cargo install cargo-audit cargo-auditable cargo-deny cargo-outdated
      - name: installation
        run:  cargo install zuu --no-default-features --features cli
      - name: zuu
        run:  git checkout "${GITHUB_REF##*/}" && zuu
```
