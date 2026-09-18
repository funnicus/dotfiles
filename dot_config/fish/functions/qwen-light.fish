function qwen-light
    launchctl kill SIGTERM gui/(id -u)/local.mlx.coder-next 2>/dev/null
    launchctl kickstart gui/(id -u)/local.mlx.qwen30b
end
