set shell := ["bash", "-cu"]

image := "dotfiles-arch-test"
fedora_image := "dotfiles-fedora-test"

# Show available commands
default:
    just --list

# Build the Arch test Docker image
build-arch:
    docker build -f Dockerfile.arch-test -t {{image}} .

# Build the Arch test ci Docker image
build-arch-ci:
    docker build -f Dockerfile.arch-test-ci -t {{image}} .

# Test Arch installer in Docker
test-arch: build-arch
    docker run --rm -it -v "$PWD:/work:ro" {{image}}

# Test Arch installer in Docker without interaction
test-arch-ci: build-arch-ci
    docker run --rm -v "$PWD:/work:ro" {{image}}

# Build the Fedora test Docker image
build-fedora:
    docker build -f Dockerfile.fedora-test -t {{fedora_image}} .

# Build the Fedora test ci Docker image
build-fedora-ci:
    docker build -f Dockerfile.fedora-test-ci -t {{fedora_image}} .

# Test Fedora installer in Docker
test-fedora: build-fedora
    docker run --rm -it -v "$PWD:/work:ro" {{fedora_image}}

# Test Fedora installer in Docker without interaction
test-fedora-ci: build-fedora-ci
    docker run --rm -v "$PWD:/work:ro" {{fedora_image}}

# Build the dotsetup binary locally
build-dotsetup:
    cd installer && cargo build --release

# Run the Rust installer locally in non-interactive mode
dry-run:
    cd installer && cargo build --release && DRY_RUN=1 ./target/release/dotsetup install

# Run the Rust installer locally for real
install:
    cd installer && cargo build --release && ./target/release/dotsetup install

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
