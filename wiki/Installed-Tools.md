# Installed Tools

The install script reads `installer/packages.toml` and installs a practical CLI
workstation: shell ergonomics, terminal workspace tools, git helpers, language
runtimes, diagnostics, and media/document utilities.

## Daily Terminal Workflow

- `fish`: default interactive shell.
  - `functions`: list loaded functions.
  - `funced name`: edit a fish function.
- `zellij`: terminal workspace/session manager.
  - `zellij`: start a session.
  - `zellij list-sessions`: show sessions.
  - `zellij attach <name>`: reconnect.
  - `Ctrl-p` then `n`/`d`/`r`: new pane, detach, rename.
- `zoxide`: smarter `cd`.
  - `z foo`: jump to a frequently used path matching `foo`.
  - `zi`: interactive jump.
- `fzf`: fuzzy finder used directly and by other tools.
  - `Ctrl-r`: fuzzy shell history search.
  - `find . -type f | fzf`: pick a file.
- `lsd`: nicer `ls`.
  - `ls -la`: detailed listing.
  - `ls --tree`: tree view.
- `yazi`: terminal file manager.
  - `yazi`: browse files.
  - `q`: quit, `Enter`: open, `Space`: select.

## Git And Code

- `git`, `git-lfs`: source control and large-file support.
  - `git status`, `git diff`, `git lfs pull`.
- `lazygit`: terminal Git UI.
  - `lazygit`: open UI in a repo.
  - Common flow: stage files, inspect diff, commit, push.
- `nvim`: editor.
  - `nvim .`: open a project.
  - `:Lazy`: plugin UI.
  - `:Mason`: language/tool installer UI.
- `ripgrep` (`rg`): fast text search.
  - `rg "text"`: search recursively.
  - `rg -n "text" path`: include line numbers.
- `fd`: fast file finder.
  - `fd name`: find paths by name.
  - `fd -e toml`: find by extension.
- `tree-sitter-cli`: parser tooling used by editor/dev workflows.
- `just`: command runner for this repo.
  - `just --list`: show recipes.
  - `just check`: local checks.
  - `just test-arch-ci`: run installer test container.
  - `just test-fedora-ci`: run Fedora installer test container.
- `bottom` (`btm`): terminal system monitor, installed through Cargo on Fedora.
- `actionlint`: validate GitHub Actions workflows.
  - `actionlint .github/workflows/install-script.yml`.

## Containers And Services

- `docker` / Docker Desktop: container runtime.
  - `docker ps`: running containers.
  - `docker compose up`: start a compose stack.
- `lazydocker`: terminal Docker UI.
  - `lazydocker`: inspect containers, logs, volumes.
- `tailscale`: private network/VPN.
  - `tailscale status`: current peers.
  - `tailscale up`: connect.

## System Inspection

- `btop` and `btm`: process/system monitors.
  - `btop`: rich interactive monitor.
  - `btm`: terminal dashboard.
- `gdu`: disk usage analyzer.
  - `gdu`: inspect current tree.
  - `gdu /path`: inspect a specific path.
- `fastfetch`: quick system summary.
- `topgrade`: update many package managers/tools.
  - `topgrade`: run updates.
- `thefuck`: shell command correction.
  - Run a failed command, then `fuck`.
- `tldr`: short practical command examples.
  - `tldr tar`, `tldr git-rebase`.

## Data, Documents, And Media

- `jq`: JSON processor.
  - `jq . file.json`: pretty-print JSON.
  - `curl ... | jq '.items[] | .name'`.
- `ffmpeg`: audio/video conversion.
  - `ffmpeg -i in.mov out.mp4`.
- `imagemagick` (`magick`): image conversion/editing.
  - `magick input.png output.webp`.
- `poppler`: PDF tools such as `pdftotext`.
  - `pdftotext file.pdf -`.
- `resvg`: render SVGs.
  - `resvg input.svg output.png`.
- `unzip`, `7zz`: archive extraction.
  - `unzip file.zip`.
  - `7zz x file.7z`.
- `file`: identify file types.

## Language And App Tooling

- `python`, `go`: language runtimes/toolchains.
- `node` via `nvm.fish`: Node LTS management.
  - `nvm install lts`.
  - `nvm use default`.
- `bun`: JavaScript runtime/package manager.
  - `bun install`, `bun run dev`.
- `pnpm`: Node package manager.
  - `pnpm install`, `pnpm dev`.
- `uv`: Python package/tool runner.
  - `uv tool install <tool>`.
  - `uvx <tool>`.
- `harlequin`: terminal SQL client.
  - `harlequin sqlite:///file.db`.
- `bw`: Bitwarden CLI used by private chezmoi templates.
  - `bw login`.
  - `export BW_SESSION="$(bw unlock --raw)"`.
  - `bw sync`.

## Small Fun Tools

- `figlet`: large ASCII text.
- `cowsay`: message bubbles.
- `fortune`: random quotes.

## Platform Notes

- Arch/CachyOS also installs `base-devel`, `wl-clipboard`, `xclip`, Linux
  Docker/Tailscale packages, and the JetBrains Mono Nerd Font package.
- macOS installs Homebrew formulae plus optional casks: Docker Desktop,
  Tailscale, and JetBrains Mono Nerd Font. Casks are skipped in CI.
