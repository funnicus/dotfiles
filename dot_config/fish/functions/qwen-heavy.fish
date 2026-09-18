function qwen-heavy
    launchctl kill SIGTERM gui/(id -u)/local.mlx.qwen30b 2>/dev/null
    launchctl kickstart gui/(id -u)/local.mlx.coder-next
end
