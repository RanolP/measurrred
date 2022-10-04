use std::str::FromStr;

use declarrred::rt::{Data, DataFormat};
use strum::EnumString;
use thiserror::Error;
use windows::{Devices::Power::Battery, System::Power::BatteryStatus};

use crate::Knowhw;

pub struct BatteryReport;

#[derive(EnumString)]
pub enum BatteryReportQuery {
    ChargeRateInMilliwatts,
    DesignCapacityInMilliwattHours,
    FullChargeCapacityInMilliwattHours,
    RemainingCapacityInMilliwattHours,
    RemainingCapacityInPercentage,
    Status,
    StatusIsPresent,
    StatusIsDischarging,
    StatusIsIdle,
    StatusIsCharging,
}

#[derive(Debug, Error)]
pub enum BatteryReportError {
    #[error("Failed to parse query: {0}")]
    InvalidQuery(#[from] strum::ParseError),

    #[error("Win32 error: {0}")]
    WindowsError(#[from] windows::core::Error),
}

impl Knowhw for BatteryReport {
    type Error = BatteryReportError;

    fn query(&mut self, query: &str, _preferred_format: &DataFormat) -> Result<Data, Self::Error> {
        let query = BatteryReportQuery::from_str(query)?;

        let battery = Battery::AggregateBattery()?;
        let report = battery.GetReport()?;

        let is_present = !matches!(report.Status(), Err(_) | Ok(BatteryStatus(0)));

        let result = match query {
            BatteryReportQuery::ChargeRateInMilliwatts => {
                if is_present {
                    Data::I32(report.ChargeRateInMilliwatts()?.GetInt32()?)
                } else {
                    Data::Unknown
                }
            }
            BatteryReportQuery::DesignCapacityInMilliwattHours => {
                if is_present {
                    Data::I32(report.DesignCapacityInMilliwattHours()?.GetInt32()?)
                } else {
                    Data::Unknown
                }
            }
            BatteryReportQuery::FullChargeCapacityInMilliwattHours => {
                if is_present {
                    Data::I32(report.FullChargeCapacityInMilliwattHours()?.GetInt32()?)
                } else {
                    Data::Unknown
                }
            }
            BatteryReportQuery::RemainingCapacityInMilliwattHours => {
                if is_present {
                    Data::I32(report.RemainingCapacityInMilliwattHours()?.GetInt32()?)
                } else {
                    Data::Unknown
                }
            }
            BatteryReportQuery::RemainingCapacityInPercentage => {
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
            BatteryReportQuery::Status => Data::I32(report.Status()?.0),
            BatteryReportQuery::StatusIsPresent => {
                Data::Bool(report.Status()? != BatteryStatus::NotPresent)
            }
            BatteryReportQuery::StatusIsDischarging => {
                Data::Bool(report.Status()? == BatteryStatus::Discharging)
            }
            BatteryReportQuery::StatusIsIdle => Data::Bool(report.Status()? == BatteryStatus::Idle),
            BatteryReportQuery::StatusIsCharging => {
                Data::Bool(report.Status()? == BatteryStatus::Charging)
            }
        };

        Ok(result)
    }
}
