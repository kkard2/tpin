# tpin
goofy ahh alias thingy

# usage
i don't know why you want to use this, but here you go
(yes this is `tpin help` output):
```
new, n     Create new/replace existing alias
delete, d  Delete alias
list, l    List all aliases
print, p   Print alias to stdout
run, r     Run alias
help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

# the script
i want to put it somewhere so it's easy to copy:
```powershell
$result = & "tpin.exe" "p" $args[0] | Out-String

if ($LastExitCode -eq 0) {
    Set-Location -Path $result.Trim()
}
```

# the script but i'm on linux now
```bash
#!/bin/bash

# Check if the number of arguments is correct
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <argument>"
    exit 1
fi

# Get the directory using tpin
directory=$(tpin p "$1")

# Check if tpin returns 0
if [ $? -eq 0 ]; then
    # Change to the specified directory
    cd "$directory"
else
    exit 1
fi

# generously provided by chatgpt
```

# also
also it requires nightly so that's fun
