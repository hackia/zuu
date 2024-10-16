# zuu

A program to verify cargo source code

![zuu](https://raw.githubusercontent.com/otechdo/zuu/refs/heads/main/zuu.svg)

[![zuu](https://github.com/otechdo/zuu/actions/workflows/zuu.yml/badge.svg)](https://github.com/otechdo/zuu/actions/workflows/zuu.yml)

## Instalation

### For cli

```bash
cargo install zuu --no-default-features --features cli 
```

### For ui

```bash
cargo install zuu broot
```

## Usage

```bash
zuu
```

### Broot verbs config


```hjson
{
      invocation: zuu
      shortcut: z
      execution: "zuu"
      leave_broot: false
}  
```

![demo](https://raw.githubusercontent.com/otechdo/zuu/refs/heads/main/zuu-23.0.0.gif)


## Github workflow

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
        os: [ ubuntu-latest, macos-latest ]
    runs-on: ${{ matrix.os }}
    steps:
    - uses: actions/checkout@v3
    - name: deps
      run:  cargo install cargo-audit cargo-auditable cargo-deny cargo-outdated
    - name: installation
      run:  cargo install zuu --no-default-features --features cli
    - name: zuu
      run: git clone <repo_url> app && cd app && git checkout "${GITHUB_REF##*/}" && zuu
```


