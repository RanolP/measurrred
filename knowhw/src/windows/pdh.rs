use std::{collections::HashMap, ptr::null_mut};

use declarrred::rt::{Data, DataFormat};
use thiserror::Error;
use windows::{
    core::PCWSTR,
    Win32::System::Performance::{
        PdhAddEnglishCounterW, PdhCollectQueryData, PdhGetFormattedCounterValue, PdhOpenQueryW,
        PDH_CALC_NEGATIVE_DENOMINATOR, PDH_CALC_NEGATIVE_VALUE, PDH_CSTATUS_INVALID_DATA,
        PDH_FMT_COUNTERVALUE, PDH_FMT_DOUBLE, PDH_FMT_LARGE, PDH_FMT_LONG, PDH_INVALID_DATA,
        PDH_NO_DATA,
    },
};

use crate::Knowhw;

pub struct Pdh {
    query: isize,
    counter: HashMap<String, isize>,
}

#[derive(Debug, Error)]
pub enum PdhError {
    #[error("Unsupported Format: {0:?}")]
    UnsupportedFormat(DataFormat),

    #[error("Win32 error: {0}")]
    WindowsError(#[from] windows::core::Error),
}

impl Pdh {
    pub fn new<'a>() -> Result<Pdh, PdhError> {
        let mut query = 0;

        let result = unsafe { PdhOpenQueryW(PCWSTR(null_mut()), 0, &mut query) };
        if result != 0 {
            Err(windows::core::Error::from_win32())?;
        }

        Ok(Pdh {
            query,
            counter: HashMap::new(),
        })
    }
}

impl Knowhw for Pdh {
    type Error = PdhError;

    fn update(&self) -> Result<(), Self::Error> {
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

    fn query(&mut self, query: &str, preferred_format: &DataFormat) -> Result<Data, Self::Error> {
        let counter = if let Some(&counter) = self.counter.get(query) {
            counter
        } else {
            let mut counter = 0;
            let result =
                unsafe { PdhAddEnglishCounterW(self.query, query.clone(), 0, &mut counter) };

            if result != 0 {
                Err(::windows::core::Error::from_win32())?;
            }

            self.counter.insert(query.to_string(), counter);
            counter
        };

        let mut value = PDH_FMT_COUNTERVALUE::default();
        let result = unsafe {
            PdhGetFormattedCounterValue(
                counter,
                match &preferred_format {
                    f @ DataFormat::String | f @ DataFormat::Bool => {
                        return Err(PdhError::UnsupportedFormat(DataFormat::clone(f)))
                    }
                    DataFormat::I32 | DataFormat::U32 => PDH_FMT_LONG,
                    DataFormat::I64 | DataFormat::Int | DataFormat::U64 | DataFormat::UInt => {
                        PDH_FMT_LARGE
                    }
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
            | PDH_CSTATUS_INVALID_DATA
            | PDH_CALC_NEGATIVE_VALUE => {
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
                    return Err(PdhError::UnsupportedFormat(DataFormat::clone(f)))
                }
                DataFormat::I32 => Data::I32(value.Anonymous.longValue),
                DataFormat::U32 => Data::U32(value.Anonymous.longValue as _),
                DataFormat::I64 | DataFormat::Int => Data::I64(value.Anonymous.largeValue),
                DataFormat::U64 | DataFormat::UInt => Data::U64(value.Anonymous.largeValue as _),
                DataFormat::F64 | DataFormat::Float => Data::F64(value.Anonymous.doubleValue),
            }
        };

        Ok(data)
    }
}
