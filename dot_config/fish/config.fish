if test -f /usr/share/cachyos-fish-config/cachyos-config.fish
  source /usr/share/cachyos-fish-config/cachyos-config.fish
end

if command -q zoxide
  zoxide init fish | source
end
# overwrite greeting
# potentially disabling fastfetch
#function fish_greeting
#    # smth smth
#end

if command -q thefuck
  thefuck --alias | source
end

if set -q XDG_RUNTIME_DIR
  set -Ux SSH_AUTH_SOCK $XDG_RUNTIME_DIR/ssh-agent.socket
end

# bun
set --export BUN_INSTALL "$HOME/.bun"
if not string match -q -- $BUN_INSTALL/bin $PATH
  set --export PATH $BUN_INSTALL/bin $PATH
end

# pnpm
set -gx PNPM_HOME "$HOME/.local/share/pnpm"
if not string match -q -- $PNPM_HOME $PATH
  set -gx PATH $PNPM_HOME $PATH
end
# pnpm end

set -gx EDITOR zed
set -gx VISUAL zed
