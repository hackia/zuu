<div align="center">


![zuu](https://raw.githubusercontent.com/otechdo/zuu/refs/heads/main/zuu.png)

</div>

# Zuu (Prerelease)

Zuu is available as both a **library** for task management in Rust and as standalone **lint and audit programs** for continuous verification of code across multiple languages.

## Features

- **Zuu Library**:
  - The library enables handling up to **9 tasks** simultaneously using `&str;9`.
  - It is designed to support task automation and management in your Rust projects.
  
- **Lint and Audit Programs**:
  - The standalone programs provide pre-configured commands to perform **linting**, **code formatting**, **security audits**, and **license checks** across various supported languages.

## Installation

### Zuu Library

To use the Zuu library in your Rust project, add it via Cargo:

```bash
cargo add zuu
```

This adds the Zuu library to your project for task management and automation.

### Zuu Programs (Lint, Audit)

To use the Zuu lint and audit programs, you can pull the Docker images from Docker Hub or use them via Docker Compose:

```bash
docker pull otechdo/zuu:rust  # Example for Rust
docker pull otechdo/zuu:php   # Example for PHP
```

Alternatively, use Docker Compose for running tasks in a containerized environment.

## Usage

### Zuu Library

You can define up to 9 tasks and execute them using the Zuu library. Here’s a sample usage:

```rust
use zuu::TaskRunner;

fn main() {
    let tasks = [
        "Test", 
        "Lint", 
        "Format", 
        "Security Audit", 
        "License Check",
        "Documentation",
        "Build",
        "Deploy",
        "Performance Check",
    ];

    let runner = TaskRunner::new();
    runner.run_tasks(&tasks);
}
```

### Lint and Audit Programs

For code linting, auditing, and testing tasks, use the pre-built commands in Zuu Docker images:

```bash
# Example for Rust code audit
docker run otechdo/zuu:rust rust-audit

# Example for PHP code linting
docker run otechdo/zuu:php php-audit
```

These programs automatically perform tasks like linting, formatting, and audits based on the language configuration.

### PHP Dependencies Installation with Phive

To use **phpstan**, **phpcs**, **phpDocumentor**, **phpunit**, and other tools for PHP auditing with Zuu, you will need to install these tools using **phive** (PHAR Installation and Verification Environment).

#### Steps to Install PHP Tools using Phive

1. **Create a `bin` directory**:
   First, create a `bin` directory in your home folder to store the installed tools:

   ```bash
   cd ~
   mkdir bin
   ```

2. **Install PHP Tools with Phive**:
   Use **phive** to install the required PHP tools into the `bin` directory. For example, to install **phpstan**:

   ```bash
   phive install -t bin phpstan
   ```

   You can also install **phpcs**, **phpunit**, and **phpDocumentor** similarly:

   ```bash
   phive install -t bin phpcs
   phive install -t bin phpunit
   phive install -t bin phpDocumentor
   ```

3. **Add the Tools to the System PATH**:
   After installing the tools, add the `bin` directory to your system’s `PATH`. On Linux, you can add the following line to your `.bashrc` or `.bash_profile`:

   ```bash
   export PATH="$HOME/bin:$PATH"
   ```

4. **Modify `composer.json` to Add Scripts**:
   After installing the tools, configure your `composer.json` to include scripts for running the tools. Add the following section to your `composer.json`:

   ```json
   {
     "scripts": {
       "test": "vendor/bin/phpunit",
       "fmt": "phpcs --standard=PSR12 app",
       "lint": "phpstan analyse app --level 9",
       "doc": "phpDocumentor -d app -t docs"
     }
   }
   ```

   This configuration allows you to run the tools with Composer commands such as `composer run-script test` or `composer run-script lint`.

### Example Phive Installation

Here’s a complete example to install **phpstan**, create a `bin` directory, and modify your system’s `PATH`:

```bash
cd ~
mkdir bin
phive install -t bin phpstan
export PATH="$HOME/bin:$PATH"
```

After running these commands, you’ll be able to use **phpstan** directly from the command line or via Composer scripts.

## Documentation

The full documentation for the Zuu library is accessible on **docs.crates.io**:

- [Zuu Library Documentation](https://docs.rs/zuu)

This version is a **stable prerelease** and provides early access to the core functionality of the Zuu library and tools.

## Goals

The goal for Zuu is to support a wide range of programming languages for both the library and the lint/audit programs. We aim to include the following languages over time:

- Bash, C, Clojure, Cobol, Crystal, Dart, Elixir, FSharp, Fortran, Go, Haskell, Julia, Kotlin, Lua, Nim, Objective-C, Perl, PHP, Python, Ruby, Rust, Scala, Swift, TypeScript, Vlang.

## Plugin System

Zuu will introduce a **plugin system** that allows developers to extend the functionality of the library and standalone tools, offering more flexibility for language support and task customization.

---

This version of the README now includes detailed instructions on using **phive** to install PHP dependencies, modifying the `composer.json`, and updating the system `PATH`. Let me know if you need further adjustments!
