function devcontainer-init --description 'Copy a devcontainer starter into the current project'
    if test (count $argv) -ne 1
        echo 'Usage: devcontainer-init node|azure-node|godot' >&2
        return 2
    end
    switch $argv[1]
        case node azure-node godot
        case '*'
            echo 'Unknown template. Choose node, azure-node, or godot.' >&2
            return 2
    end
    set -l config_root "$HOME/.config"
    if set -q XDG_CONFIG_HOME; and test -n "$XDG_CONFIG_HOME"
        set config_root "$XDG_CONFIG_HOME"
    end
    set -l source_dir "$config_root/devcontainer-templates/$argv[1]"
    if not test -d "$source_dir"
        echo "Template directory missing: $source_dir" >&2
        return 1
    end
    if test -e .devcontainer; or test -L .devcontainer
        echo '.devcontainer already exists; no files were copied.' >&2
        return 1
    end
    mkdir .devcontainer; or return 1
    cp -R "$source_dir/." .devcontainer/
    set -l result $status
    if test $result -ne 0
        echo 'Copy failed; inspect .devcontainer before retrying.' >&2
        return $result
    end
    echo "Created .devcontainer from $argv[1]. Open this project in Zed."
end
