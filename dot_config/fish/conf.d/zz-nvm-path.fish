# nvm.fish exports nvm_current_version, so nested shells (tmux, editor terminals)
# inherit it and skip activation while brew shellenv has moved brew's node first.
# Runs after conf.d/nvm.fish to put the active version back in front.
if set --query nvm_current_version
  fish_add_path --global --move --path $nvm_data/$nvm_current_version/bin
end
