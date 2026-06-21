set shell := ["bash", "-cu"]

image := "dotfiles-arch-test"
bootstrap_image := "dotfiles-arch-bootstrap-test"

# Show available commands
default:
    just --list

# Build the Arch test Docker image
build-arch:
    docker build -f Dockerfile.arch-test -t {{image}} .

# Build the Arch test ci Docker image
build-arch-ci:
    docker build -f Dockerfile.arch-test-ci -t {{image}} .

# Build the Arch bootstrap test Docker image
build-arch-bootstrap:
    docker build -f Dockerfile.arch-bootstrap-test -t {{bootstrap_image}} .

# Build the Arch bootstrap test ci Docker image
build-arch-bootstrap-ci:
    docker build -f Dockerfile.arch-bootstrap-test-ci -t {{bootstrap_image}} .

# Test Arch installer in Docker
test-arch: build-arch
    docker run --rm -it -v "$PWD:/work:ro" {{image}}

# Test Arch installer in Docker without interaction
test-arch-ci: build-arch-ci
    docker run --rm -v "$PWD:/work:ro" {{image}}

# Test Arch bootstrap in Docker
test-arch-bootstrap: build-arch-bootstrap
    docker run --rm -it -v "$PWD:/work:ro" {{bootstrap_image}}

# Test Arch bootstrap in Docker without interaction
test-arch-bootstrap-ci: build-arch-bootstrap-ci
    docker run --rm -v "$PWD:/work:ro" {{bootstrap_image}}

# Build the dotsetup binary locally
build-dotsetup:
    cd installer && cargo build --release

# Run the Rust installer locally in non-interactive mode
dry-run:
    cd installer && cargo build --release && DRY_RUN=1 ./target/release/dotsetup install

# Run the Rust bootstrap locally in non-interactive mode
bootstrap-dry-run:
    cd installer && cargo build --release && DRY_RUN=1 ./target/release/dotsetup bootstrap

# Run the Rust installer locally for real
install:
    cd installer && cargo build --release && ./target/release/dotsetup install

# Run the Rust bootstrap locally for real
bootstrap:
    cd installer && cargo build --release && ./target/release/dotsetup bootstrap

# Check shell templates for syntax
check-sh:
    bash -n run_once_install-packages.sh.tmpl

# Check Rust installer
check-rust:
    cd installer && cargo check

# Format Rust installer
fmt:
    cd installer && cargo fmt

# Run all reasonable local checks
check: check-sh check-rust
