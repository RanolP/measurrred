# `windows/pdh` Data Source

<small>`#[target_os = "windows"]`</small>

`windows/pdh`(Performance Data Helper) data source queries **Performance Counters**, that provide a high-level abstraction layer for collecting various system data like CPU, memory, disk, etc.

There are various real-world application utilizing these data. For instance, Microsoft include Performance Monitor (`perfmon.exe`) and Resource Monitor (`resmon.exe`) utilize that.

## See Also

- [Powershell Command `Get-Counter`](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.diagnostics/get-counter)
- [Network-Related Performance Counters](https://docs.microsoft.com/en-us/windows-server/networking/technologies/network-subsystem/net-sub-performance-counters)