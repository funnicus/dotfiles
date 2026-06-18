# Install Script Testing

The chezmoi run-once script is only a wrapper. It detects the current platform
and runs a committed binary from `installer/bin/`. The installer logic lives in
the Rust crate under `installer/`, and package data lives in
`installer/packages.toml`.

## Local checks

```bash
bash -n run_once_install-packages.sh.tmpl
chezmoi execute-template < run_once_install-packages.sh.tmpl | bash -n
chezmoi execute-template < dot_config/fish/config.fish.tmpl | fish -n
cd installer && cargo fmt --check
cd installer && cargo check
actionlint .github/workflows/install-script.yml
git diff --check
```

Or run the grouped checks:

```bash
just check
```

`just check` currently covers shell syntax and `cargo check`. Run
`cargo fmt --check` and `actionlint` separately when touching Rust formatting or
workflow YAML.

## Rebuild Binaries

Build the local platform binary:

```bash
cd installer
cargo build --release
```

Then copy it into `installer/bin/` using the platform name expected by the
wrapper:

```bash
# Linux x86_64
cp installer/target/release/dotsetup installer/bin/dotsetup-linux-x86_64

# macOS Apple Silicon
cp installer/target/release/dotsetup installer/bin/dotsetup-macos-arm64
```

The wrapper currently supports these asset names:

- `dotsetup-linux-x86_64`
- `dotsetup-linux-arm64`
- `dotsetup-macos-x86_64`
- `dotsetup-macos-arm64`

Only commit binaries for platforms that are actually supported and tested.

## Local Installer Runs

Run the committed binary through the rendered chezmoi wrapper:

```bash
chezmoi execute-template < run_once_install-packages.sh.tmpl | sh
```

Or run a locally built binary directly:

```bash
cd installer
cargo build --release
./target/release/dotsetup install
```

`DRY_RUN=1` does not make the installer print commands only. It makes prompts
non-interactive, which is useful in CI, but install commands still execute.
Use it only in disposable environments or with the current command behavior in
mind:

```bash
cd installer
DRY_RUN=1 ./target/release/dotsetup install
```

## Arch and CachyOS

The installer supports Arch and CachyOS through the same pacman/AUR path. It
detects Arch-like systems from `/etc/os-release`.

Use the Docker targets for disposable Arch verification:

```bash
just test-arch
just test-arch-ci
```

`test-arch` runs the installer interactively in an Arch container.
`test-arch-ci` sets `CI=1 DRY_RUN=1` so prompts take defaults, but package
commands still run inside the container.

The Docker images compile and run the Rust installer binary:

```bash
docker build -f Dockerfile.arch-test-ci -t dotfiles-arch-test-ci .
docker run --rm -v "$PWD:/work:ro" dotfiles-arch-test-ci
```

## macOS

The macOS installer supports Apple Silicon only. It rejects Intel macOS.

On a real macOS machine:

```bash
cd installer
cargo build --release
./target/release/dotsetup install
```

By default, local macOS runs install both Homebrew formulae and casks. In CI,
skip GUI/system-extension casks such as Docker Desktop and Tailscale:

```bash
cd installer
INSTALL_CASKS=0 CI=1 DRY_RUN=1 ./target/release/dotsetup install
```

Only `INSTALL_CASKS=1`, `INSTALL_CASKS=true`, or `INSTALL_CASKS=yes` enables
cask installation explicitly. If `INSTALL_CASKS` is unset, casks are installed.

The GitHub Actions macOS job also repairs a stale `xcode-select` developer path
before compiling the installer. This handles hosted runner images where
`xcode-select -p` points at a removed Xcode bundle.

## GitHub Actions

`.github/workflows/install-script.yml` runs:

- Arch: builds `Dockerfile.arch-test-ci` and runs the installer in the
  container.
- Apple Silicon: runs on `macos-15`, repairs the Xcode developer path, builds
  the Rust installer, runs it with `CI=1 DRY_RUN=1 INSTALL_CASKS=0`, then
  verifies expected commands are available.

The macOS job intentionally skips casks because hosted CI runners are not a
good place to install GUI apps or system-extension apps.
