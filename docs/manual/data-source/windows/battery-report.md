# `windows/battery-report` Data Source

<small>`#[target_os = "windows"]`</small>

`windows/battery-report` data source fetches data using following article: [Get battery information](https://docs.microsoft.com/en-us/windows/uwp/devices-sensors/get-battery-info).

Be careful that the properties except Status-related property are considered `Unknown` when the battery controller isn't present.

It accepts a query matched with the properties of [`BatteryReport`](https://docs.microsoft.com/en-us/uwp/api/Windows.Devices.Power.BatteryReport) struct respectively. And a bit more fields for convenience. See following table for more information.

| Query                                                         | Description                                                                                                                                                                          |
| ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `ChargeRateInMilliwatts`                                      | Gets the rate that the battery is charging.                                                                                                                                          |
| `DesignCapacityInMilliwattHours`                              | Gets the estimated energy capacity of a new battery of this type.                                                                                                                    |
| `FullChargeCapacityInMilliwattHours`                          | Gets the fully-charged energy capacity of the battery.                                                                                                                               |
| `RemainingCapacityInMilliwattHours`                           | Gets the remaining power capacity of the battery.                                                                                                                                    |
| `RemainingCapacityInPercentage`<sup>measurrred specific</sup> | Gets the remaining power capacity of the battery as percentage.                                                                                                                      |
| `Status`                                                      | Gets a [BatteryStatus](https://docs.microsoft.com/en-us/uwp/api/windows.system.power.batterystatus) enumeration that indicates the status of the battery. Will be treated as number. |
| `StatusIsPresent`<sup>measurrred specific</sup>               | Whether the `Status` is not `NotPresent` (`0`).                                                                                                                                      |
| `StatusIsDischarging`<sup>measurrred specific</sup>           | Whether the `Status` is `Discharging` (`1`).                                                                                                                                         |
| `StatusIsIdle`<sup>measurrred specific</sup>                  | Whether the `Status` is `Idle` (`2`).                                                                                                                                                |
| `StatusIsCharging`<sup>measurrred specific</sup>              | Whether the `Status` is `Charging` (`3`).                                                                                                                                            |
