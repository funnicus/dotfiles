function dotsetup
    set installer "$HOME/.local/share/chezmoi/installer"

    if not test -d "$installer"
        echo "Could not find dotfiles installer at $installer"
        return 1
    end

    cargo run --manifest-path "$installer/Cargo.toml" -- $argv
end
