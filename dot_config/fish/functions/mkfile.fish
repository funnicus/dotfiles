function mkfile --description "Create a file and all parent directories"
    if test (count $argv) -ne 1
        echo "Usage: mkfile <path>"
        return 1
    end

    set file_path $argv[1]
    set dir_path (dirname "$file_path")

    if test "$dir_path" != "."
        mkdir -p "$dir_path"
    end

    touch "$file_path"
end
