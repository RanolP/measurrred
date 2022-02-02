![banner](docs/images/banner.png)

## Features

- Monitor the System
  - [x] CPU Usage
  - [x] RAM Usage
  - [x] Network Up/Down

- Monitor these stuffs:
  - GPU usage
  - Power usage trend
  - Batteries remaining
  - Core temperature
  - Weather
  - Disk usage
  - Fan speed

## Motivation

As of Windows 11, Microsoft decided to eliminate the old DeskBand API, so the programs like [BatteryBar](https://batterybarpro.com/) (they decided to use floating window), and [NetSpeedMonitor](https://netspeedmonitor.net/) (I think they aren't maintaining because the latest version was published in 2019) have faced their end of life in DeskBand. By the way, there are some programs that didn't rely on the DeskBand API, like [TrafficMonitor](https://github.com/zhongyang219/TrafficMonitor/blob/master/README_en-us.md). But I need something extensible and customizable easily enough like [RainMeter](https://www.rainmeter.net/). So I made this, measurrred, a tiny taskbar integrated system monitor. Keep watch this evolving. Thanks!

### Features I Want to Implement

- Move center of taskbar a little
- Alter clock
- Remove network integrated sound icon or replace that with EarTrumpet's one
- Applying WinUI?
  - Especially Windows 11 Mica material

## Caveats

- Only aims to support Windows 11.
  - Especially target my local environment. Other environment wouldn't be tested.
