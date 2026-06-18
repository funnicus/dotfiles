function dotsetup
    set installer "$HOME/.local/share/chezmoi/installer"

    if not test -d "$installer"
        echo "Could not find dotfiles installer at $installer"
        return 1
    end

    switch (uname -s)
        case Darwin
            set os macos
        case Linux
            set os linux
        case '*'
            echo "Unsupported OS: "(uname -s)
            return 1
    end

    switch (uname -m)
        case arm64 aarch64
            set arch arm64
        case x86_64 amd64
            set arch x86_64
        case '*'
            echo "Unsupported architecture: "(uname -m)
            return 1
    end

    set binary "$installer/bin/dotsetup-$os-$arch"
    if not test -x "$binary"
        echo "Missing dotsetup binary: $binary"
        return 1
    end

    pushd "$installer" >/dev/null
    "$binary" $argv
    set status_code $status
    popd >/dev/null
    return $status_code
end
