function sysinfo --description "Display system information"
    echo "System Information:"
    echo "=================="
    echo "Hostname: "(hostname)
    echo "User: "$USER
    echo "OS: "(uname -s)
    echo "Kernel: "(uname -r)
    echo "Uptime: "(uptime | cut -d',' -f1 | cut -d' ' -f4-)
    echo "Memory: "(free -h | grep '^Mem:' | awk '{print $3"/"$2}')
    echo "Disk Usage: "(df -h / | tail -1 | awk '{print $5" used"}')
end
