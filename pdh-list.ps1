Get-Counter -ListSet * |
    Sort-Object -Property CounterSetName |
    Format-Table CounterSetName, Paths
