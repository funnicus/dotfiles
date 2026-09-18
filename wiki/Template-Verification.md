# Template Verification

## Git config

Render with explicit test data:

```bash
chezmoi execute-template \
  --override-data '{"git":{"name":"Alice Example","email":"alice@example.com","signingKey":""}}' \
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
bash -n run_once_install-packages.sh.tmpl
chezmoi execute-template < dot_config/fish/config.fish.tmpl | fish -n
chezmoi execute-template < run_once_install-packages.sh.tmpl | bash -n
```

## Rust installer

```bash
cd installer
cargo fmt --check
cargo check
```

## Full dry run

Without `BW_SESSION`, chezmoi skips the SSH config and both ngrok config
locations, leaving existing files untouched. Unlock Bitwarden and export
`BW_SESSION` to include them in the diff. Secret lookups use `--nointeraction`
so an invalid session fails without trying to prompt from a template (which
can cause Bitwarden's `ERR_USE_AFTER_CLOSE: readline was closed` error).

Run the isolated Bitwarden template regression checks with `just test-templates`.
These use a fake `bw` command and do not access your vault.

```bash
chezmoi diff
chezmoi apply --dry-run --verbose
```

Dry-run output includes script content, but does not install packages.

For package installer dry runs and full Docker-based install tests, see
[Install Script Testing](Install-Script-Testing.md).
