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

# also
also it requires nightly so that's fun
