# `global-memory-status` Data Source

<small>`#[target_os = "windows"]`</small>

`global-memory-status` data source fetches data using [`GlobalMemoryStatusEx`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-globalmemorystatusex) Function.

It accepts a query that respectfully matching with the field of [`MEMORYSTATUSEX`](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-memorystatusex) struct. And a bit more fields for convenience. See following table for more information.

| Query                                       | Description                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `dwMemoryLoad`                              | A number between 0 and 100 that specifies the approximate percentage of physical memory that is in use (0 indicates no memory use and 100 indicates full memory use).                                                                                                                                                                                                                                                                      |
| `ullTotalPhys`                              | The amount of actual physical memory, in bytes.                                                                                                                                                                                                                                                                                                                                                                                            |
| `ullAvailPhys`                              | The amount of physical memory currently available, in bytes. This is the amount of physical memory that can be immediately reused without having to write its contents to disk first. It is the sum of the size of the standby, free, and zero lists.                                                                                                                                                                                      |
| `ullUsedPhys`<sup>measurrred specific</sup> | The amount of physical memory currently used, in bytes.                                                                                                                                                                                                                                                                                                                                                                                    |
| `ullTotalPageFile`                          | The current committed memory limit for the system or the current process, whichever is smaller, in bytes. To get the system-wide committed memory limit, call GetPerformanceInfo.                                                                                                                                                                                                                                                          |
| `ullAvailPageFile`                          | The maximum amount of memory the current process can commit, in bytes. This value is equal to or smaller than the system-wide available commit value. To calculate the system-wide available commit value, call GetPerformanceInfo and subtract the value of CommitTotal from the value of CommitLimit.                                                                                                                                    |
| `ullTotalVirtual`                           | The size of the user-mode portion of the virtual address space of the calling process, in bytes. This value depends on the type of process, the type of processor, and the configuration of the operating system. For example, this value is approximately 2 GB for most 32-bit processes on an x86 processor and approximately 3 GB for 32-bit processes that are large address aware running on a system with 4-gigabyte tuning enabled. |
| `ullAvailVirtual`                           | The amount of unreserved and uncommitted memory currently in the user-mode portion of the virtual address space of the calling process, in bytes.                                                                                                                                                                                                                                                                                          |
| `ullAvailExtendedVirtual`                   | Reserved. This value is always 0.                                                                                                                                                                                                                                                                                                                                                                                                          |
| `dMemoryLoad`<sup>measurrred specific</sup> | Same as `dwMemoryLoad` but more precise floating point value.                                                                                                                                                                                                                                                                                                                                                                              |