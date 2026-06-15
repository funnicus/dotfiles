# Template Verification

## Git config

Render with explicit test data:

```bash
chezmoi execute-template \
  --override-data '{"git":{"name":"Alice Example","email":"alice@example.com","signingkey":""}}' \
  --file dot_gitconfig.tmpl
```

## SSH config without Bitwarden

This should render only the public `Host *` block:

```bash
chezmoi execute-template \
  --override-data '{"ssh":{"bitwardenItem":""}}' \
  --file private_dot_ssh/config.tmpl
```

## SSH config with Bitwarden

```bash
bw login
export BW_SESSION="$(bw unlock --raw)"
bw sync
chezmoi execute-template < private_dot_ssh/config.tmpl
```

## Shell syntax

```bash
fish -n dot_config/fish/config.fish
bash -n run_once_install-packages.sh.tmpl
```

## Full dry run

```bash
chezmoi diff
chezmoi apply --dry-run --verbose
```

Dry-run output includes script content, but does not install packages.

For package installer dry runs and full Docker-based install tests, see
[Install Script Testing](Install-Script-Testing.md).
