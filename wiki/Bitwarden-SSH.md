# Bitwarden SSH Hosts

This repo is public, so private SSH host aliases, IPs, usernames, and ports are
not committed. Store private host blocks in Bitwarden and render them with
chezmoi.

## Bitwarden item

- Item name: `chezmoi-ssh-config`
- Item type: any type with a multiline notes/body field
- Notes/body: private `ssh_config` host blocks

Example notes/body value:

```sshconfig
Host example
	HostName example.com
	User alice
	Port 2222
	IdentityFile ~/.ssh/id_ed25519
```

## Chezmoi data

Set `ssh.bitwardenItem` during `chezmoi init`.

The item name can work after `bw sync`, but the item UUID is more reliable:

```bash
# Install first if needed:
# brew install bitwarden-cli
# sudo pacman -S bitwarden-cli

export BW_SESSION="$(bw unlock --raw)"
bw sync
bw list items --search chezmoi-ssh-config \
  | jq -r '.[] | "\(.id)\t\(.name)\tnotes_length=\((.notes // "") | length)"'
```

Use the printed UUID as `ssh.bitwardenItem` if name lookup is unreliable.

## Template

`private_dot_ssh/config.tmpl` renders the public default block and appends the
Bitwarden item notes:

```gotemplate
{{ output "bw" "get" "notes" $bitwardenItem }}
```

Verify rendering without applying:

```bash
# Install first if needed:
# brew install bitwarden-cli
# sudo pacman -S bitwarden-cli

export BW_SESSION="$(bw unlock --raw)"
bw sync
chezmoi execute-template < private_dot_ssh/config.tmpl
```
