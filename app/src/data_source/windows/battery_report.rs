use windows::{Devices::Power::Battery, System::Power::BatteryStatus};

use crate::{
    data_source::DataSource,
    system::{Data, DataFormat},
};

pub struct BatteryReportDataSource;

impl DataSource for BatteryReportDataSource {
    fn name(&self) -> &'static str {
        "windows/battery-report"
    }

    fn update(&self) -> eyre::Result<()> {
        Ok(())
    }

    fn query(&mut self, query: &str, _preferred_format: &DataFormat) -> eyre::Result<Data> {
        let battery = Battery::AggregateBattery()?;
        let report = battery.GetReport()?;

        let is_present = !matches!(report.Status(), Err(_) | Ok(BatteryStatus(0)));

        let result = match query {
            "ChargeRateInMilliwatts" => {
                if is_present {
                    Data::I32(report.ChargeRateInMilliwatts()?.GetInt32()?)
                } else {
                    Data::Unknown
                }
            }
            "DesignCapacityInMilliwattHours" => {
                if is_present {
                    Data::I32(report.DesignCapacityInMilliwattHours()?.GetInt32()?)
                } else {
                    Data::Unknown
                }
            }
            "FullChargeCapacityInMilliwattHours" => {
                if is_present {
                    Data::I32(report.FullChargeCapacityInMilliwattHours()?.GetInt32()?)
                } else {
                    Data::Unknown
                }
            }
            "RemainingCapacityInMilliwattHours" => {
                if is_present {
                    Data::I32(report.RemainingCapacityInMilliwattHours()?.GetInt32()?)
                } else {
                    Data::Unknown
                }
            }
            "RemainingCapacityInPercentage" => {
                if is_present {
                    Data::F64(
                        report.RemainingCapacityInMilliwattHours()?.GetInt32()? as f64
                            / report.FullChargeCapacityInMilliwattHours()?.GetInt32()? as f64
                            * 100.0,
                    )
                } else {
                    Data::Unknown
                }
            }
            "Status" => Data::I32(report.Status()?.0),
            "StatusIsPresent" => Data::Bool(report.Status()? != BatteryStatus::NotPresent),
            "StatusIsDischarging" => Data::Bool(report.Status()? == BatteryStatus::Discharging),
            "StatusIsIdle" => Data::Bool(report.Status()? == BatteryStatus::Idle),
            "StatusIsCharging" => Data::Bool(report.Status()? == BatteryStatus::Charging),
            _ => eyre::bail!("Unknown query: {}", query),
        };

        Ok(result)
    }
}
