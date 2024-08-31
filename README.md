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
- run `zuu commit` to check your source code and commit
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
    
            The following changes were made : 
            
                    * why
                    ...
            
    %if resolve
        
            The commit resolve their issues :
        
                    Fixes issue_number
                    ...
    %end
    
    %if close 
    
            The commit close their issues :

                    Closes #12
    %end
            Authored by :

                    * %git.config.user.name% <%git.config.user.email%> the %date%
    
```

## Examples with close issue


```text

    Nebula(commit): create a command for commit

            The following changes were made :

                    * no blocking script

            The commit resolve their issues :

                    Fixes #12

            The commit close their issues :

                    Closes #12

            Authored by :

                    * Willy Micieli <otechdo@otechdo.com> the 2024-08-31
```

The commit type it's based on sky.

### Without close issue

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
