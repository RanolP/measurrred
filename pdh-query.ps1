Get-Counter -ListSet * |
    Where-Object -Property CounterSetName -eq $args[0] |
    Select -expand Paths
