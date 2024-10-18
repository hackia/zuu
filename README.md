<div align="center">

![zuu](https://raw.githubusercontent.com/otechdo/zuu/refs/heads/main/zuu.png)

</div>

# Zuu Project

Zuu is a continuous verification tool for multiple programming languages. 

It allows you to run tests, check code formatting, perform security audits, and more, based on the configured programming language.

## Table of Contents
- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [Customisation](#customisation)
- [Continuous Integration](#continuous-integration)
  - [GitHub Actions](#github-actions)
  - [Travis CI](#travis-ci)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Multi-language support**:
  - Bash
  - C
  - Clojure
  - Cobol
  - Crystal
  - Dart
  - Elixir
  - Erlang
  - FSharp
  - Fortran
  - Go
  - Groovy
  - Haskell
  - Java
  - Julia
  - Kotlin
  - Lua
  - Matlab
  - Nim
  - Objective-C
  - Perl
  - PHP
  - Python
  - R
  - Ruby
  - Rust
  - Scala
  - Swift
  - TypeScript
  - Vlang
- Customizable testing options using environment variables for test control.
- Isolation of development and test environments via Docker and Docker Compose.
- Flexible configuration using Cargo `features`.
- Execute tests, format checks, security audits, and more for each supported language.

## Requirements

- [Docker](https://www.docker.com/get-started) (version >= 20.10)
- [Docker Compose](https://docs.docker.com/compose/install/) (version >= 1.29)
- [Rust](https://www.rust-lang.org/tools/install) and Cargo (for local development)

## Installation

1. Clone the repository to your machine:
    ```bash
    git clone https://github.com/otechdo/zuu.git
    cd zuu
    ```

2. Running with Docker Compose

To run the project in an isolated Docker environment, use Docker Compose.

You can run tests for each supported language.

To start a test container for a specific language, use the following command:

```bash
docker-compose up --build --abort-on-container-exit <service-name>
```

For example, to run the tests for **Rust**:

```bash
docker-compose up --build --abort-on-container-exit rust-tests
```

1. Running Tests for All Languages


You can also run all the tests sequentially for every supported language:

```bash
docker-compose up --build --abort-on-container-exit
```

1. Using Cargo

If you're working locally with **Rust** and want to compile the project with a specific language feature, use Cargo `features`. For example, to activate the feature for **Rust**:

```bash
cargo build --no-default-features --features "rust"
```

## Zuu Configuration Options

Zuu provides several options that you can control via environment variables to customize the checks it performs. The configuration options allow you to enable or disable specific tasks such as testing, linting, formatting checks, security audits, and license validation.

### Available Options:

| Environment Variable | Description                                      | Default |
|----------------------|--------------------------------------------------|---------|
| `TESTS`              | Enable or disable the execution of tests.        | `false` |
| `FORMAT`             | Check if the code is properly formatted.         | `false` |
| `LINT`               | Run a linting process to catch potential issues. | `false` |
| `AUDIT`              | Perform a security audit of the dependencies.    | `false` |
| `LICENSE`            | Check the license compatibility of dependencies. | `false` |

### Usage

These options are controlled using environment variables, which you can set when running the project in either your development or Docker environment. By default, all options are disabled (`false`), but you can enable them as needed by setting them to `true`.

#### Running Locally:

When running locally, you can set these environment variables in your shell before executing the Zuu tool:

```bash
export TESTS=true       # Enable test execution
export FORMAT=true      # Enable code formatting checks
export LINT=true        # Enable linting
export AUDIT=true       # Enable security audit
export LICENSE=true     # Enable license checks

# Run the tool after setting the environment variables
cargo run
```

#### Using Docker Compose:

In the `docker-compose.yml` file, you can define these options under the `environment` section for your service. For example:

```yaml
version: '3'
services:
  zuu:
    image: "your-docker-image"
    environment:
      - TESTS=true
      - FORMAT=true
      - LINT=true
      - AUDIT=true
      - LICENSE=true
    volumes:
      - /dir/on/host:/app
```

This will run Zuu with all the options enabled.

### Customizing Options in Docker:

You can adjust these options in Docker by modifying the environment variables in your `docker-compose.yml` or when running the Docker container.

For example:

```bash
docker run -e TESTS=true -e FORMAT=true -e LINT=false -e AUDIT=true -e LICENSE=true your-docker-image
```

## Customisation

Follow these steps to customise the project, edit the Dockerfiles, and push the images to your own Docker repository:

1. **Clone the repository** to your machine:
    ```bash
    git clone https://github.com/otechdo/zuu.git
    cd zuu/dockers
    ```

2. **Ensure Docker and Docker Compose** are installed on your system:
    - Docker: [Install Docker](https://docs.docker.com/get-docker/)
    - Docker Compose: [Install Docker Compose](https://docs.docker.com/compose/install/)

3. **Edit the Dockerfiles** according to your needs:
    - Navigate to the appropriate directories for each language (e.g., `rust/Dockerfile`, `python/Dockerfile`).
    - Make the necessary changes to each Dockerfile.

4. **Login to Docker Hub**:
    - If you are not logged in already, log in to Docker Hub:
      ```bash
      docker login
      ```

5. **Build and push your images**:
    - Use the following command to build and push all Docker images to your own repository:
      ```bash
      make -j 4 USERNAME="your_docker_username" REPO="your_repository_name"
      ```

6. **Verify your images on Docker Hub**:
    - Once pushed, you can verify the images by logging into Docker Hub and navigating to your repository.

## Continuous Integration

You can use both **GitHub Actions** and **Travis CI** for continuous integration (CI) to automate the build and testing process for the project.

### GitHub Actions

1. **Create a `.github/workflows/ci.yml` file** in your repository with the following configuration:

```yaml
name: zuu
on:
  push:
    branches:
        - main
        - develop
  pull_request:
    branches:
      - main
      - develop
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
        run:  cargo install zuu --no-default-features --features rust
      - name: zuu
        run:  git checkout "${GITHUB_REF##*/}" && zuu

```

1. **Set up Docker credentials** in GitHub Secrets:
    - Go to your repository’s **Settings > Secrets**.
    - Add `DOCKER_USERNAME` and `DOCKER_PASSWORD` with your Docker Hub credentials.

With this configuration, GitHub Actions will build and test your Docker images on every push or pull request to the `main` branch.

### Travis CI

1. **Create a `.travis.yml` file** in your repository:

```yaml
language: minimal

services:
  - docker

before_script:
  - docker-compose --version
  - docker-compose up --build --abort-on-container-exit

script:
  - docker-compose run <your_test_service>

deploy:
  provider: script
  script: docker-compose push
  on:
    branch: main

env:
  global:
    - DOCKER_USERNAME=$DOCKER_USERNAME
    - DOCKER_PASSWORD=$DOCKER_PASSWORD
```

1. **Set up Docker credentials** in Travis CI:
    - Go to your Travis CI project’s **Settings**.
    - Add `DOCKER_USERNAME` and `DOCKER_PASSWORD` as environment variables.

This Travis CI configuration will build and test the Docker images, and push them to Docker Hub when the `main` branch is updated.

## Contributing

Contributions are welcome! To contribute to the project, follow these steps:

1. Fork the project.
2. Create a branch for your feature (git checkout -b feature/amazing-feature`).
3. Commit your changes (`git commit -m 'Add some amazing feature'`).
4. Push to the branch (`git push origin feature/amazing-feature`).
5. Open a Pull Request.

## License

This project is licensed under the AGPL-3.0 License. See the [LICENSE](https://raw.githubusercontent.com/otechdo/zuu/refs/heads/main/LICENSE) file for more details.
