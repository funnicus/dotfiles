function qwen-bootstrap
    for name in local.mlx.coder-next local.mlx.qwen30b
        launchctl bootout gui/(id -u)/$name
        launchctl bootstrap gui/(id -u) "$HOME/Library/LaunchAgents/$name.plist"
    end
end
