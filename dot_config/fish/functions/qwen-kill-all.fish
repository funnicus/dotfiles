function qwen-kill-all
    launchctl kill SIGTERM gui/(id -u)/local.mlx.coder-next 2>/dev/null
    launchctl kill SIGTERM gui/(id -u)/local.mlx.qwen30b 2>/dev/null
end
