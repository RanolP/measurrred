#![cfg(target_os = "windows")]

use std::{collections::HashMap, ptr::null_mut};

use windows::Win32::{
    Foundation::PWSTR,
    System::Performance::{
        PdhAddEnglishCounterW, PdhCollectQueryData, PdhGetFormattedCounterValue, PdhOpenQueryW,
        PDH_CALC_NEGATIVE_DENOMINATOR, PDH_CSTATUS_INVALID_DATA, PDH_FMT_COUNTERVALUE,
        PDH_FMT_DOUBLE, PDH_FMT_LARGE, PDH_FMT_LONG, PDH_INVALID_DATA, PDH_NO_DATA,
    },
};

use crate::{
    data_source::DataSource,
    system::{Data, DataFormat, DataHandle},
};

pub struct PdhDataSource {
    query: isize,
    counter: HashMap<String, isize>,
}

impl PdhDataSource {
    pub fn new<'a>() -> eyre::Result<PdhDataSource> {
        let mut query = 0;

        let result = unsafe { PdhOpenQueryW(PWSTR(null_mut()), 0, &mut query) };
        if result != 0 {
            Err(windows::core::Error::from_win32())?;
        }

        Ok(PdhDataSource {
            query,
            counter: HashMap::new(),
        })
    }
}

impl DataSource for PdhDataSource {
    fn name(&self) -> &'static str {
        "windows/pdh"
    }

    fn update(&self) -> eyre::Result<()> {
        if self.counter.len() == 0 {
            return Ok(());
        }
        unsafe {
            let result = PdhCollectQueryData(self.query);
            if result != 0 {
                Err(::windows::core::Error::from_win32())?;
            }
        }

        Ok(())
    }

    fn query(&mut self, query: String, preferred_format: DataFormat) -> eyre::Result<DataHandle> {
        let counter = if let Some(&counter) = self.counter.get(&query) {
            counter
        } else {
            let mut counter = 0;
            let result =
                unsafe { PdhAddEnglishCounterW(self.query, query.clone(), 0, &mut counter) };

            if result != 0 {
                Err(::windows::core::Error::from_win32())?;
            }

            self.counter.insert(query, counter);
            counter
        };

        Ok(DataHandle(Box::new(move || {
            let mut value = PDH_FMT_COUNTERVALUE::default();
            let result = unsafe {
                PdhGetFormattedCounterValue(
                    counter,
                    match &preferred_format {
                        f @ DataFormat::String | f @ DataFormat::Bool => {
                            eyre::bail!("Unsupported format: {:?}", f)
                        }
                        DataFormat::I32 => PDH_FMT_LONG,
                        DataFormat::I64 | DataFormat::Int => PDH_FMT_LARGE,
                        DataFormat::F64 | DataFormat::Float => PDH_FMT_DOUBLE,
                    },
                    null_mut(),
                    &mut value,
                )
            };
            match result {
                0 => {}
                PDH_CALC_NEGATIVE_DENOMINATOR
                | PDH_INVALID_DATA
                | PDH_NO_DATA
                | PDH_CSTATUS_INVALID_DATA => {
                    return Ok(Data::Unknown);
                }
                _ => {
                    println!("{:x}", result);
                    Err(::windows::core::Error::from_win32())?;
                }
            }

            let data = unsafe {
                match &preferred_format {
                    f @ DataFormat::String | f @ DataFormat::Bool => {
                        eyre::bail!("Unsupported format: {:?}", f)
                    }
                    DataFormat::I32 => Data::I32(value.Anonymous.longValue as _),
                    DataFormat::I64 | DataFormat::Int => Data::I64(value.Anonymous.largeValue),
                    DataFormat::F64 | DataFormat::Float => Data::F64(value.Anonymous.doubleValue),
                }
            };

            Ok(data)
        })))
    }
}
