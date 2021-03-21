use crate::error::IntDateError;
use crate::helpers::*;
use crate::IntDate;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

impl TryFrom<u32> for IntDate {
    type Error = IntDateError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if !is_day_valid(value) {
            return Err(IntDateError::InvalidDay);
        }

        if !is_month_valid(value) {
            return Err(IntDateError::InvalidMonth);
        }

        Ok(IntDate(value))
    }
}

impl FromStr for IntDate {
    type Err = IntDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut string = String::from(s);
        string.retain(|x| x.is_numeric());

        let value: u32 = match string.parse() {
            Ok(v) => v,
            Err(_) => return Err(IntDateError::ParsingFailed),
        };

        value.try_into()
    }
}

macro_rules! impl_tryfrom {
    ($($t:ty)*) => ($(
        impl TryFrom<$t> for IntDate {
            type Error = IntDateError;

            fn try_from(value: $t) -> Result<Self, Self::Error> {
                (value as u32).try_into()
            }
        }
    )*)
}

impl_tryfrom!(usize u8 u16 u64 u128 isize i8 i16 i32 i64 i128);
