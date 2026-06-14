# Dotfiles

Personal dotfiles managed with [chezmoi](https://www.chezmoi.io/). Works for macOS (Apple Silicon) and Linux (Arch/CachyOS).

## Quick start

```bash
# macOS
brew install chezmoi bitwarden-cli

# Arch/CachyOS
sudo pacman -S chezmoi bitwarden-cli

chezmoi init git@github.com:funnicus/dotfiles.git

# Required when applying private SSH hosts from Bitwarden.
bw login
export BW_SESSION="$(bw unlock --raw)"
bw sync

chezmoi diff
chezmoi apply --dry-run --verbose
chezmoi apply
```

## Wiki

- [Setup](wiki/Setup.md)
- [Bitwarden SSH hosts](wiki/Bitwarden-SSH.md)
- [Managing dotfiles](wiki/Managing-Dotfiles.md)
- [Template verification](wiki/Template-Verification.md)
