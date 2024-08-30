# Zuu

A program to commit rust source code in order to commit only good code.

## Requirement

- cargo
    - cargo-outdated
    - cargo-watch (recommended)
    - cargo-audit
    - cargo-auditable
    - cargo-deny
- hunspell
    - hunspell-en_US
- vcs
    - git
- config
    - `zuu.toml`

## How to use it ?

- First create the `zuu.toml`
    - `zuu init`
- After you can :
- run `zuu` on your repository to check your source code
    - if source code can be commited `zuu` will ask you if you want commit your code
        - If you enter y or Y you will be entering in commit mode
        - else exit the program

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
after-cargo = ["echo 'Great jobs'"]
```

> clippy it's not required in cargo hooks in run automatically with clippy group

## Projects linked

- [cargo-configure](https://github.com/otechdo/cargo-configure)

## Commit message format

```text
    type(scope): summary

            description
            description
            ...
            
            The following changes were made :

                    * why
                    * why
                    ...
            
            Authored by :

                    * author email the date
```

The commit type it's based on sky.

## Log output example

```text
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Aug 30 16:26:29 2024 +0000

    Comet(zuu): fix typo and increase zuu version

            The following changes were made :

                    * use correct words for console output

            Authored by :

                    * Willy Micieli <otechdo@otechdo.com> the 2024-08-30

commit 443e48e96df5142b44599db744053b55a590c833 (origin/main, origin/develop, origin/HEAD, main)
Author: Willy Micieli <otechdo@otechdo.com>
Date:   Fri Aug 30 16:17:58 2024 +0000

    Big Bang(commit): add commit message possibility

            The following changes were made :

                    * purpose to commit source code after success

            Authored by :

                    * Willy Micieli <otechdo@otechdo.com> the 2024-08-30
```
