# Local AI

## MLX

MLX is an array framework for efficient and flexible machine learning on Apple silicon. Basically, it allows us to use LLMs more efficiently on Apple silicon.

You should have a few useful fish functions to get started:

```bash
qwen-bootstrap # Load changed plist files to launchagent
qwen-light # Start lighter 30B Qwen Coder model
qwen-heavy # Start heavier Next Coder model
qwen-kill-all # Stop all models (they consume a lot of memory)
```

To debug problems:

```bash
curl http://127.0.0.1:8080/v1/models # Show loaded models
launchctl print gui/(id -u)/local.mlx.coder-next  # Print info about the coder next, see plist files for the names of the other models
cat /tmp/mlx-coder-next-error.log # See error logs
launchctl kickstart gui/$(id -u)/local.mlx.coder-next # Manualy start a model
```
