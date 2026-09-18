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

Without `BW_SESSION`, chezmoi skips the private SSH and ngrok configs and leaves
existing copies untouched. An invalid session fails without prompting; unlock
Bitwarden again to include secrets in the diff or apply.

The installer includes Atuin and Zed on both supported platforms, plus Raycast
on macOS. The macOS `Chezmoi` iTerm2 profile launches fish with a Nord dark palette.
See [Setup](wiki/Setup.md) for updating an existing machine.

## Testing

Install [just](https://github.com/casey/just). Local checks also need Rust/Cargo;
template tests need Python 3 and chezmoi. Arch container tests need Docker running.

See [justfile](justfile) for available test commands.

Examples:

```bash
just check
just test-templates
just test-arch-ci
just test-arch-bootstrap-ci
```

See [Install Script Testing](wiki/Install-Script-Testing.md) for test scope and
Apple Silicon Docker limitations.

## Wiki

- [Setup](wiki/Setup.md)
- [Installed tools](wiki/Installed-Tools.md)
- [Bitwarden SSH hosts](wiki/Bitwarden-SSH.md)
- [Managing dotfiles](wiki/Managing-Dotfiles.md)
- [Template verification](wiki/Template-Verification.md)
- [Local AI](wiki/Local-AI.md)
