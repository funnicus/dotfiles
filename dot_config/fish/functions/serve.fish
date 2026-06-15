function serve --description "Start a simple HTTP server"
    set port 8000
    if test (count $argv) -gt 0
        set port $argv[1]
    end

    if command -v python3 >/dev/null
        python3 -m http.server $port
    else if command -v python >/dev/null
        python -m SimpleHTTPServer $port
    else
        echo "Python not found"
        return 1
    end
end
