# Install Script Testing

## Syntax checks

```bash
bash -n run_once_install-packages.sh.tmpl
chezmoi execute-template < run_once_install-packages.sh.tmpl | bash -n
chezmoi execute-template < dot_config/fish/config.fish.tmpl | fish -n
actionlint
git diff --check -- run_once_install-packages.sh.tmpl dot_config/fish/config.fish.tmpl
```

## Dry run

Use `DRY_RUN=1` to print install commands without executing them.

```bash
DRY_RUN=1 bash run_once_install-packages.sh.tmpl
```

To test the Arch decision path without installing anything on the host:

```bash
docker run --rm \
  -v "$PWD:/work:ro" \
  -w /work \
  archlinux:latest \
  bash -lc 'bash -n run_once_install-packages.sh.tmpl && DRY_RUN=1 bash run_once_install-packages.sh.tmpl'
```

## Full Arch test

This runs the real installer inside a disposable Arch container.

```bash
docker run --rm \
  -v "$PWD:/work:ro" \
  -w /work \
  archlinux:latest \
  bash -lc 'pacman -Sy --needed --noconfirm sudo ca-certificates && bash run_once_install-packages.sh.tmpl'
```

For extra verification, run command checks after the install in the same
container shell:

```bash
docker run --rm \
  -v "$PWD:/work:ro" \
  -w /work \
  archlinux:latest \
  bash -lc '
    set -euo pipefail
    pacman -Sy --needed --noconfirm sudo ca-certificates
    bash run_once_install-packages.sh.tmpl
    export PATH="$HOME/.cargo/bin:$HOME/go/bin:$HOME/.local/bin:$HOME/.bun/bin:$HOME/.local/share/pnpm:$PATH"
    for cmd in git fish nvim rg fd fzf lazygit zellij zoxide yazi topgrade lazydocker harlequin bun pnpm; do
      command -v "$cmd" >/dev/null
    done
    fish -c "functions -q nvm"
    fish -c "nvm use default --silent; and node --version; and npm --version; and bw --version"
  '
```

## CachyOS

CachyOS uses `pacman`, so the installer takes the same branch as Arch. The
Arch Docker test is the closest automated proxy for CachyOS package-name
compatibility. If a CachyOS machine has the normal Arch/CachyOS repositories
enabled, the package list should work out of the box.

## Full macOS test

On a real macOS machine, run:

```bash
bash run_once_install-packages.sh.tmpl
```

In CI, use `INSTALL_CASKS=0` to skip GUI/system-extension casks such as Docker
Desktop and Tailscale while still testing Homebrew formulae and language-level
installers:

```bash
INSTALL_CASKS=0 bash run_once_install-packages.sh.tmpl
```

## GitHub Actions

`.github/workflows/install-script.yml` runs:

- Arch: full install inside `archlinux:latest`.
- Apple Silicon: full install on `macos-15` with `INSTALL_CASKS=0`.

The macOS job intentionally skips casks because hosted CI runners are not a good
place to install GUI apps or system-extension apps. Local macOS installs still
install casks by default.
