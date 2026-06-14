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
chezmoi init git@github.com:yourname/dotfiles.git
```

Answer the prompts from `.chezmoi.toml.tmpl`:

```text
git.name          -> git config user.name
git.email         -> git config user.email
git.signingkey    -> optional git config user.signingkey
ssh.bitwardenItem -> Bitwarden item name or ID for private SSH host blocks
```

Then preview and apply:

```bash
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

Because the private SSH template needs `bw` before the first apply, install
`bitwarden-cli` manually with chezmoi in the prerequisite step. The bootstrap
script still installs `bw` later if it is missing, but it cannot help before
the first Bitwarden-backed template render.

The script checks for commands before installing packages, so tools already
installed through another manager are left alone.

To force the bootstrap again, run the rendered script manually or clear the
relevant chezmoi script state.
