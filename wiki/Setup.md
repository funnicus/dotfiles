# Setup

## Install prerequisites

```bash
# macOS
brew install chezmoi bitwarden-cli

# Arch/CachyOS
sudo pacman -S chezmoi bitwarden-cli
```

## First-time setup

Use this only when creating a new chezmoi source repo from an existing machine.

```bash
chezmoi init

chezmoi add ~/.config/fish/config.fish
chezmoi add ~/.gitconfig
chezmoi add ~/.config/nvim
chezmoi add ~/.ssh/config

chezmoi edit ~/.config/fish/config.fish
chezmoi apply
```

## Copy setup to a new machine

```bash
chezmoi init git@github.com:funnicus/dotfiles.git
```

Answer the prompts from `.chezmoi.toml.tmpl`:

```text
git.name          -> git config user.name
git.email         -> git config user.email
git.signingkey    -> optional git config user.signingkey
ssh.bitwardenItem -> Bitwarden item name or ID for private SSH host blocks
ngrok.bitwardenItem -> Bitwarden item name or ID for the ngrok authtoken
```

Then preview and apply:

```bash
bw login
export BW_SESSION="$(bw unlock --raw)"
bw sync

chezmoi diff
chezmoi apply --dry-run --verbose
chezmoi apply
```

## Bootstrap script

`run_once_install-packages.sh.tmpl` is a chezmoi script. Chezmoi runs
`run_once_` scripts automatically during `chezmoi apply`, but only once per
machine. If the script changes later, chezmoi still will not rerun it
automatically under the same run-once state.

The script is now a thin wrapper around a committed `dotsetup` installer binary.
It selects `installer/bin/dotsetup-<os>-<arch>` and executes:

```bash
dotsetup-<os>-<arch> install
```

Cargo is not required on a fresh machine just to run the installer. Cargo is
only needed when rebuilding the committed binaries.

The installer package list lives in `installer/packages.toml`; the Rust code
handles platform detection and command execution.

On non-CI run-once script runs, the wrapper calls `dotsetup bootstrap` before
`dotsetup install`. The bootstrap command has its own confirmation prompt. CI
runs skip bootstrap and go straight to `dotsetup install`.

Because the private SSH template needs `bw` before the first apply, install
`bitwarden-cli` manually with chezmoi in the prerequisite step. The bootstrap
script still installs `bw` later if it is missing, but it cannot help before
the first Bitwarden-backed template render.

On a fresh machine, run `bw login` before `bw unlock`. After login, `bw status`
should report `locked` or `unlocked`; if it reports `unauthenticated`, login has
not completed for that local CLI profile.

The script checks for commands before installing packages, so tools already
installed through another manager are left alone.

Supported installer targets are macOS on Apple Silicon and Arch/CachyOS/Fedora
Linux. Other Linux distributions are detected separately, but package
installation is not implemented for them yet.

To force the bootstrap again, run the rendered script manually or clear the
relevant chezmoi script state.

See [Install Script Testing](Install-Script-Testing.md) for dry-run, Docker, and
GitHub Actions verification commands.
