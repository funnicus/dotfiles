# Dotfiles

Personal dotfiles managed with [chezmoi](https://www.chezmoi.io/). Works for macOS (Apple Silicon) and Linux (Arch/CachyOS). Also installs a large set of tools and packages I want in every system.

## Quick start

```bash
# macOS
brew install chezmoi bitwarden-cli

# Arch/CachyOS
sudo pacman -S chezmoi bitwarden-cli

chezmoi init git@github.com:funnicus/dotfiles.git

# Required when applying private variables from Bitwarden.
# Remember to unlock each time you want to apply changed private variables.
bw login
export BW_SESSION="$(bw unlock --raw)"
bw sync

chezmoi diff
chezmoi apply --dry-run --verbose
chezmoi apply
```

## Testing

Prerequisite: [just installed](https://github.com/casey/just).

See [justfile](justfile) for available test commands.

Examples:

```bash
just test-arch
just test-arch-ci
```

## Wiki

- [Setup](wiki/Setup.md)
- [Bitwarden SSH hosts](wiki/Bitwarden-SSH.md)
- [Managing dotfiles](wiki/Managing-Dotfiles.md)
- [Template verification](wiki/Template-Verification.md)
